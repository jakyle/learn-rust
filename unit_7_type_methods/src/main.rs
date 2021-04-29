fn main() {
    // methods that exist on types from the std (standard) library

    // i32

    let x: i32 = 2;

    let y = x.pow(3);

    println!("x: {}, x cubed: {}", x, y);

    let x = i32::abs(-32 - 5);

    println!("absolute value of x: {}", x);

    let x = 32.min(20);

    println!("min of x: {}", x);

    let x = 8.max(10);

    println!("max of x: {}", x);

    let x = 120.clamp(10, 20);

    println!("clamped value of x: {}", x);

    // Strings
    // 01234
    let mut name = "James".to_string();

    let name_uppered = name.to_uppercase();

    println!("name uppercased {}", name_uppered);

    name.push_str(" Jackson");

    println!("full name: {}", name);

    name.push('z');

    println!("full name with a z: {}", name);

    name.truncate(5);

    println!("first name again: {}", name);

    let first_last_name = "James Jackson".to_string();
    let names: Vec<&str> = first_last_name.split(' ').collect();

    println!("names in a collection: {:?}", names);

    let first_name = names.first();

    if let Some(f_name) = first_name {
        println!("hey, I'm an existing first name!: {}", f_name);
    }

    let last_name = names.last();

    if let Some(l_name) = last_name {
        println!("hey, I'm an existing last name!: {}", l_name);
    }

    let spaces_around_name = "                James             ".to_string();
    let trimmed_name = spaces_around_name.trim();

    println!("trimmed name: {}", trimmed_name);

    let jimmy = first_last_name.replace("James", "Jimmy");

    println!("nick name instead: {}", jimmy);

    let lol = "#".repeat(70);

    println!("lulz: {}", lol);

    // String exersize

    let full_name = "James Brian Jackson".to_string();

    let initials = get_name_initials("James Brian Jackson");

    println!("My initials: {}", initials);

    let initials = get_name_initials(
        "Siddig El Tahir El Fadil El Siddig Abdurrahman Mohammed Ahmed Abdel Karim El Mahdi",
    );

    println!("My initials: {}", initials);

    let initials = get_name_initials(
        "                                  James     Brian         Jackson           ",
    );

    println!("My initials: {}", initials);

    let cases = [
        ("training", Some("aii".to_string())),
        ("beautiful", Some("eauiu".to_string())),
        ("today", Some("oa".to_string())),
        ("tsk", None),
    ];

    for (word, expected) in cases.iter() {
        let result = get_all_vowels(word);
        println!("expected: {:?}, result: {:?}", expected, result);
    }

    let vowels = get_all_vowels("training");

    println!("vowels: {:#?}", vowels);
}

//   "hello".to_string()     ...   "hello"[0..3]  =   &"hel"
// &str IS NOT THE SAME AS A String
// example: "James Brian Jackson"
fn get_name_initials(name: &str) -> String {
    let full_name = name.to_string().to_uppercase();

    let names_iter = full_name.split_whitespace();

    let mut initials = String::new();

    for name in names_iter {
        let mut s = String::from(name);
        s.truncate(1);
        s.push('.');
        initials.push_str(&s);
    }

    initials
}

// "training" -> Some("aii")
// "beautiful" -> Some("eauiu")
// "today" -> Some("oa")
// "tsk" -> None
fn get_all_vowels(word: &str) -> Option<String> {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut result = String::new();

    for c in word.chars() {
        for vowel in vowels.iter() {
            if &c == vowel {
                result.push(c);
            }
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

fn get_all_vowels_jimmy_solution(word: &str) -> Option<String> {
    let vowels = word.replace(|c| !matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'), "");

    if vowels.is_empty() {
        None
    } else {
        Some(vowels)
    }
}
