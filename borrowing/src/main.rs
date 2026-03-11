fn main() {
    // Ссылки и заимствование.

    // Ссылка похожа на указатель в том смысле, что это адрес, по которому мы
    // можем проследовать, чтобы получить доступ к данным, хранящимся по
    // этому адресу.
    let s1 = String::from("hello");

    // &s1 создает ссылку, которая ссылается на значение s1, но не владеет им.
    // Процесс создания ссылки называется заимствованием.
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // Изменяемые ссылки.

    let mut s = String::from("hello");

    change(&mut s);

    println!("{s}"); // hello, world

    // Недопустимо создание более одной изменяемой ссылки на одно и то же
    // значение.

    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{r1}, {r2}");

    // Также не может быть изменяемой ссылки, пока есть неизменяемая ссылка на
    // то же значение.

    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{r1}, {r2}, and {r3}");

    // Область действия ссылки начинается с того места, где она была введена,
    // и продолжается до последнего использования этой ссылки.

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Переменные r1 и r2 больше не используются.

    let r3 = &mut s; // no problem
    println!("{r3}");
}

fn calculate_length(s: &String) -> usize {
    // Не допускается изменять значение по ссылке.
    // s.push_str(", world"); // Ошибка!
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
