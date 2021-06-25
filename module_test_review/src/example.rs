pub const PI: f64 = 3.14;

pub fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

pub struct Person {
    pub name: String,
    pub age: i32
}

impl Person {
    pub fn get_name_age(&self) -> String {
        format!("name: {}, age: {}", self.name, self.age)
    }
}

pub enum Things {
    One,
    Two(i32),
    Three(Person),
    Four { name: String, age: i32 }
}

