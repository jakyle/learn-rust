fn main() {

    // variable has OWNERSHIP over the number 2.
    let mut x = 2;

    // not transfering ownership, instead, we are COPYING this data into memory.
    let mut y = x;

    x += 10;
    y += 5;

    println!("x: {}, y: {}", x, y);

    // declaring initial variable here
    let name = "James".to_string();

    // Transfered the "James string" into the s1 variable
    let s1 = name;

    println!("{}", s1);

    // Ownership by Functions
    // transfering the "James" string to the print_name function
    // print_name(s1);
    // print_name(s1);  THis cannot be done, transering ownership
    // do a variable that was transfered on line 24

    take_a_number(x);


    // Reference pointer
    print_name_by_ref(&s1);
    print_name_by_ref(&s1);
    print_name_by_ref(&s1);
    print_name_by_ref(&s1);
    print_name_by_ref(&s1);
    print_name_by_ref(&s1);
    print_name_by_ref(&s1);
    print_name_by_ref(&s1);
    print_name_by_ref(&s1);
    print_name_by_ref(&s1);

    // mut reference
    let mut s2 = "Jake".to_string();

    let s2_mut_ref = &mut s2;
    // let s3_mut_ref = &mut s2; Cannot have more than ONE
    // Mutable REFERENCE for a given variable

    add_character_to_string(s2_mut_ref);

    print_name_by_ref(s2_mut_ref);

    // If a variable type does NOT implement COPY, whenever you re assign a variable, or PASS it to a function, you are 
    // transferring ownership.
    // if you DO NOT WANT to transfer ownership, you can assign a REFERENCE to that variable by using the '&' key.
    // IF you want to pass by reference and also have the ability to MUTATE the data that the reference is referencing, 
    // you have to use '&mut' instead.
    // HOWEVER, if you DO pass by mutable reference, you can ONLY HAVE one mutable reference at a time.


    // intro to structs

    let point = Point {x: 5, y: 13};

    let mut cup = Cup { 
        material: "ceramic".to_string(), 
        has_handle: true, 
        holds_liquid: true, 
        has_lid: false, 
        is_microwave_safe: true,
        durability: 20,
        length: 3,
        width: 3,
        height: 5
    };

    let cup_2 = &mut cup;

    cup_2.length = 4;

    println!("{:#?}", cup);

    println!("Cup volume = {}", cup.get_volume());
}

fn print_name(name: String) {
    println!("{}", name)

    // the "James" string is dropped out of memory
}

fn print_name_by_ref(name: &String) {

    println!("{}", name)
}

fn add_character_to_string(name: &mut String) {
    name.push('z');
}

fn take_a_number(x: i32) {

}


// struct, a struct is a it's OWN TYPE, it's purpose is to 
// simply contain data
struct Point {
    x: i32,
    y: i32
}

struct Pen {
    has_feather: bool,
    is_sharp: bool,
    has_ink: bool,
    ink_color: String,
    length: u8
}

struct AbilityScores {
    str: i32,
    dex: i32,
    con: i32,
    int: i32,
    wis: i32,
    cha: i32
}


#[derive(Debug)]
struct Cup {
    material: String,
    has_handle: bool,
    holds_liquid: bool,
    has_lid: bool,
    is_microwave_safe: bool,
    durability: i32,


    length: i32,
    width: i32,
    height: i32,
}

impl Cup {
    fn get_volume(&self) -> i32 {
        self.length * self.width * self.height
    }

    fn drink(&self) {
        // logic to drink from cup
    }
}