use std::collections::HashMap;

fn main() {
    // Создание новой хеш-карты.

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Доступ к данным в HashMap.

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Перебор каждой пары ключ/значение в HashMap.

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Хеш-карты и владение.

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{field_name}"); // Ошибка владения!

    // Обновление данных в HashMap.

    // Перезапись старых значений.

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}"); // {"Blue": 25}

    // Вставка значения только в том случае, когда ключ не имеет значения.

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}"); // {"Blue": 10, "Yellow": 50}

    // Создание нового значения на основе старого значения.

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}"); // {"hello": 1, "world": 2, "wonderful": 1}
}
