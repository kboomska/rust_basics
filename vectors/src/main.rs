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
}
