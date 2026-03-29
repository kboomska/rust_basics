use std::collections::HashMap;

use rand::Rng;

fn main() {
    let v = generate_vector();
    println!("Vector: {v:?}");

    // Среднее значение вектора.
    let mean = vector_mean(&v);
    println!("Mean: {mean}");

    // Медианное значение вектора.
    let median = vector_median(&v);
    println!("Median: {median}");

    // Мода вектора.
    let mode = vector_mode(&v);
    println!("Mode: {mode}");
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

/// Вычисление медианы для вектора.
fn vector_median(vector: &[i32]) -> i32 {
    let mut v = vector.to_vec(); // Клонирование вектора.
    v.sort();
    v[v.len() / 2]
}

/// Вычисление моды вектора.
fn vector_mode(vector: &[i32]) -> i32 {
    let mut map = HashMap::new();
    let mut mode = vector[0];
    let mut biggest_count = 0;

    for value in vector {
        let count = map.entry(value).or_insert(0);
        *count += 1;
    }

    for (key, value) in map {
        if value > biggest_count {
            mode = *key;
            biggest_count = value;
        }
    }

    mode
}
