// fn main() {
//     let x = 5;
//     let y = Box::new(x);

//     // Использование оператора разыменования с типом Box<i32>
//     assert_eq!(5, x);
//     assert_eq!(5, *y); // Разыменование - переход от указателя Box<T> к значению
// }

// Определение собственного умного указателя

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Трактование типа как ссылки реализуя типаж Deref
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // *(y.deref())

    // Неявные разыменованные приведения с функциями и методами

    let m = MyBox::new(String::from("Rust"));
    // Преобразование &MyBox<String> в &String, а затем в &str, вызывая deref
    hello(&m);

    // Без реализации разыменованного приведения ссылок в Rust код выглядел бы
    // иначе:
    // hello(&(*m)[..]);
}

// Как разыменованное приведение взаимодействует с изменяемостью

// Rust выполняет разыменованное приведение, когда находит типы и реализации
// типажей в трёх случаях:

// Из типа &T в тип &U когда верно T: Deref<Target=U>
// Из типа &mut T в тип &mut U когда верно T: DerefMut<Target=U>
// Из типа &mut T в тип &U когда верно T: Deref<Target=U>
