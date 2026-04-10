fn main() {
    // Неустранимые ошибки с макросом panic!

    // panic!("crash and burn");

    // Использование обратной трассировки panic!

    let v = vec![1, 2, 3];

    // v[99]; // Ошибка

    // Исправимые ошибки с Result

    use std::fs::File;

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
