// lib.rs - корневой модуль библиотечного крейта.

// Содержимое любого из файлов main.rs или lib.rs образует неявный модуль
// crate в корне структуры модулей крейта, известной как дерево модулей.

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
