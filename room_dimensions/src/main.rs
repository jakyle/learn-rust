use std::io::stdin;

struct Room {
    length: i32,
    width: i32,
    height: i32,
}

impl Room {
    fn get_area(&self) -> i32 {
        self.length * self.width
    }

    fn get_volume(&self) -> i32 {
        self.length * self.width * self.height
    }
    fn get_perimeter(&self) -> i32 {
        2 * (self.length + self.width)
    }
}

fn get_user_int_input(message: &str) -> i32 {
    loop {
        println!("{}", message);
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");

        if let Ok(x) = input.trim().parse::<i32>() {
            break x
        }

        println!("Invalid input");
    }
}

fn main() {
    let length = get_user_int_input("Please enter length of room");
    let width = get_user_int_input("Please enter width of room");
    let height = get_user_int_input("Please enter height of room");

    let room = Room {
        length,
        width,
        height,
    };

    println!("Room Perimeter: {}", room.get_perimeter());
    println!("Room Area: {}", room.get_area());
    println!("Room Volume: {}", room.get_volume());
}
