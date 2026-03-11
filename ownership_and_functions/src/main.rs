fn main() {
    // Владение и функции.

    // Передача переменной в функцию приведёт к перемещению или копированию,
    // как и присваивание.
    let s = String::from("hello");

    takes_ownership(s); // Значение s перемещается в функцию.

    let x = 5;

    makes_copy(x); // Значение x копируется в функцию.

    // Возвращение значений и область видимости.

    // Перемещение возвращаемого значения в s1.
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    // Значение s2 перемещается в функцию, а затем возвращается из нее в s3.
    let s3 = takes_and_gives_back(s2);

    // Возвращение из функций нескольких значений используя кортеж.

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // Освобождение памяти some_string.

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string // Значение some_string возвращается из функции.
}

// Функция принимает строку и возвращает строку.
fn takes_and_gives_back(a_string: String) -> String {
    a_string // Значение a_string возвращается из функции.
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() возвращает длину String в байтах.

    (s, length)
}
