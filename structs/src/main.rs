fn main() {
    // Структура (struct) — это пользовательский тип данных, позволяющий
    // назвать и упаковать вместе несколько связанных значений, составляющих
    // значимую логическую группу.

    // Создание экземпляра структуры User.

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Изменять значение полей можно только для изменяемой структуры.
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("sometwo@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user1 = build_user(
        String::from("somethree@example.com"),
        String::from("someusername123"),
    );

    // Создание экземпляра структуры из экземпляра другой структуры с помощью
    // синтаксиса обновления структуры.

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // Синтаксис обновления структуры использует = как присваивание.
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Больше не можем использовать user1, т.к. его поле String username было
    // перемещено в user2.

    // println!("{}", user1.username); // Ошибка!
    println!("{}", user1.active); // Нет ошибки.

    // Кортежные структуры: структуры без именованных полей для создания
    // разных типов.

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Единично-подобные структуры: структуры без полей.

    struct AlwaysEqual;

    let subject = AlwaysEqual;
}

// Выражение создания структуры может использоваться для возврата из функции
// нового экземпляра.
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }

// Использование сокращённой инициализации поля.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Пример структуры для хранения информации об учетной записи пользователя.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
