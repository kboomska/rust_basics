fn main() {
    let greeting: &str = "Hello";
    let some_string = String::from("Some string");
    println!("{some_string}");
    let mut age = 25; // Объявлена как изменяемая переменная.
    let name = "Alex";
    println!("{}, world!", greeting);
    println!("Name: {}, age: {}", name, age);
    age = 33;
    print!("Age changed to ");
    println!("{age}");
    const PI: f64 = 3.14159; // Константа.
    println!("PI = {}", PI);

    // Пример "затенения" - создание переменной с существующим именем.
    let num = 5;
    println!("num {}", num); // 5
    let num = "Десять"; // Новая переменная с тем же именем, но другим типом.
    println!("num {}", num); // Десять
    let num = num.len(); // Длина строки (в байтах).
    println!("num {}", num); // 12
}
