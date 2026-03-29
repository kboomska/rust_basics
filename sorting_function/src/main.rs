use rand::Rng;

fn main() {
    let v = generate_vector();
    println!("Vector: {:?}", v);

    // Среднее значение списка.
    let mean = vector_mean(&v);
    println!("Mean: {mean}");
}

/// Генерация исходного вектора значений.
fn generate_vector() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let length = rng.gen_range(5..=10);
    let mut vector = vec![0; length];

    for index in 0..vector.len() {
        let value = rng.gen_range(1..=5);
        vector[index] = value;
    }

    vector
}

/// Вычисление среднего значения для вектора.
fn vector_mean(vector: &[i32]) -> f64 {
    let length = vector.len();

    // Сумма всех значений используя метод sum для Iter.
    let sum: i32 = vector.iter().sum();

    sum as f64 / length as f64
}
