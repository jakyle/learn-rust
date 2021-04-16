fn main() {
    println!("Hello, world!");

    let x = add_two_ints(2, 3) + 13;
    let y = add_two_ints(x, 10);

    let z = add_two_ints(
        add_two_ints(x, y),
        add_two_ints(10, 22)
    );

    println!("x: {}, y: {}, z: {}", x, y, z);
}


// f(x) -> y  - Functions represents modular code that takes
// in input, and returns and output
//
// //  output symbol (- >)
// most popular type of function takes in INPUT AND gives OUTPUT
fn add_two_ints(x: i32, y: i32) -> i32 {
    x + y
}

// function with ONLY output
fn add_two_and_three() -> i32 {
    add_two_ints(2, 3)
}

// function with ONLY inputs
fn print_summed_numbers(x: i32, y: i32) {
    println!("x: {}, y: {}", x, y)
}

// function with NO input and NO output
fn i_do_nothing() {
    println!("nothing to see here folks");
}