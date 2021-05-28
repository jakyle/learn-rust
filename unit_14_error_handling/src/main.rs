#![feature(int_error_matching)]

use std::{error::Error, num::{IntErrorKind, ParseIntError}, str::FromStr};

fn main() {


    let age = loop {
        let mut age_input = String::new();
        std::io::stdin().read_line(&mut age_input).unwrap();
        match age_input.parse::<i32>() {
            Ok(a) => break a,
            Err(e) => println!("hey, I asked for a number and and god damnit I'm going to get one!")
        }
    };


    let mut age_input = String::new();
    std::io::stdin().read_line(&mut age_input).unwrap();
    let age = age_input.parse::<i32>().ok();


    // this is a strategy to Always get an Ok result out of your Result.
    // this is a FORM of Error handling, we are "Handling" the error.
    let input = loop {
        match get_user_input("What is your name?") {
            Ok(m) => break m,
            Err(e) => println!("Invalid input: {}", e)
        }
    };


    // unwrap() will consume the Result, and GET the OK() value
    // out of the result.  HOWEVER, IF the Result Value is an
    // Err(), the program will Panic!, and your entire program
    // will shit down.  AVOID CALLING UNWRAP IF YOU CAN HELP.
    let x = "2".parse::<i32>().unwrap();

    // expect() will consume the Result, and GET the OK() value
    // out of the result.  HOWEVER, IF the Result Value is an
    // Err(), the program will Panic!, and your entire program
    // will shit down. In addition, expect() takes in a custom
    // error message for you to display in the terminal when the 
    // program crashes. AVOID CALLING EXPECT IF YOU CAN HELP IT.
    let x = "2".parse::<i32>().expect("I crashed lol");

    // unwrap or default will attempt to get the Ok() value of the Result
    // however, if there is an Err(), the code will get the Default 
    // value of that type instead.  for example, if you try to parse
    // to an number type, you will get 0 if you get an Err().
    let x = "a".parse::<i32>().unwrap_or_default();


    // unwrap or will attempt to get the Ok() value of the Result
    // however, if there is an Err(), the code will get the VALUE you 
    // provide into the unwrap_or(T) function.  for example, if you try to 
    // parse to a number, you will get 10 if you provide 10 as an argument
    // to the unwrap_or function.
    let x = "a".parse::<i32>().unwrap_or(10);


    let initial_age = 23;


    // unwrap_or_else() is simillar to unwrap_or in that its intended use is to 
    // get a default value IF your result is an Err().  The difference is, instead of 
    // providing a flat value as an argument, you provide a Function instead.  with this
    // function, you can provide custom logic to figure out your "default value".
    let relative_age = "ououou".parse::<i32>().unwrap_or_else(|x| {
        if initial_age % 2 == 0 {
            18
        } else {
            19
        }
    });


    // is_err() will return true IF your result is an Err()
    // however, you will NOT consume the result, which implies that
    // IF you still need to get the Err() or Ok() value OUT of the Result,
    // you need to still write code that gets the value out.
    if "a".parse::<i32>().is_err() {
        println!("couldn't parse!");
    }

    // is_ok() will return true IF your result is an Ok()
    if "1".parse::<i32>().is_ok() {
        println!("Hey, shits okay");
    }


    enum LegalAge {
        Drinking,
        Driving,
        NothingSpecial
    }

    // map() takes in a closure (a function) and if the Result is Ok(), the closure
    // will take the Ok value, and MAP it into a new value, ie. map a -> b.  In addition, 
    // this does NOT consume the result. which means you will still have to match on it, or 
    // unwrap somehow.  IF the Result is Err(), the closure provided to the map function will
    // NOT be called, map ONLY calls the closure IF the result is Ok().
    let value = "23".parse::<i32>().map(|x| {
        match x {
            x if x > 18 => LegalAge::Driving,
            x if x > 21 => LegalAge::Drinking,
            _ => LegalAge::NothingSpecial,
        }
    });

    // map_err() takes in a closure (a function) and if the result is Err(), the closure
    // will take the Err value, and MAP it into a new value, ie. map errA -> errB. In addition, 
    // this does NOT consume the result.  which means you will still have to match on it, or 
    // unwrap somehow.  IF the Result is Ok(), the closure provided to the map_err function will
    // NOT be called, map ONLY calls the closure IF the result is Err(), this function is the Inverse to 
    // map()
    let value = "hello".parse::<i32>()
        .map_err(|error| format!("tried to get a valid age, but couldn't, {}", error));


    // ok() will CONVERT the Result Into an Option.  IF the result is an Okay, the Option it's
    // converted into will be Some(T), which means it contains a value. Otherwise, if the result 
    // is an Err(), the Option will be None. of course, this consumes the Result and converts the Result
    // into an Option.
    let x = "a".parse::<i32>().ok();


    // err() will CONVERT the Result Into an Option.  IF the result is an Okay, the Option it's
    // converted into will be None, which means it contains NO value. Otherwise, if the result 
    // is an Err(), the Option will be Some(E). of course, this consumes the Result and converts the Result
    // into an Option.
    let x = "a".parse::<i32>().err();

    // and() test against two results, and we apply logical operator AND on both results, if both
    // results are Ok, your final result will be Ok, if ANY of the results is an Err(), then your
    // final result will be an error.  If your final result is okay, the Result returned will be the 
    // result that is passed INTO the and() function.
    let x = "2".parse::<i32>().and("1".parse::<i32>());

    // or() test against two results, and we apply logical operator OR on both results, if one
    // result is Ok, your final result will be Ok, if BOTH of the results is an Err(), then your
    // final result will be an error.  If your final result is okay, the Result returned will be the 
    // result that is passed INTO the or() function.
    let x = "2".parse::<i32>().or("1".parse::<i32>());

    // convert your Result into an iterator, it will ONLY have Some(T) value when you call next()
    // IF the result is Ok()
    let x = "2".parse::<i32>().into_iter();


    println!("relative age: {}", relative_age);

}

//                                              Ok        Err
fn convert_to_i32_then_add_5(s: &str) -> Result<i32, ParseIntError> {

    // This is the long hand form for whats happening on line 25
    let mut num = match s.parse::<i32>() {
        Ok(x) => x,
        Err(m) => return Err(m)
    };

    // This is the SAME exact code as line 19 - 22
    let mut num = s.parse::<i32>()?;

    num += 5;

    Ok(num)
}

fn get_num_between_1_10(msg: &str) -> Result<i32, Box<dyn Error>> {
    let input = get_user_input(msg)?;
    let input = input.parse::<i32>()?;

    if (1..=10).contains(&input) {
        Ok(input)
    } else {
        Err(Box::new(std::fmt::Error))
    }
}

fn get_user_input(msg: &str) -> Result<String, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input)
}


// Result !== i32  they are DIFFERENT TYPES

// THIS IS JUST an EXAMPLE ENUM
enum MyResult<T, E> {
    Ok(T),// the Ok value of the Result Enum contains the Expected value
    Err(E) // Err contains the Error "details", ie, what you get back IF you CANNOT get the OK value.
    // The idea with using Result, is you WANT the OK value, however, shit may fuck up and you MAY get 
    // an ERROR instead.
}
