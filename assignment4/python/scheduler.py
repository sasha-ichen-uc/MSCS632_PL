"""Employee shift scheduler (Assignment 4 - Python implementation).

Demonstrates conditionals, loops, and branching while solving a constrained
scheduling problem:
  - no employee works more than one shift per day
  - no employee works more than 5 days per week
  - every shift on every day needs at least MIN_PER_SHIFT employees
  - shifts cap out at MAX_PER_SHIFT employees, which is what creates the
    "preferred shift is full" conflicts the assignment asks us to resolve

Bonus: employees rank the three shifts in priority order per day (1st
choice, 2nd choice, 3rd choice) instead of naming a single preferred shift.
The scheduler tries each employee's choices in rank order and falls back
only when every choice is unavailable.
"""

import random
from dataclasses import dataclass, field

DAYS = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]
SHIFTS = ["Morning", "Afternoon", "Evening"]

MIN_PER_SHIFT = 2
MAX_PER_SHIFT = 4
MAX_DAYS_PER_WEEK = 5

RANDOM_SEED = 42

EMPLOYEE_NAMES = [
    "Alice", "Bob", "Carol", "Dave", "Eve", "Frank", "Grace",
    "Heidi", "Ivan", "Judy", "Mallory", "Niaj", "Olivia", "Peggy",
]


@dataclass
class Employee:
    name: str
    preferences: dict  # day -> ranked list of shifts, best choice first
    days_worked: int = 0
    assigned_days: set = field(default_factory=set)

    def is_available(self, day):
        return day not in self.assigned_days and self.days_worked < MAX_DAYS_PER_WEEK


class Stats:
    """Tracks how well we honored everyone's ranked preferences, so the
    bonus's "accommodate preferences as much as possible" claim is backed
    by a number instead of just an assertion."""

    def __init__(self):
        self.by_rank = [0, 0, 0]  # index 0 = 1st choice, 1 = 2nd, 2 = 3rd
        self.random_backfill = 0
        self.unplaced = 0

    def record_rank(self, rank):
        self.by_rank[rank] += 1

    def record_backfill(self):
        self.random_backfill += 1

    def record_unplaced(self):
        self.unplaced += 1


def build_employees(rng):
    """Collect employee names and a ranked shift preference per day.

    In a real UI this would come from user input; here we generate it so the
    scheduler can be run end-to-end and produce a reproducible screenshot.
    """
    employees = []
    for name in EMPLOYEE_NAMES:
        prefs = {}
        for day in DAYS:
            ranked = list(SHIFTS)
            rng.shuffle(ranked)
            prefs[day] = ranked
        employees.append(Employee(name=name, preferences=prefs))
    return employees


def assign(schedule, employee, day, shift):
    schedule[day][shift].append(employee.name)
    employee.assigned_days.add(day)
    employee.days_worked += 1


class DayBudget:
    """Tracks how many above-minimum ("discretionary") assignments we're
    still willing to make today without jeopardizing future days' minimum
    staffing. Shared by every placement attempt today so nothing can
    silently overspend what the reservation below accounted for."""

    def __init__(self, extra_budget):
        self.extra_budget = extra_budget
        self.extra_used = 0

    def has_room(self):
        return self.extra_used < self.extra_budget

    def spend(self):
        self.extra_used += 1


def try_place_today(schedule, employee, day, budget, stats):
    """Walk the employee's ranked choices for today in priority order and
    take the first one with room. This is also how same-day shift conflicts
    get resolved: a full 1st choice just falls through to the 2nd choice,
    etc., instead of needing separate conflict-handling code."""

    for rank, shift in enumerate(employee.preferences[day]):
        count = len(schedule[day][shift])
        if count < MIN_PER_SHIFT:
            # Filling the shift's minimum is always allowed; it was already
            # accounted for in the day's reservation.
            assign(schedule, employee, day, shift)
            stats.record_rank(rank)
            return True
        if count < MAX_PER_SHIFT and budget.has_room():
            assign(schedule, employee, day, shift)
            budget.spend()
            stats.record_rank(rank)
            return True

    return False


