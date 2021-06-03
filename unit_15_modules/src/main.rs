// in rust, ANY file in the src directory that ends in *.rs IS considered a Module
// the same exact that using the ``mod name {...}`` is a module.
// so there are two ways to create modules in rust.  you can Either create files
// or you can explicitly create a mod, using the mod keyword inside your mod file.

mod measure_dimensions;
mod math;

// It's modules time!

// the 'use' keyword is used to pull in ANY public items inside other modules so those public 
// items can be used in your CURRENT scope or module.
use rand::Rng;
use measure_dimensions::{*, triangle_dimensions::{ get_triangle, get_triangle_perimeter } };
use math::{get_area, add_two_numbers};


fn main() {

    let x = add_two_numbers(1, 1);

    let room = measure_dimensions::Room::new(20, 15, 10);

    println!("Volume of Room: {}", room.get_volume());

    let square = measure_dimensions::Shapes::Square;

    square.shapes_message();

    let circle_area = measure_dimensions::circle_dimensions::get_area(20.);
}


fn hello() {
    let x = add_two_numbers(1, 2);
}

fn get_triangle_dimensions_dude() {
    get_triangle();
}
