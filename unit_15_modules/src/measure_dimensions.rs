pub mod triangle_dimensions;
pub mod circle_dimensions;

use rand::Rng;

#[derive(Debug)]
pub enum Shapes {
    Triangle,
    Circle,
    Square
}

impl Shapes {
    pub fn shapes_message(&self) {
        println!("lol I'm a shape: {:?}", self);
    }
}

pub struct Room {
    pub length: i32,
    pub width: i32, 
    pub height: i32,
    id: i32
}

impl Room {

    pub fn new(length: i32, width: i32, height: i32) -> Self {

        let mut rng = rand::thread_rng();
        let id = rng.gen_range(0..2_00_000_000);
        Self {
            length,
            width,
            height,
            id
        }
    }

    pub fn get_volume(&self) -> i32 {
        self.get_area() * self.height
    }

    fn get_area(&self) -> i32 {
        self.length * self.width
    }
}

fn im_in_measure_dimensions() {
    println!("lol I'm in measure dimensions!");
}

fn calling_im_in_measure_dimensions() {
    im_in_measure_dimensions()
}

fn get_area() {

}


