# Assignment 4 - Employee Shift Scheduler (Python + Rust)

Two implementations of the same employee scheduling problem (see `assignment.md`
for the full prompt), written in Python and Rust to contrast how each language
expresses the same conditionals, loops, and branching.

## Rules implemented

- Company runs 7 days/week with Morning, Afternoon, and Evening shifts.
- No employee works more than one shift per day.
- No employee works more than 5 days per week.
- Every shift needs at least 2 employees per day (randomly backfilled from
  employees who haven't hit 5 days yet if preferences alone don't cover it).
- Shifts cap out at 4 employees; when an employee's top choice is full, the
  scheduler falls through to their next-ranked choice the same day, or the
  next day if every choice is full (see "Shift conflicts" below).
- **Bonus:** employees rank all three shifts in priority order per day (1st
  choice, 2nd choice, 3rd choice) instead of naming one preferred shift. The
  scheduler tries each employee's choices in rank order and only falls back
  to a lower-ranked choice, or the next day, once a higher choice is
  unavailable. The schedule output prints a "Preference satisfaction"
  summary (how many people got their 1st/2nd/3rd choice, how many were
  placed by random backfill, how many couldn't be placed) so the bonus's
  "accommodate preferences as much as possible" claim is backed by a number.

Employee names and ranked shift preferences per day are generated with a
seeded RNG rather than typed in interactively, so the program runs
end-to-end and produces a reproducible schedule to screenshot. Swapping in
real stdin input would only change `build_employees`; the scheduling logic
is unaffected.

## Shift conflicts and the "day budget"

A purely greedy day-by-day scheduler (fill preferences for Monday, then
Tuesday, etc.) runs into a real bug: employees who are popular / flexible get
assigned preferred shifts early in the week, hit their 5-day cap by Thursday
or Friday, and by Saturday/Sunday there's nobody left to meet the 2-person
minimum. Both implementations fix this with a small reservation scheme: at
the start of each day, the scheduler computes how much total employee-day
capacity must be preserved for the minimums on every day still to come, and
only spends the leftover ("discretionary budget") on assignments above a
shift's minimum. An employee whose ranked choices are all either full or
out of today's discretionary budget is carried over to the next day, where
a fresh budget applies and their ranked choices are tried again. This was
verified against roughly 50 different random seeds in each language with no
constraint violations.

## Running it

**Python** (standard library only):

```
cd python
python scheduler.py
```

**Rust** (no external crates, builds offline):

```
cd rust
cargo run
```

## Bonus

Priority-ranked preferences are implemented in both languages (see above).
A GUI for input/output is also listed as optional in the assignment and is
not implemented here; the core requirements (1-4) plus the ranking bonus
are fully implemented and verified in both languages.
