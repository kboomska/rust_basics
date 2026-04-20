use tests::{add_two, sum};

// Интеграционные тесты

#[test]
fn it_equal_ten() {
    let add_result = add_two(8);
    let return_result = sum(4, 6);
    assert_eq!(add_result, return_result);
}

// Подмодули в интеграционных тестах

mod common;

#[test]
fn it_adds_two() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}
