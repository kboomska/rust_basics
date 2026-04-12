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

// struct Point<T> {
//     x: T,
//     y: T,
// }
// struct AnotherPoint<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let integer = Point { x: 4, y: 10 };
//     let float = Point { x: 1.0, y: 10.0 };
//     // let wont_work = Point { x: 5, y: 4.0 }; // Ошибка!
//     let wont_work = AnotherPoint { x: 5, y: 4.0 };
// }

// Обобщённые типы данных в объявлении перечислений

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Обобщённые типы данных в определении методов

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Определяем метод для конкретного типа f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// fn main() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x());
// }

struct SomePoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> SomePoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: SomePoint<X2, Y2>) -> SomePoint<X1, Y2> {
        SomePoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = SomePoint { x: 5, y: 10.4 };
    let p2 = SomePoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // p3.x = 5, p3.y = c
}
