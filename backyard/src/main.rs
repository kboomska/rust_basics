// main.rs - корневой модуль бинарного крейта.

// Создание псевдонима для пути к типу Asparagus в подмодуле vegetables
// модуля garden.
use crate::garden::vegetables::Asparagus;

// Подключение общедоступного модуля garden.
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
