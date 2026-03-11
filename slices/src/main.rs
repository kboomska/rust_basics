fn main() {
    // Тип срезы.

    // Срезы позволяют ссылаться на непрерывную последовательность элементов
    // в коллекции, а не на всю коллекцию. Срез является своего рода ссылкой,
    // поэтому он не имеет права владения.

    // Нахождение первого слова в строке без использования срезов.

    let mut s = String::from("hello world");

    let word = first_word_example(&s); // 5

    s.clear(); // Пустая строка не связанная с полученным значением word.

    // Строковые срезы.

    // Строковый срез - это ссылка на часть строки String.
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // Синтаксис Rust `..`

    let s = String::from("hello");

    let slice = &s[0..2]; // he
    let slice = &s[..2]; // he

    let len = s.len();

    let slice = &s[3..len]; // lo
    let slice = &s[3..]; // lo

    let slice = &s[0..len]; // hello
    let slice = &s[..]; // hello

    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // Ошибка!

    println!("the first word is: {word}");

    // Строковые литералы - это срезы.
    let s: &str = "Hello, world!";

    let my_string = String::from("hello world");

    // `first_word` работает со срезами `String`, как частичными так и целыми.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` также работает с ссылкой на `String`, которая эквивалентна
    // целому срезу от `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` работает со срезами от строковых литералов (частичными и
    // целыми).
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Поскольку строковые литералы являются срезами, функция first_word
    // может работать с ними напрямую.
    let word = first_word(my_string_literal);

    // Срезы массива.

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

// Метод определения первого слова в строке.
fn first_word_example(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Метод определения первого слова в строке используя срезы.
// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
