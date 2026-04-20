pub fn add_two(a: u64) -> u64 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Проверка на равенство с помощью макросов assert_eq! и assert_ne!

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    // Создание сообщений об ошибках

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    //Проверка с помощью макроса should_panic

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
