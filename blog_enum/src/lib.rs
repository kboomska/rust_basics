/// Структура записи в блоге.
pub struct Post {
    state: Option<State>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(State::Draft),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if matches!(self.state, Some(State::Draft)) {
            self.content.push_str(text);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

/// Перечисление состояния.
enum State {
    Draft,
    PendingReview,
    Published,
}

impl State {
    fn request_review(self) -> Self {
        match self {
            State::Draft => State::PendingReview,
            other => other,
        }
    }
    fn approve(self) -> Self {
        match self {
            State::PendingReview => State::Published,
            other => other,
        }
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        match self {
            State::Published => &post.content,
            _ => "",
        }
    }
}
