use std::io;

fn main() {
    let mut input = String::new();

    println!("Please type in a string");
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let mut input = input.trim().chars();
    let first = &input.next();

    if let None = first {
        println!("Invalid input")
    }

    let first = first.unwrap();

    let result : String;
    if is_vowel(&first) {
        result = format!("{}{}-hay", first, input.as_str());
    } else if is_consonant(&first) {
        result = format!("{}-{}ay", input.as_str(), first);
    } else {
        result = format!("{}{}", first, input.as_str());
    }

    println!("{}", result);
}

fn is_vowel(c: &char) -> bool {
    let lower_vowels = ['a', 'e', 'i', 'o', 'u'];
    let upper_vowels = ['A', 'E', 'I', 'O', 'U'];

    lower_vowels.contains(c) || upper_vowels.contains(c)
}

fn is_consonant(c: &char) -> bool {
    ((*c >= 'a' && *c <= 'z') || (*c >= 'A' && *c <= 'Z')) && !is_vowel(c)
}
