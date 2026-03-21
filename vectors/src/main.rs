fn main() {
    // Создание нового вектора.

    let v: Vec<i32> = Vec::new();

    // Создание вектора с помощью макроса vec! с начальными значениями.

    let v = vec![1, 2, 3];

    // Изменение вектора.

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Чтение данных вектора.

    let v = vec![1, 2, 3, 4, 5];

    // Обращение к значению по индексу.
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // Обращение к значению с помощью метода get.
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100]; // index out of bounds!
    let does_not_exist = v.get(100); // None

    // Пример нарушения правил заимствования, когда изменение вектора
    // может привести к изменению области занимаемой им памяти, после чего
    // созданная ранее ссылка на первый элемент будет указывать на
    // освобожденную память. Borrow checker сигнализирует о таких ошибках!

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6); // Ошибка!

    println!("The first element is: {first}");

    // Перебор значений в векторе.

    // Получение неизменяемых ссылок на каждый элемент в векторе
    // с использованием цикла for.

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // Изменение каждого элемента вектора через изменяемые ссылки.

    // Чтобы изменить значение на которое ссылается изменяемая ссылка,
    // мы должны использовать оператор разыменования ссылки `*` для получения
    // значения по ссылке в переменной i прежде чем использовать оператор `+=`.

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Использование перечислений для хранения множества разных типов.

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Удаление последнего элемента из вектора.

    let mut v = vec![100, 32, 57, 0];
    let zero = &v.pop();

    match zero {
        Some(last) => println!("{last}"),
        None => (),
    }

    for i in &v {
        println!("{i}");
    }

    // Удаление элементов из вектора.

    {
        let v = vec![1, 2, 3, 4];
    } // Освобождение памяти вектором и содержащимися в нем значениями.
}
