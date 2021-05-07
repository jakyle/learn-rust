#![feature(slice_group_by)]

fn main() {


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

    get_string_length(string_ref); // rust TYPE COERCIoN a &String to a &str FOR you if 
    // you pass in a String by reference as a function argument

    let first_three_characters = &name[0..3];

    // Slice,... just Slice
    // a SLICE is it's OWN type
    let mut names = vec!["Jimmy", "Ryan", "Jake", "Wendy"];
    // THe ONLY time you should pass in a Vector as a PARAMETER in a function 
    // is IF you ONLY plan on ADDING or REMOVING elements from that Vector
    // in that function.  Instead of passing in a Vec, pass in a Slice instead 
    // 😀

    pass_in_slice(&names); // Although we are passing in a &Vec<&str> (A reference to a Vector of String slices)
    // in this function, rust is Coercing the &Vec<&str> INTO a &[&str]  (A Slice of string slices)

    let numbers = [1, 2, 3, 4, 5];

    // let number = numbers[10]; CAnnot do this, index out of range, our array ONLY holds 5

    let numbers_slice = &mut [1, 2, 3, 4, 5, 6, 7, 8, 9, 10][..];


    let bad_boys_names = &names[0..2]; // by putting the square brackets 
    // AFTER your collection variable, you are getting a SLICE of it instead.


    let mut split_numbers = numbers_slice.split(|&x| x % 2 == 0);

    let slice_one = split_numbers.next();

    let slice_two = split_numbers.next();

    let slice_three = split_numbers.next();

    let joined_arrays = [[1, 2], [3, 4]].join(&0);

    println!("{:?}", joined_arrays);

    let joined_arrays = [[1, 2], [3, 4], [5, 6]].concat();

    println!("{:?}", joined_arrays);

    numbers_slice.reverse();

    println!("{:?}", numbers_slice);

    numbers_slice.reverse();

    for x in numbers_slice.chunks(2) { // Chunk RETURNS an Iterator over it's chunks
        println!("{:?}", x);
    }


    if let Some(x) = numbers_slice.first() {
        println!("first element: {}", x);
    } // WITHOUT creating an iterator, get the first element from that slice


    if let Some(x) = numbers_slice.last() {
        println!("last element: {}", x);
    } // WITHOUT creating an iterator, get the last element from that slice

    if numbers_slice.is_empty() {  // DOES not create an iterator
        println!("Hey, I'm empty!");
    } else {
        println!("I contain elements");
    }

    let x = numbers_slice.get(10); // This is the SAFE way to explicitly get an element at a desired index, BECAUSE get() returns an option.

    for x in numbers_slice.iter() {
        println!("{}", x);
    }

    for window in numbers_slice.windows(2) {
        println!("{:?}", window);
    }

    //   [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] window of size 2
    //   [1, 2]
    //       [2, 3]
    //         [3, 4]
    //            [4, 5]
    //  [1, 2, 3, 4, 5, 6, 7, 8, 9]   window size of 9
    //      [2, 3, 4, 5, 6, 7, 8, 9, 10]

    // practical use cases for slices!


    // windows example

    let characters = "I'm going to the movies later yall... jk, pandemic lol, I'm fucking cool guys.".chars().collect::<Vec<char>>();

    for window in characters.windows(4) {
        println!("{:?}", window);
    }

    // group by
    let mut numbers_repeated = [1, 2, 2, 2, 3, 3, 4, 5, 5, 6, 6, 8, 7, 9, 9, 11, 12, 11, 12, 1, 1, 2, 3, 4, 4];

    numbers_repeated.sort();
    let grouped = my_sort(&mut numbers_repeated).group_by(|a, b| a == b);

    for group in grouped {
        println!("{:?}", group);
    }
}

fn get_string_length(word: &str) -> usize {
    word.len()
}

fn pass_in_slice(collection: &[&str]) {

}

fn my_sort<T>(slice: &mut [T]) -> &mut [T]
    where T: PartialOrd + Ord {
        slice.sort();
        slice
}