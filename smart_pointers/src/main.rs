// fn main() {
//     // Хранение значения i32 в куче
//     let b = Box::new(5);
//     println!("b = {b}");
// }

// Включение рекурсивных типов с помощью Boxes

// enum List {
//     Cons(i32, List), // Рекурсивное определение вызывает ошибку.
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let list = Cons(1, Cons(2, Cons(3, Nil)));
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
