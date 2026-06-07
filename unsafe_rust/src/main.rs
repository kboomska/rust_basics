use std::slice;

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

    // Вызов небезопасной функции или метода

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    // Создание безопасных абстракций вокруг небезопасного кода

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Самостоятельно реализуем split_at_mut функцию для значений типа i32
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);

    // Использование extern функций для вызова внешнего кода

    unsafe extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // Попытка реализации split_at_mut с использованием только безопасного Rust
    // (&mut values[..mid], &mut values[mid..])

    // Использование небезопасного кода в реализации функции split_at_mut
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
