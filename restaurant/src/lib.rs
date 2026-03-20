// lib.rs - корневой модуль библиотечного крейта.

// Содержимое любого из файлов main.rs или lib.rs образует неявный модуль
// crate в корне структуры модулей крейта, известной как дерево модулей.

mod front_of_house {
    // Публичный модуль.
    pub mod hosting {
        // Публичный метод.
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    // Модуль приватный (по умолчанию).
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// Два способа вызова функции add_to_waitlist.
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// Относительный путь с использованием super.

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
