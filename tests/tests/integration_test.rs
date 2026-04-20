use tests::{add_two, sum};

// Интеграционные тесты

#[test]
fn it_equal_ten() {
    let add_result = add_two(8);
    let return_result = sum(4, 6);
    assert_eq!(add_result, return_result);
}
