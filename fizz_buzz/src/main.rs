fn main() {
    fizz_buzz_iter().for_each(|s| println!("{}", s));
}

fn fizz_buzz_iter() -> impl Iterator<Item=String> {
    (0..=100)
        .map(|i| get_fizz_buzz_message(i, 3, 5))
}

fn get_fizz_buzz_message(i: i32, x: i32, y: i32) -> String {
    match i {
        i if i % x == 0 && i % y == 0 => "FizzBuzz".to_string(),
        i if i % x == 0 => "Fizz".to_string(),
        i if i % y == 0 => "Buzz".to_string(),
        _ => i.to_string(),
    }
}


// In this exercise, create a function that takes in zero input, and gives zero output, call
// this function fizz_buzz()
// What you will do, is loop over the numbers 0..=100
// and as you loop through this range of numbers
// if the number you are currently looping over is evenly divisible by 3, print to the console "fizz"
// if the number you are currently looping over is evenly divisible by 5, print to the console "buzz"
// if the number you are currently looping over is evenly divisible by 5 AND 3, print to the console "fizzbuzz"
// if the current number is NOT evenly divisible by either of the numbers noted above, simply print the number into the console 