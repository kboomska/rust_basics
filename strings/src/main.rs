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
}
