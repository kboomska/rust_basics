// Хранение закодированного текста UTF-8 в строках.

fn main() {
    // Создание новой строки.

    let mut s = String::new();

    // Создание строки из строкового литерала.

    let data = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();

    // Создание строки используя метод from().

    let s = String::from("initial contents");

    // Обновление строковых данных.

    // Добавление к строке нового строкового среза.

    let mut s = String::from("foo");

    // Метод принимает строковый срез, чтобы не владеть входным параметром.
    s.push_str("bar");

    // Метод push() принимает один символ в качестве параметра.
    let mut s = String::from("lo");
    s.push('l');

    // Объединение строк с помощью оператора + или макроса format!

    // Важно! s1 будет перемещен и не сможет использоваться после создания s3.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // fn add(self, s: &str)

    // Для более сложного комбинирования строк используется макрос format!

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // Макрос format! использует ссылки и не забирает во владение ни один из
    // параметров.
    let s = format!("{s1}-{s2}-{s3}");

    // Индексирование в строках.

    let s1 = String::from("hi");
    // let h = s1[0]; // Ошибка! Индексация не поддерживается.

    let hello = String::from("Hola");
    let length = hello.len();
    println!("{length}"); // 4 байт (один символ - один байт)

    let hello = String::from("Здравствуйте");
    let length = hello.len();
    println!("{length}"); // 24 байта (один символ - два байта)

    // Существует три способа рассмотрения строк с точки зрения Rust: как байты,
    // как скалярные значения и как кластеры графем (самая близкая вещь к тому,
    // что мы назвали бы буквами).

    // Срезы строк.

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}"); // Зд

    // let s = &hello[0..1];
    // println!("{s}"); // Ошибка!

    // Методы для перебора строк.

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
