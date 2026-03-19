// lib.rs - корневой модуль библиотечного крейта.

// Содержимое любого из файлов main.rs или lib.rs образует неявный модуль
// crate в корне структуры модулей крейта, известной как дерево модулей.

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
