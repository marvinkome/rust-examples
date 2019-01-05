use std::io;

fn main() {
    let n = get_n();

    println!("Fibonacci sequence for length {}", n);
    for num in 0..=n {
        println!("{}", calculate_fib(num));
    }
}

fn calculate_fib(n: i32) -> i32 {
    if n < 2 {
        return n;
    } else {
        return calculate_fib(n - 2) + calculate_fib(n - 1);
    }
}

fn get_n() -> i32 {
    let mut temp;

    loop {
        println!("Please input length");

        temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: i32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break temp;
    }
}
