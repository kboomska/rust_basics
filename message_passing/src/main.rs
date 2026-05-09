// Передача данных с помощью сообщений между потоками

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // Передача владения
        // println!("val is {val}"); // Ошибка
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
