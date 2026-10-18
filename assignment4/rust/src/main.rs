// Employee shift scheduler (Assignment 4 - Rust implementation).
//
// Same scheduling rules and algorithm as the Python version in
// ../python/scheduler.py, ported to Rust's control-flow constructs
// (match, while, for, if/else-if chains) and ownership model:
//   - no employee works more than one shift per day
//   - no employee works more than 5 days per week
//   - every shift on every day needs at least MIN_PER_SHIFT employees
//   - shifts cap out at MAX_PER_SHIFT employees, which is what creates the
//     "preferred shift is full" conflicts the assignment asks us to resolve
//
// Bonus: employees rank the three shifts in priority order per day (1st
// choice, 2nd choice, 3rd choice) instead of naming a single preferred
// shift. The scheduler tries each employee's choices in rank order and
// falls back only when every choice is unavailable.
//
// No external crates are used (including for randomness) so the project
// builds offline with just `cargo build`.

use std::collections::HashSet;

const DAYS: [&str; 7] = [
    "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday",
];
const SHIFTS: [&str; 3] = ["Morning", "Afternoon", "Evening"];

const MIN_PER_SHIFT: usize = 2;
const MAX_PER_SHIFT: usize = 4;
const MAX_DAYS_PER_WEEK: u32 = 5;

const RANDOM_SEED: u64 = 42;

const EMPLOYEE_NAMES: [&str; 14] = [
    "Alice", "Bob", "Carol", "Dave", "Eve", "Frank", "Grace", "Heidi", "Ivan", "Judy",
    "Mallory", "Niaj", "Olivia", "Peggy",
];

/// Minimal xorshift64* PRNG so the project has no external dependencies.
struct Rng {
    state: u64,
}

impl Rng {
    fn new(seed: u64) -> Self {
        // xorshift requires a non-zero seed.
        Rng { state: if seed == 0 { 0xdead_beef } else { seed } }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Random integer in [0, n).
    fn gen_range(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }

    fn choice<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.gen_range(items.len())]
    }

    fn shuffle<T>(&mut self, items: &mut [T]) {
        let len = items.len();
        if len < 2 {
            return;
        }
        for i in (1..len).rev() {
            let j = self.gen_range(i + 1);
            items.swap(i, j);
        }
    }
}

struct Employee {
    name: String,
    // preferences[day] = the three shift indices in priority order,
    // best choice first.
    preferences: [[usize; 3]; 7],
    days_worked: u32,
    assigned_days: HashSet<usize>,
}

impl Employee {
    fn is_available(&self, day: usize) -> bool {
        !self.assigned_days.contains(&day) && self.days_worked < MAX_DAYS_PER_WEEK
    }
}

/// schedule[day][shift] = employee indices assigned to that shift.
type Schedule = Vec<[Vec<usize>; 3]>;

/// Tracks how well we honored everyone's ranked preferences, so the
/// bonus's "accommodate preferences as much as possible" claim is backed
/// by a number instead of just an assertion.
#[derive(Default)]
struct Stats {
    by_rank: [u32; 3], // index 0 = 1st choice, 1 = 2nd, 2 = 3rd
    random_backfill: u32,
    unplaced: u32,
}

fn build_employees(rng: &mut Rng) -> Vec<Employee> {
    EMPLOYEE_NAMES
        .iter()
        .map(|&name| {
            let mut preferences = [[0usize, 1, 2]; 7];
            for day in 0..DAYS.len() {
                let mut ranked = [0usize, 1, 2];
                rng.shuffle(&mut ranked);
                preferences[day] = ranked;
            }
            Employee {
                name: name.to_string(),
                preferences,
                days_worked: 0,
                assigned_days: HashSet::new(),
            }
        })
        .collect()
}

fn assign(schedule: &mut Schedule, employees: &mut [Employee], emp_idx: usize, day: usize, shift: usize) {
    schedule[day][shift].push(emp_idx);
    employees[emp_idx].assigned_days.insert(day);
    employees[emp_idx].days_worked += 1;
}

/// Tracks how many above-minimum ("discretionary") assignments we're still
/// willing to make today without jeopardizing future days' minimum
/// staffing. Shared by every placement attempt today so nothing can
/// silently overspend what the reservation below accounted for.
struct DayBudget {
    extra_budget: i64,
    extra_used: i64,
}

impl DayBudget {
    fn has_room(&self) -> bool {
        self.extra_used < self.extra_budget
    }

    fn spend(&mut self) {
        self.extra_used += 1;
    }
}

/// Walk the employee's ranked choices for today in priority order and take
/// the first one with room. This is also how same-day shift conflicts get
/// resolved: a full 1st choice just falls through to the 2nd choice, etc.,
/// instead of needing separate conflict-handling code.
fn try_place_today(
    schedule: &mut Schedule,
    employees: &mut [Employee],
    emp_idx: usize,
    day: usize,
    budget: &mut DayBudget,
    stats: &mut Stats,
) -> bool {
    let ranked = employees[emp_idx].preferences[day];
    for (rank, &shift) in ranked.iter().enumerate() {
        let count = schedule[day][shift].len();
        if count < MIN_PER_SHIFT {
            assign(schedule, employees, emp_idx, day, shift);
            stats.by_rank[rank] += 1;
            return true;
        }
        if count < MAX_PER_SHIFT && budget.has_room() {
            assign(schedule, employees, emp_idx, day, shift);
            budget.spend();
            stats.by_rank[rank] += 1;
            return true;
        }
    }
    false
}

