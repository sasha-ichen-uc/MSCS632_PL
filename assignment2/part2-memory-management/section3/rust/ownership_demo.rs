// Rust: Ownership and Borrowing demo
//
// Rust manages memory with NO garbage collector and NO manual free().
// Instead, every value has exactly one OWNER. When the owner goes out of
// scope, the compiler inserts a call to `drop` automatically (RAII).
// The BORROW CHECKER enforces, at compile time, that you never read/write
// memory that has been freed or moved away -- eliminating dangling
// pointers and use-after-free as a whole class of runtime bugs.

fn takes_ownership(v: Vec<i32>) -> i32 {
    // `v` is MOVED into this function; the caller no longer owns it.
    let sum: i32 = v.iter().sum();
    sum
    // `v` (the heap-allocated Vec buffer) is freed HERE, automatically,
    // when it goes out of scope at the end of this function.
}

fn borrows(v: &Vec<i32>) -> i32 {
    // `v` is BORROWED (read-only reference); ownership stays with the caller.
    v.iter().sum()
}

fn mutably_borrows(v: &mut Vec<i32>) {
    v.push(100);
}

fn main() {
    // Heap-allocated, dynamically-sized memory (like malloc in C++,
    // but freed automatically).
    let numbers = vec![1, 2, 3, 4, 5];
    println!("numbers = {:?}", numbers);

    // Borrow immutably first -- `numbers` is still valid afterward.
    let total = borrows(&numbers);
    println!("Sum via immutable borrow: {}", total);

    // Borrow mutably to modify in place.
    let mut numbers = numbers;
    mutably_borrows(&mut numbers);
    println!("After mutable borrow push(100): {:?}", numbers);

    // Now MOVE ownership into takes_ownership(). After this call,
    // `numbers` is no longer valid -- the compiler tracks this statically.
    let sum = takes_ownership(numbers);
    println!("Sum after move into takes_ownership: {}", sum);

    // Uncommenting the next line is a COMPILE-TIME error, not a runtime
    // crash: `numbers` was moved on the previous call, so using it again
    // is rejected before the program can ever run.
    // println!("{:?}", numbers); // error[E0382]: borrow of moved value

    println!("main() ending -- all heap memory above is freed automatically here.");
}
