// Rust: this file intentionally violates ownership rules so we can capture
// the REAL compiler diagnostic. It will not compile.

fn takes_ownership(v: Vec<i32>) -> i32 {
    v.iter().sum()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum = takes_ownership(numbers); // `numbers` is moved here
    println!("sum = {}", sum);

    // Use-after-move: `numbers` no longer owns its heap buffer.
    // A C++ equivalent (using it after a raw pointer was freed) would be
    // a silent, undefined-behavior dangling-pointer bug at RUNTIME.
    // Rust instead refuses to COMPILE this program at all.
    println!("numbers = {:?}", numbers);
}
