fn main() {

    let funny: Vec<i32> = vec![1];

    let string_length = get_string_length("hello dude, i am jacob hoover");     
    let can_drink = can_drink(23);
    let capitalized_word = capitalize("tinyword");

    // String Slices
    // when do I need a regular string? 
    // If you NEED complete ownership over that String, and you 
    // may MODIFY that string, you want the String type OR if you want to MODIFY the data 
    // in the String (characters or bytes).
    let mut name = "Jimmy".to_string();

    // a Slice, is ALWAYS a REFERENCE to a SECTION of elements in a collection
    // String::from("Jimmy") ex->  "Jim"
    // you CANNOT make a mutable string
    let name_slice = &mut name[..];


    let other_name = "Wendy"; // whenever you create a string literal or
    // string slice without Referencing an already existing String, the compiler
    // will handle referencing a non existent variable for you.

    let string_ref = &name;

    get_string_length(&name); // rust CAST a &String to a &str FOR you if 
    // you pass in a String by reference as a function argument

    let first_three_characters = &name[0..3];

    // Slice,... just Slice
    // a SLICE is it's OWN type
    let mut names = vec!["Jimmy", "Ryan", "Jake", "Wendy"];
    // THe ONLY time you should pass in a Vector as a PARAMETER in a function 
    // is IF you ONLY plan on ADDING or REMOVING elements from that Vector
    // in that function.  Instead of passing in a Vec, pass in a Slice instead 
    // 😀

    let bad_boys_names = &names[0..2]; // by putting the square brackets 
    // AFTER your collection variable, you are getting a SLICE of it instead.

    use std::collections::VecDeque;
    let set: VecDeque<i32> = VecDeque::new();


    let set_slice = &set.as_slices();
}


fn make_laugh(funny: &[&str]) -> bool {
    
    true 
}

fn get_funny_numbers(nums: &[i32]) -> bool {
    true
}

fn add_numbers(number_one: i32, number_two: i32) -> i32 {
    let sum = number_one + number_two;
    sum
}

fn capitalize(word: &str) -> String {
    word.to_uppercase()
} 

fn can_drink(age: i32) -> bool {
    age >= 21
}

fn get_string_length(word: &str) -> i32 {
    word.chars().count() as i32
}