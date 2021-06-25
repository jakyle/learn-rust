fn main() {

    let rock = RPS::Rock;

    let ford = Car::new(4, "F-150".to_string(), false);
    let honda = Car::new(4, "civic".to_string(), true);
    let toyota = Car::new(4, "camry".to_string(), false);

    println!("{}", ford.get_model());
    println!("{}", honda.get_model());
    println!("{}", toyota.get_model());

    ford.drive();

    println!("My ford has {} tires", ford.get_tires());


    let mut cars = vec![]; 
    for i in 0..3 {
        println!("give me the model of car number {}", i+1);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        cars.push(Car::new(4, input.trim().into(), false));
    }

    println!("Look at all of these cars!");
    for car in cars {
        println!("model: {}", car.get_model());
    }

}

#[derive(Debug)]
struct Car {
    number_of_tires: i32,
    model: String,
    is_imported: bool
}

// implement
impl Car {

    // this is NOT A METHOD, this is an associated function
    fn new(number_of_tires: i32, model: String, is_imported: bool) -> Self {
        Self {
            number_of_tires,
            model,
            is_imported
        }
    }

    // a Method is a function that is OWNED by that INSTANCE of the struct
    fn get_tires(&self) -> i32 {
        self.number_of_tires
    }

    fn set_model(&mut self, model: String) {
        self.model = model;
    }

    fn get_model(&self) -> &str {
        &self.model
    }

    fn drive(&self) {
        // self will drive now
    }
}

fn new(number_of_tires: i32, model: String, is_imported: bool) -> Car {
    Car {
        number_of_tires,
        model,
        is_imported
    }
}

// The enum VALUE can ONLY BE one of the ARMS in the enum itself
enum RPS {
    Rock,
    Paper,
    Scissor
}

enum OfficeSupplies {
    Printer {
        has_cartridge: bool,
        has_paper: bool
    },
    Stapler {
        has_stables: bool
    },
    StickyNotes(u32)
}