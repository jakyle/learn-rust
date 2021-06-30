use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, TimeZone, Timelike, Utc, Duration};

fn main() {
    // 2021-06-30T:0256:10Z
    let now = Utc::now();
    println!("{}", now);
    println!("{}", now.to_rfc2822());
    println!("{}", now.to_rfc3339());

    println!("{}", now.timestamp());
    println!("{}", now.second());

    println!("{}", now.day());
    println!("{}", now.year());
    println!("{}", now.month());
    println!("{}", now.weekday());

    // let user_date = loop {

    //     let mut buffer = String::new();
    //     println!("please enter a valid date time [...]");
    //     std::io::stdin().read_line(&mut buffer).unwrap();

    //     match DateTime::parse_from_rfc2822(&buffer.trim()) {
    //         Ok(dt) => break dt,
    //         Err(msg) => println!("Try again mutha fucka!, {}", msg)
    //     }
    // };

    let user_year = 1993;
    if (2013..=2021).contains(&user_year) {

    }

    let user_date = loop {

        let mut buffer = String::new();
        println!("please enter a valid date [ mm/dd/yyyy ]");
        std::io::stdin().read_line(&mut buffer).unwrap();

        match NaiveDate::parse_from_str(&buffer.trim(), "%m/%d/%Y") {
            Ok(dt) => break dt,
            Err(msg) => println!("Try again mutha fucka!, {}", msg)
        }
    };

    let user_time = loop {

        let mut buffer = String::new();
        println!("please enter a valid time [ mm/dd/yyyy ]");
        std::io::stdin().read_line(&mut buffer).unwrap();

        match NaiveTime::parse_from_str(&buffer.trim(), "%H:%M:%S") {
            Ok(dt) => break dt,
            Err(msg) => println!("Try again mutha fucka!, {}", msg)
        }
    };


    let user_date_time = Utc.from_local_date(&user_date).unwrap();

    let future_user_date = user_date_time + Duration::days(32) + Duration::seconds(32);

    println!("{}", user_date);
    println!("{}", user_time);
    println!("{}", user_date_time);


    println!("Hello, world!");
}