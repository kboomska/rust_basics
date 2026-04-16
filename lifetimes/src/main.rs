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

fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {result}");
    }
}

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
