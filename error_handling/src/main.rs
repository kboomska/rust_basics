fn main() {
    // Неустранимые ошибки с макросом panic!

    // panic!("crash and burn");

    // Использование обратной трассировки panic!

    let v = vec![1, 2, 3];

    // v[99]; // Ошибка

    // Исправимые ошибки с Result

    use std::fs::{self, File};
    use std::io::{Error, ErrorKind, Read};

    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         },
    //         _ => {
    //             panic!("Problem opening the file: {error:?}");
    //         }
    //     },
    // };

    // Альтернативы использованию match с Result<T, E>

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Лаконичные способы обработки ошибок - unwrap и expect

    let greeting_file = File::open("hello.txt").unwrap();

    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // Проброс ошибок

    // fn read_username_from_file() -> Result<String, Error> {
    //     let username_file_result = File::open("hello.txt");

    //     let mut username_file = match username_file_result {
    //         Ok(file) => file,
    //         Err(e) => return Err(e),
    //     };

    //     let mut username = String::new();

    //     match username_file.read_to_string(&mut username) {
    //         Ok(_) => Ok(username),
    //         Err(e) => Err(e),
    //     }
    // }

    // Сокращение для проброса ошибок: оператор ?

    // fn read_username_from_file() -> Result<String, Error> {
    //     let mut username_file = File::open("hello.txt")?;
    //     let mut username = String::new();
    //     username_file.read_to_string(&mut username)?;
    //     Ok(username)
    // }

    // Сокращение кода за счет использования цепочки вызовов

    // fn read_username_from_file() -> Result<String, Error> {
    //     let mut username = String::new();
    //     File::open("hello.txt")?.read_to_string(&mut username)?;
    //     Ok(username)
    // }

    // Использование fs::read_to_string

    fn read_username_from_file() -> Result<String, Error> {
        fs::read_to_string("hello.txt")
    }
}
