use std::io::stdin;

enum MyOption<T> {
    Some(T),
    None,
}

enum MileStoneAges {
    Thirteen,
    Eighteen,
    TwentyOne,
    TwentySix,
    NothingSpecial,
}

enum ElectronicDevices {
    Phone {
        phone_number: String,
        screen_size: i32,
    },
    Vacuum {
        suck_power: i32,
        is_bag_full: bool,
    },
    HairDryer(bool),
}

impl MileStoneAges {
    fn convert_from_int(age: i32) -> Self {
        match age {
            13 => MileStoneAges::Thirteen,
            18 => MileStoneAges::Eighteen,
            21 => MileStoneAges::TwentyOne,
            26 => MileStoneAges::TwentySix,
            _ => MileStoneAges::NothingSpecial,
        }
    }
}

fn main() {
    let mut user_age_input = String::new();

    stdin()
        .read_line(&mut user_age_input)
        .expect("Did not enter a correct string");

    let age = user_age_input.trim().parse::<i32>().unwrap();

    let milestone_age = MileStoneAges::convert_from_int(age);

    let message = match milestone_age {
        MileStoneAges::Thirteen => "you're a teenager",
        MileStoneAges::Eighteen => "you can drive and vote and buy cigs",
        MileStoneAges::TwentyOne => "you can drink",
        MileStoneAges::TwentySix => "you can rent... a car",
        MileStoneAges::NothingSpecial => "nothing special here",
    };

    println!("{}", message);

    // enums with value

    let electronic_device = ElectronicDevices::HairDryer(true);

    match &electronic_device {
        ElectronicDevices::Phone {
            phone_number,
            screen_size,
        } => {
            println!(
                "Hey, call me on {}, you'll never believe that my screen size is {} inches!",
                phone_number, screen_size
            );
        }
        ElectronicDevices::Vacuum {
            suck_power,
            is_bag_full,
        } => {
            println!("OH man, the suck power on this vacuum is {}", suck_power);
        }
        ElectronicDevices::HairDryer(can_blow) => {
            println!("oh, I'm running late for work! I hope my hair dryer can blow! can hair dyer blow? {}", can_blow);
        }
    }

    if let ElectronicDevices::HairDryer(can_blow) = &electronic_device {
        println!("hey, maybe I can blow? {}", can_blow);
    };

    // how the standard library uses Enums, ie, the Option type

    let optional_age = Some(32);

    if let Some(x) = optional_age {
        println!("Hey, I'm {} years old!", x);
    }

    let added_numbers = add_numbers(5, 13, None);

    let other_added_numbers = add_numbers(13, 6, Some(40));
}

fn add_numbers(x: i32, y: i32, z: Option<i32>) -> i32 {
    let mut sum = x + y;

    if let Some(z) = z {
        sum += z;
    }

    sum
}
