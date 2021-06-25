extern crate rand;

pub mod example;
pub mod help_with_structs;
pub mod control_flow;

use example::{Things, Person, add_numbers};
use rand::{Rng, thread_rng};

struct Dog {
    name: String,
    age: i32
}

impl Dog {
    fn new(name: String, age: i32) -> Self {
        Self {
            name,
            age
        }
    }
}

fn main() {

//     let dog = Dog::new("Coco".into(), 1);

//     let mut rng_thread = thread_rng();
//     rng_thread.gen_range(0..2);


//     let thing = Things::One;
//     let other_thing = Things::Two(2);
//     let final_thing = Things::Three (Person {
//         name: "James".to_string(),
//         age: 32
//     });

//     match other_thing {
//         Things::One => println!("I'm the first choice!"),
//         Things::Two(x) => println!("I'm the second choice, with a value of: {}", x),
//         Things::Three(example::Person { age, name }) => println!("I'm the final choice, my name is: {}, I am {} years old", name, age),
//         Things::Four { name, age } => println!("I'm the final choice, my name is: {}, I am {} years old", name, age),
//     }

//     let person = Person {
//         name: "James".to_string(),
//         age: 32
//     };

//     println!("{}", person.get_name_age());

//     let result = add_numbers(2, 3);
    control_flow::sandbox();
}