use crate::math::get_area as get_math_area;
use rand::thread_rng as generate_rng_num;

fn call_parent_measure_dimension() {
    super::im_in_measure_dimensions();
}

fn lol() {
    let thread_rng = generate_rng_num();
}

pub fn get_area(radius: f64) -> f64 {
    3.14 * radius * radius
}

pub fn get_math_my_area() {
    get_math_area();
}

pub mod sphere_dimensions {
    fn call_grand_parent_measure_dimension() {
        super::super::calling_im_in_measure_dimensions();
    }

    fn get_grand_parents_area() {
        super::super::get_area();
    }
}
