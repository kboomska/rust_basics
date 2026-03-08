fn main() {
    let n: u32 = 6;

    fibonacci(n);
}

fn fibonacci(n: u32) {
    let mut previous: u32 = 0;
    let mut current: u32 = 1;
    let mut count: u32 = 1;

    if n == 0 {
        println!("{previous}");
    } else if n == 1 {
        println!("{current}");
    } else {
        while count != n {
            let temp: u32 = current;
            current += previous;
            previous = temp;
            count += 1;
        }
        println!("{current}");
    }
}
