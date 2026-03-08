fn main() {
    // Повторное выполнение кода с помощью циклов.

    // Повторение выполнения кода с помощью loop.
    // loop {
    //     println!("again!");
    // }

    // Выполнение цикла можно прервать с помощью команды break.
    // Пропустить оставшийся код и перейти к новой итерации цикла можно
    // с помощью команды continue.

    // Возвращение значений из циклов.

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Метки циклов для устранения неоднозначности между несколькими циклами
    // Если у вас есть циклы внутри циклов, break и continue применяются к
    // самому внутреннему циклу в этой цепочке. При желании вы можете создать
    // метку цикла, которую вы затем сможете использовать с break или continue
    // для указания, что эти ключевые слова применяются к помеченному циклу,
    // а не к самому внутреннему циклу. Метки цикла должны начинаться с
    // одинарной кавычки.

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // Циклы с условием while.

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Цикл по элементам коллекции с помощью for.

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {element}");
    }

    // Пример обратного отсчета с помощью for.

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
