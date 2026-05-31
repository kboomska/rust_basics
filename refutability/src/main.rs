fn main() {
    let some_option_value: Option<i32> = None;

    // let Some(x) = some_option_value; // Ошибка!

    let Some(x) = some_option_value else {
        return;
    };

    if let Some(x) = some_option_value {}

    let x = 5 else {
        return;
    };
}
