use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap(); // Сначала дожидается завершения порожденного потока

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join().unwrap(); // Приводит к чередованию вывода

    // Использование move-замыканий в потоках

    let v = vec![1, 2, 3];

    // Замыкание забирает используемые значения во владение с помощью move
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
