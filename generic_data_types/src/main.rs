// Обобщённые типы данных в объявлении функций

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {result}");

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {result}");
// }

// Обобщённые типы данных в объявлении структур

struct Point<T> {
    x: T,
    y: T,
}
struct AnotherPoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 4, y: 10 };
    let float = Point { x: 1.0, y: 10.0 };
    // let wont_work = Point { x: 5, y: 4.0 }; // Ошибка!
    let wont_work = AnotherPoint { x: 5, y: 4.0 };
}

// Обобщённые типы данных в объявлении перечислений

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
