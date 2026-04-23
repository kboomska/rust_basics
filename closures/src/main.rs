use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Выведение и аннотация типов замыкания

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // Ошибка! Тип определен ранее String

    // Захват ссылок или передача владения

    // Захват неизменяемой ссылки
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // Захват изменяемой ссылки
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    // Неизменяемое заимствование недоступно, при наличии изменяемого
    // заимствования.
    // println!("Before calling closure: {list:?}"); Ошибка!
    borrows_mutably();
    println!("After calling closure: {list:?}");

    // Использование move для принуждения замыкания потока принять на себя
    // владение list
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // Перемещение захваченных значений из замыканий и трейты Fn

    // FnOnce применяется к замыканиям, которые могут быть вызваны один раз.
    // Все замыкания реализуют по крайней мере этот трейт, потому что все
    // замыкания могут быть вызваны. Замыкание, которое перемещает захваченные
    // значения из своего тела, реализует только FnOnce и ни один из других
    // признаков Fn, потому что оно может быть вызвано только один раз.

    // FnMut применяется к замыканиям, которые не перемещают захваченные
    // значения из своего тела, но могут изменять захваченные значения.
    // Такие замыкания могут вызываться более одного раза.

    // Fn применяется к замыканиям, которые не перемещают захваченные
    // значения из своего тела и не модифицируют захваченные значения,
    // а также к замыканиям, которые ничего не захватывают из своего окружения.
    // Такие замыкания могут выполняться более одного раза и не меняют ничего
    // в своём окружении, что важно в таких случаях, как одновременный вызов
    // замыкания несколько раз.

    // impl<T> Option<T> {
    //     pub fn unwrap_or_else<F>(self, f: F) -> T
    //     where
    //         F: FnOnce() -> T,
    //     {
    //         match self {
    //             Some(x) => x,
    //             None => f(),
    //         }
    //     }
    // }

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}
