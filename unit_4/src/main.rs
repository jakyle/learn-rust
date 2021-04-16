fn main() {

    let mut coconut = Coconut::new("brown".to_string(), "fuzzy".to_string(), 2.0);

    // c1 is storing a REFERENCE to coconut.color, it is a pointer that POINTS to the ACTUAL coconut.color
    let c1 = &coconut.color;

    println!("coconut color: {}", coconut.color);
    println!("coconut weight: {}", coconut.weight);
    println!("coconut age: {}", coconut.age);
    println!("coconut is cracked {}", coconut.is_cracked);

    coconut.crack_coconut();

    let mut coconut_2 = Coconut::new("green".to_string(), "smooth".to_string(), 3.0);

    coconut_2.crack_coconut();

    println!("coconut 2 is cracked?: {}", coconut_2.is_cracked);
    println!("is coconut 2 expired?: {}", coconut_2.is_expired());
}


// Structs are simply structures that contain properties that make up the struct you are defining.
// Also, a struct is it's OWN type


struct Coconut {
    color: String,
    texture: String,
    weight: f32,
    is_cracked: bool,
    age: i32
}

impl Coconut {
    fn new(color: String, texture: String, weight: f32) -> Self {
        Coconut {
            color,
            texture,
            weight,
            is_cracked: false,
            age: 0
        }
    }

    fn crack_coconut(&mut self) {
        self.is_cracked = true;
    }

    fn is_expired(&self) -> bool {
        self.age > 5
    }
}


// Create a room application. this room application will take input from the user
// and we are going to take in the width, height, and length of a room from a user.
// after getting all of these dimensions, we will thne store them in a "Room" struct
// after creating your Room struct, implement the Room struct and create three methods
// get_area, and get_volume, this will then refer to the properties in the struct (self)
// and print the calculations to the console.