use std::io;

fn main() {
    println!("Enter a number:");

    let input_num: u32 = loop {
        let mut input_num = String::new();

        io::stdin()
            .read_line(&mut input_num)
            .expect("Failed to read line");

        match input_num.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Input must be nonnegative");
                continue
            },
        };
    };

    println!("{}", fib(input_num));
}

fn fib(n: u32) -> u32 {
    match n {
        0 => return 0,
        1 => return 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
