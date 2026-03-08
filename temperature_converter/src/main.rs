fn main() {
    let f_temp = 32.0;
    let c_temp = f_to_c(f_temp);

    print!("{f_temp} градусов по Фаренгейту равно");
    println!(" {c_temp} градусов по Цельсию");
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}
