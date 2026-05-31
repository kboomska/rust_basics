fn main() {
    // Создание сырых указателей

    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    // Создание сырого указателя на произвольный адрес памяти
    let address = 0x012345usize;
    let r = address as *const i32;

    // Разыменование сырых указателей в блоке unsafe
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
