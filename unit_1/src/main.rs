fn main() {

    let mut thing = "sup"; // &str            String
                                // ^string slice,  
    thing = "32";

    let mut age: i32 = 32;

    age = 16 + 13; // age = 29


    // Equality operators
    // 
    // == -> Equals, checks if left and right
    //       hand values are equal, ie.  '1 == 1'
    //
    // !=  (! =) Not Equals, checks if left and right operands
    //     are NOT equal,  ie. 2 != 1  (evaluates to true)
    //
    // > -> Greater Than, checks if left hand operand
    //      is greater than right operand, ie. '2 > 1'
    //
    // >= (> =)-> Greater THan OR equal...
    //
    // <  -> Less than
    //
    // <= (< =) -> Less than OR equal 

    age = 32;
    
    // basic if statement
    if age >= 21 {
        println!("What do you want to drink?");
    } else if age >= 18 {
        println!("You can smoke, but you can't drink tee hee.");
    } else {
        println!("Get outta here you ringus!");
    }

    // treating an if statement as an expression
    let bartender_message = if age >= 21 {
        "What do you want to drink?"
    } else {
        "2"
    };

    println!("{}", bartender_message);


    // Logical Operators - these operators allow you to 
    //   chain multiple boolean expression to create a larger
    //   boolean expression
    //
    // && ->  Left expression AND Right expression have to be true
    //  in order for this entire expression to evaluate to true
    //  ie.  1 == 1 && 2 == 2 <-- true
    //       1 == 2 && 2 == 2 <-- false
    //
    // || -> Left expression OR Right expression needs to evaluate to true
    //  in order for this entire expression to evaluate to true
    //  ie.  1 == 1 || 2 == 2 <-- true
    //       1 == 2 || 2 == 2 <-- true
    //       1 == 2 || 1 == 2 <-- false
    //
    //
    // ! -> Not operator gets the INVERSE of the current 
    //     boolean expression
    //     ie.   !(1 == 1) <-- false
    //           !(1 == 2 || 1 == 2) <-- true

    let mut cannot_drink = !(age >= 21);


    // Loops
    loop {
        println!("sup dude");
        break;
    }

    let loop_value = loop {
        break 5
    };

    // While loops will ONLY loop if the condition expressed
    // next to the while keyword evaluates to true
    let mut counter = 0;
    while counter <= 5 {
        println!("counter == {}", counter);
        counter += 1;
    }

    // Loops over a collection of items, the looping will
    // STOP the moment said collection is out of 
    // Elements to loop over
    // for loops are Syntactic sugar for "Iterators"
    for letter in 'a'..='z' {
        println!("letter == {}", letter);
    }

    for letter in "Alabama".."Texas" {
        println!("letter == {}", letter);
    }

    for x in 1..=100 {
        match x {
            21 => println!("you are old enough to drink"),
            13 => println!("you are a teenager, wow!"),
            18 => println!("you can smoke, Wow!"),
            26 => {
                println!("insurance is cheaper... maybe?");
                println!("you can now rent a car!");
            },
            0..=30 => println!("you are between age 0 to 30"),
            0..=17 => println!("Epstien"),
            30..=100 => println!("please help"),
            2 | 3 => println!("... fuck parenting this age"),
            _ => println!("nothing special with your age...")
        }
    }

    // Compound Types
    // tuples
    let our_first_tuple = (32, "James");

    let age = our_first_tuple.0;
    let name = our_first_tuple.1;

    let (jake_age, jake_name) = (31, "Jake");

    println!("I'm {}, and I am {} yearls old.", jake_age, jake_name);

    match our_first_tuple {
        (8, "Jerry") => println!("you're just a baby boy!"),
        (_, "Larry") => println!("LARYYYYY!!!!"),
        (40, _) => println!("Someones over the hill lol"),
        (32, "James") => println!("Hey guys tee hee 🧨"),
        (_, _) => println!("Nothing to see here")
    }

    // array
    // a collection of one TYPE, and has a SET STATIC
    // LENGTH, that will never change
    //                             index: 0  1  2  3  4
    let mut numbers = [0; 5]; // [0, 0, 0, 0, 0]
    let first_element = numbers[0];
    // let seventh_element = numbers[7]; will not compile, out of range

    numbers[3] = 340;

    println!("{:?}", numbers);

    for x in &numbers {
        println!("{}", x);
    }

}
