// lib.rs - корневой модуль библиотечного крейта.

// Содержимое любого из файлов main.rs или lib.rs образует неявный модуль
// crate в корне структуры модулей крейта, известной как дерево модулей.

// Использование вложенных путей для уменьшения длинных списков use.

// use std::{cmp::Ordering, io};

use std::io::{self, Write};

// Оператор * (glob).

use std::collections::*;

// Подключение путей в область видимости с помощью ключевого слова use.

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// mod customer {
//     pub fn eat_at_restaurant() {
//         // Ошибка из-за отличия области видимости.
//         hosting::add_to_waitlist();
//     }
// }

mod customer {
    use super::hosting; // Добавили hosting в текущую область видимости.

    pub fn eat_at_restaurant() {
        // Ошибка из-за отличия области видимости.
        hosting::add_to_waitlist();
    }
}

// Создание идиоматических путей с use.

use std::collections::HashMap; // Указали путь напрямую к структуре.

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// Ошибка из-за одинаковых названий.

// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// Предоставление новых имён с помощью ключевого слова as.

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

// Реэкспорт имён с pub use.

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// Общедоступные структуры и перечисления.

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }

//     // Все варианты публичного перечисления также публичны.
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant() {
//     // Order a breakfast in the summer with Rye toast.
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // Change our mind about what bread we'd like.
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // The next line won't compile if we uncomment it; we're not allowed
//     // to see or modify the seasonal fruit that comes with the meal.
//     // meal.seasonal_fruit = String::from("blueberries");

//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }

// mod front_of_house {
//     // Публичный модуль.
//     pub mod hosting {
//         // Публичный метод.
//         pub fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     // Модуль приватный (по умолчанию).
//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// // Два способа вызова функции add_to_waitlist.
// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

// // Относительный путь с использованием super.

// fn deliver_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }

//     fn cook_order() {}
// }
