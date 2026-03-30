fn main() {
    let string = String::from("First apple");

    for word in string.split_whitespace() {
        println!("{}", translate_to_pig_latin(word));
    }
}

/// Метод перевода слов на Pig Latin.
fn translate_to_pig_latin(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let first_char = word.to_lowercase().chars().next().unwrap();
    let rest = &word[first_char.len_utf8()..];

    if vowels.contains(&first_char) {
        format!("{word}-hay")
    } else {
        format!("{rest}-{first_char}ay")
    }
}
