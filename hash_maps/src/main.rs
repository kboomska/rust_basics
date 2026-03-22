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
}
