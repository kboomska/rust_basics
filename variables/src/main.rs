fn main() {
    // Создаем изменяемую переменную путем добавления mut при её объявлении.
    let mut a = 5;
    println!("The value of x is: {}", a);
    a = 6;
    println!("The value of x is: {}", a);

    // Константы могут быть заданы только константным выражением,
    // но не результатом вычисленного во время выполнения программы значения.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);

    // Затенение переменной (shadowing).
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
    }
    println!("The value of x is: {x}"); // 6

    // Затенение с изменением типа переменной.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}")
}
