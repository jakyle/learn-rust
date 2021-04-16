use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    let def = 15;
    let attack = 8;
    let (chance, atk_roll, did_hit) = calc_atk_roll(attack, def);

    println!("Chance to hit: {}", chance);
    println!("attack roll: {}", atk_roll);

    match did_hit {
        true => println!("You hit!"),
        false => println!("You Miss!"),
    }
}

pub fn calc_atk_roll(atk_bonus: u8, def_bonus: u8) -> (u8, u8, bool) {
    let chance_to_hit = num::clamp(
        calc_to_hit_formula(def_bonus as f32, atk_bonus as f32),
        0.,
        100.,
    ) as u8;

    let mut rng = rand::thread_rng();
    let attack_roll = rng.gen_range(1..=20) + atk_bonus;

    (chance_to_hit, attack_roll, attack_roll >= def_bonus)
}

fn calc_to_hit_formula(def: f32, atk: f32) -> f32 {
    ((21. - (def - atk)) / 20.) * 100.
}

// fn main() -> Result<(), std::io::Error> {
//     let mut rng = rand::thread_rng();

//     loop {
//         let user_guess = loop {
//             let x = match get_user_input("Enter a number between 1-10:")?.parse::<u8>() {
//                 Ok(x) => x,
//                 Err(_) => continue,
//             };

//             if (1..=10).contains(&x) {
//                 break x;
//             }
//         };

//         let random_number: u8 = rng.gen_range(1..=10);

//         let number_message = format!(
//             "\nYour number is: {}.\nOur number is: {}",
//             &user_guess, &random_number
//         );
//         match user_guess.cmp(&random_number) {
//             Ordering::Less => println!("Too small!{}", number_message),
//             Ordering::Greater => println!("Too big!{}", number_message),
//             Ordering::Equal => println!("You win!{}", number_message)
//         }

//         if get_user_input("Do you want to play again? [y/n]")? != "y" {
//             break;
//         }
//     }

//     Ok(())
// }

fn get_user_input(message: &str) -> Result<String, std::io::Error> {
    println!("{}", message);
    let mut s = String::new();
    stdin().read_line(&mut s)?;
    Ok(s.trim().into())
}

// UAC - (User Acceptance Criteria) ->
// summary: Create a guessing number game
// a user will enter a number, and we will reveal
// if the number chose the correct number!
//
//  1. the user is ONLY allowed to enter a number between 1 - 10
//  2. if the user guessed correctly, inform the user that they chose correctly
//  3. if the user guessed incorrectly, inform the user they chose incorrectly
//  4. have application run at all times.
//  5. The user should have a choice whether to play again or not
//

//  1. be able to enter a number
//  1b. make sure the users input is between 1 - 10, ie it's valid input
//  1c. if number is invalid, have them enter a number again :(
//  2. a number needs to be generated
//  3. program needs to check if the number the user entered is the same number
//     that the program randomly generated
//
//  4. display message based on if the user guessed correctly or not
//  5. prompt the user if they want to play the game again
//  6. the program needs to repeat if the user DOES want to play again
//  7. if the user does not want to play again, the program should close

// what we do know, our constraints
// 1. this is a console application