/// Every choice was full today. Try the employee's ranked choices again on
/// the next day. Tomorrow's own budget pass hasn't run yet, so this can
/// only claim a spot within tomorrow's guaranteed minimum layer; anything
/// discretionary is left for tomorrow's own pass once its budget exists.
fn carry_over_to_next_day(
    schedule: &mut Schedule,
    employees: &mut [Employee],
    emp_idx: usize,
    day: usize,
    stats: &mut Stats,
) -> bool {
    if day + 1 >= DAYS.len() {
        return false;
    }
    let next_day = day + 1;
    if !employees[emp_idx].is_available(next_day) {
        return false;
    }

    let ranked = employees[emp_idx].preferences[next_day];
    for (rank, &shift) in ranked.iter().enumerate() {
        if schedule[next_day][shift].len() < MIN_PER_SHIFT {
            assign(schedule, employees, emp_idx, next_day, shift);
            stats.by_rank[rank] += 1;
            return true;
        }
    }

    false
}

/// Top a shift up to MIN_PER_SHIFT with random eligible employees if not
/// enough people's preferences landed on it. This is a hard requirement, so
/// it always runs regardless of the day's discretionary budget.
fn fill_minimum_staffing(
    schedule: &mut Schedule,
    employees: &mut [Employee],
    day: usize,
    rng: &mut Rng,
    stats: &mut Stats,
) {
    for shift in 0..SHIFTS.len() {
        while schedule[day][shift].len() < MIN_PER_SHIFT {
            let pool: Vec<usize> = (0..employees.len())
                .filter(|&idx| employees[idx].is_available(day))
                .collect();
            if pool.is_empty() {
                break;
            }
            let chosen = *rng.choice(&pool);
            assign(schedule, employees, chosen, day, shift);
            stats.random_backfill += 1;
        }
    }
}

/// Total employee-days still available this week across everyone.
fn remaining_capacity(employees: &[Employee]) -> i64 {
    employees
        .iter()
        .map(|e| (MAX_DAYS_PER_WEEK - e.days_worked) as i64)
        .sum()
}

fn build_schedule(employees: &mut Vec<Employee>, rng: &mut Rng) -> (Schedule, Stats) {
    let mut schedule: Schedule = (0..DAYS.len()).map(|_| [Vec::new(), Vec::new(), Vec::new()]).collect();
    let slots_per_day = (MIN_PER_SHIFT * SHIFTS.len()) as i64;
    let mut stats = Stats::default();

    for day in 0..DAYS.len() {
        // Every day still to come (including today) needs at least
        // slots_per_day employee-days of capacity for its own minimum
        // staffing. Whatever capacity is left over after reserving for
        // every future day is "discretionary" budget we're willing to
        // spend today letting people work beyond a shift's minimum just
        // because they prefer it. Without this reservation, greedily
        // honoring preferences on the early days of the week exhausts
        // everyone's 5-day cap and leaves the weekend without enough
        // staff left to even hit the minimum, let alone preferences.
        let days_remaining_after_today = (DAYS.len() - day - 1) as i64;
        let reserve_for_future = days_remaining_after_today * slots_per_day;
        let extra_budget = (remaining_capacity(employees) - reserve_for_future - slots_per_day).max(0);
        let mut budget = DayBudget { extra_budget, extra_used: 0 };

        let mut candidates: Vec<usize> = (0..employees.len())
            .filter(|&idx| employees[idx].is_available(day))
            .collect();
        // Break ties randomly, but give priority to employees who have
        // worked fewer days so far, to spread shifts evenly across staff.
        rng.shuffle(&mut candidates);
        candidates.sort_by_key(|&idx| employees[idx].days_worked);

        for emp_idx in candidates {
            if !employees[emp_idx].is_available(day) {
                continue;
            }
            if try_place_today(&mut schedule, employees, emp_idx, day, &mut budget, &mut stats) {
                continue;
            }
            if !carry_over_to_next_day(&mut schedule, employees, emp_idx, day, &mut stats) {
                stats.unplaced += 1;
            }
        }

        // Any shift that still didn't reach its minimum (nobody's ranked
        // choices landed on it) gets topped up at random from whoever is
        // left.
        fill_minimum_staffing(&mut schedule, employees, day, rng, &mut stats);
    }

    (schedule, stats)
}

fn print_schedule(schedule: &Schedule, employees: &[Employee], stats: &Stats) {
    println!("{}", "=".repeat(50));
    println!("WEEKLY EMPLOYEE SCHEDULE");
    println!("{}", "=".repeat(50));

    for (day_idx, day_name) in DAYS.iter().enumerate() {
        println!("\n{}:", day_name);
        for (shift_idx, shift_name) in SHIFTS.iter().enumerate() {
            let names: Vec<&str> = schedule[day_idx][shift_idx]
                .iter()
                .map(|&idx| employees[idx].name.as_str())
                .collect();
            if names.is_empty() {
                println!("  {:10}: (unstaffed)", shift_name);
            } else {
                println!("  {:10}: {}", shift_name, names.join(", "));
            }
        }
    }

    println!("\n{}", "-".repeat(50));
    println!("Days worked per employee:");
    for employee in employees {
        println!("  {:10}: {} day(s)", employee.name, employee.days_worked);
    }

    println!("\n{}", "-".repeat(50));
    println!("Preference satisfaction:");
    let labels = ["1st choice", "2nd choice", "3rd choice"];
    for (rank, label) in labels.iter().enumerate() {
        println!("  {:16}: {}", label, stats.by_rank[rank]);
    }
    println!("  {:16}: {} (not preference-driven)", "Random backfill", stats.random_backfill);
    println!("  {:16}: {} (no shift had room same or next day)", "Unplaced", stats.unplaced);
}

fn main() {
    let mut rng = Rng::new(RANDOM_SEED);
    let mut employees = build_employees(&mut rng);
    let (schedule, stats) = build_schedule(&mut employees, &mut rng);
    print_schedule(&schedule, &employees, &stats);
}