def carry_over_to_next_day(schedule, employee, day, stats):
    """Every choice was full today. Try the employee's ranked choices again
    on the next day. Tomorrow's own budget pass hasn't run yet, so this can
    only claim a spot within tomorrow's guaranteed minimum layer; anything
    discretionary is left for tomorrow's own pass once its budget exists."""

    day_index = DAYS.index(day)
    if day_index + 1 >= len(DAYS):
        return False

    next_day = DAYS[day_index + 1]
    if not employee.is_available(next_day):
        return False

    for rank, shift in enumerate(employee.preferences[next_day]):
        if len(schedule[next_day][shift]) < MIN_PER_SHIFT:
            assign(schedule, employee, next_day, shift)
            stats.record_rank(rank)
            return True

    return False


def fill_minimum_staffing(schedule, employees, day, rng, stats):
    """Top a shift up to MIN_PER_SHIFT with random eligible employees if not
    enough people's preferences landed on it. This is a hard requirement, so
    it always runs regardless of the day's discretionary budget."""

    for shift in SHIFTS:
        while len(schedule[day][shift]) < MIN_PER_SHIFT:
            pool = [e for e in employees if e.is_available(day)]
            if not pool:
                break
            chosen = rng.choice(pool)
            assign(schedule, chosen, day, shift)
            stats.record_backfill()


def remaining_capacity(employees):
    """Total employee-days still available this week across everyone."""
    return sum(MAX_DAYS_PER_WEEK - e.days_worked for e in employees)


def build_schedule(employees, rng):
    schedule = {day: {shift: [] for shift in SHIFTS} for day in DAYS}
    slots_per_day = MIN_PER_SHIFT * len(SHIFTS)
    stats = Stats()

    for day_index, day in enumerate(DAYS):
        # Every day still to come (including today) needs at least
        # slots_per_day employee-days of capacity for its own minimum
        # staffing. Whatever capacity is left over after reserving for
        # every future day is "discretionary" budget we're willing to
        # spend today letting people work beyond a shift's minimum just
        # because they prefer it. Without this reservation, greedily
        # honoring preferences on the early days of the week exhausts
        # everyone's 5-day cap and leaves the weekend without enough
        # staff left to even hit the minimum, let alone preferences.
        days_remaining_after_today = len(DAYS) - day_index - 1
        reserve_for_future = days_remaining_after_today * slots_per_day
        extra_budget = max(0, remaining_capacity(employees) - reserve_for_future - slots_per_day)
        budget = DayBudget(extra_budget)

        candidates = [e for e in employees if e.is_available(day)]
        # Break ties randomly, but give priority to employees who have
        # worked fewer days so far, to spread shifts evenly across staff.
        rng.shuffle(candidates)
        candidates.sort(key=lambda e: e.days_worked)

        for employee in candidates:
            if not employee.is_available(day):
                continue
            if try_place_today(schedule, employee, day, budget, stats):
                continue
            if not carry_over_to_next_day(schedule, employee, day, stats):
                stats.record_unplaced()

        # Any shift that still didn't reach its minimum (nobody's ranked
        # choices landed on it) gets topped up at random from whoever is
        # left.
        fill_minimum_staffing(schedule, employees, day, rng, stats)

    return schedule, stats


def print_schedule(schedule, employees, stats):
    print("=" * 50)
    print("WEEKLY EMPLOYEE SCHEDULE")
    print("=" * 50)

    for day in DAYS:
        print(f"\n{day}:")
        for shift in SHIFTS:
            names = schedule[day][shift]
            if names:
                print(f"  {shift:10s}: {', '.join(names)}")
            else:
                print(f"  {shift:10s}: (unstaffed)")

    print("\n" + "-" * 50)
    print("Days worked per employee:")
    for employee in employees:
        print(f"  {employee.name:10s}: {employee.days_worked} day(s)")

    print("\n" + "-" * 50)
    print("Preference satisfaction:")
    labels = ["1st choice", "2nd choice", "3rd choice"]
    for rank, label in enumerate(labels):
        print(f"  {label:12s}: {stats.by_rank[rank]}")
    print(f"  {'Random backfill':12s}: {stats.random_backfill} (not preference-driven)")
    print(f"  {'Unplaced':12s}: {stats.unplaced} (no shift had room same or next day)")


def main():
    rng = random.Random(RANDOM_SEED)
    employees = build_employees(rng)
    schedule, stats = build_schedule(employees, rng)
    print_schedule(schedule, employees, stats)


if __name__ == "__main__":
    main()
