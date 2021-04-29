fn main() {
    let result = to_pig_latin("crib");

    println!("{}", result);
}


/// Most words in Pig Latin end in "ay." Use the rules below to translate normal English into Pig Latin.
/// 1. If a word starts with a consonant, put the first letter of the word at the end of the word and add "ay."
/// Example: Happy = appyh + ay = appyhay
/// Example: String = ingstr + ay = ingstray
/// 3. If a word starts with a vowel add the word "way" at the end of the word.
/// Example: Awesome = Awesome +way = Awesomeway

// define a vowel
// check to see if word starts with vowel
// if it starts with vowel add WAY to the end
// if it does not start with vowel move first letter to the back
// check to see if next letter is vowel if so move it back
// add AY to the end
// "also, "training, crib" ,stream"
fn to_pig_latin(word: &str) -> String { // &str is NOT the same TYPE as a String

    let mut letters = word.chars().peekable();
    let mut pig_latin_word = String::from(word);

    match letters.peek().unwrap() {
        c if is_vowel(*c)  => {
            pig_latin_word.push_str("way")
        }
        _ => {

            while !is_vowel(*letters.peek().unwrap())  {
                let first_character = pig_latin_word.remove(0);
                pig_latin_word.push(first_character);
                letters.next();
            }

            pig_latin_word.push_str("ay");
        }
    }
    pig_latin_word
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}