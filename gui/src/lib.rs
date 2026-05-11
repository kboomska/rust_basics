// Определение типажа Draw
pub trait Draw {
    fn draw(&self);
}

// Определение структуры Screen с полем components, которое является вектором
// типаж-объектов, которые реализуют типаж Draw
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// Реализация метода run у структуры Screen, который вызывает метод draw каждого
// компонента из вектора
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
