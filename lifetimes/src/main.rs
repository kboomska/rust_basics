// Валидация ссылок при помощи времён жизни

// fn main() {

//     let r;

//     {
//         let x = 5;
//         r = &x; // Времена жизни предотвращают появление "повисших" ссылок.
//     }

//     println!("r: {r}");
// }

// Анализатор заимствований

// fn main() {
//     let r;                // ---------+-- 'a
//                           //          |
//     {                     //          |
//         let x = 5;        // -+-- 'b  |
//         r = &x;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {r}");   //          |
// }                         // ---------+

// fn main() {
//     let x = 5;            // ----------+-- 'b
//                           //           |
//     let r = &x;           // --+-- 'a  |
//                           //   |       |
//     println!("r: {r}");   //   |       |
//                           // --+       |
// }                         // ----------+

// Обобщённые времена жизни в функциях

// fn main() {
//     let string1 = String::from("long string is long");
//     {
//         let string2 = "xyz";

//         let result = longest(string1.as_str(), string2);
//         println!("The longest string is {result}");
//     }
// }

// Аннотации времени жизни в сигнатурах функций

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// При возврате ссылки из функции, параметр времени жизни для возвращаемого типа
// должен соответствовать параметру времени жизни одного из аргументов.

// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str() // Ошибка!
// }

// Определение времён жизни при объявлении структур

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// Шаблоны, запрограммированные в анализаторе ссылок языка Rust, называются
// правилами неявного выведения времени жизни.

// Времена жизни параметров функции или метода называются временем жизни ввода,
// а времена жизни возвращаемых значений называются временем жизни вывода.

// Первое правило заключается в том, что каждый параметр являющийся ссылкой,
// получает свой собственный параметр времени жизни.

// Второе правило говорит, что если есть ровно один входной параметр времени
// жизни, то его время жизни назначается всем выходным параметрам.

// Третье правило о том, что если есть множество входных параметров времени
// жизни, но один из них является ссылкой &self или &mut self, так как эта
// функция является методом, то время жизни self назначается временем жизни всем
// выходным параметрам.

// Аннотация времён жизни в определении методов

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

// Статическое время жизни

// Данная ссылка может жить всю продолжительность работы программы.
// let s: &'static str = "I have a static lifetime.";

// Обобщённые типы параметров, ограничения типажей и времена жизни вместе

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
