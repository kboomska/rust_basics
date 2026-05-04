// Rc<T>, умный указатель с подсчётом ссылок

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // Увеличение счетчика ссылок без глубокого копирования данных
    let c = Cons(4, Rc::clone(&a));
}
