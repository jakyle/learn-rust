pub fn sandbox() {
    for x in 0..10 { // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
        println!("iteration: {}", x);
    }

    let mut x = 0;
    while x < 10 {
        x += 1;
    }

    let mut counter: i32 = 0;
    loop {
        counter += 1;

        println!("counter: {}", counter);
        if counter == 10 {
            break
        }
    }

    let input: i32 = loop {
        println!("enter a number between 1-5");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer);

        let input = buffer.trim().parse::<i32>().unwrap();
        match validate_input(input) {
            Ok(v) => break input,
            Err(e) => println!("{}", e)
        }
    };


    let x = 1 + 2 / 3 * 5;
    let is_something = x >= 2 || x >= 4 && x % 2 == 0;
    let statement = if x == 2 {
        "sup"
    } else {
        "nope"
    };                                                  

    let message = match input { 
        1 => "yep, that was one...",
        2 => "now theres a couple of you 👩🏼👨🏼",
        3 => "wow it's the three stooges 👨🏻👳🏻‍♂️👨🏻‍🦲",
        4 => "2 * 2, holy 🐄!",
        5 => "give me a high five! ✋🏻",
        _ => "you're not one of the five you idiot 😡"
    };

    println!("{}", message);
}

fn validate_input(input: i32) -> Result<i32, &'static str> {
    if (1..=5).contains(&input) {
        Ok(input)
    } else {
        Err("incorrect input")
    }
}