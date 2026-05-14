/// Структура записи в блоке.
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

/// Типаж состояния.
trait State {}

/// Структура состояния черновика.
struct Draft {}

impl State for Draft {}
