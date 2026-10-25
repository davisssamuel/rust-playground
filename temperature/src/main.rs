use std::io;

enum Scale {
    Fahrenheit,
    Celsius,
}

fn main() {
    println!("Enter a temperature:");

    let (temp, scale) = loop {
        let mut input_temp = String::new();

        io::stdin()
            .read_line(&mut input_temp)
            .expect("Failed to read line");

        let input_temp = input_temp.trim();

        let input_scale = match input_temp.chars().last() {
            Some('F' | 'f') => Scale::Fahrenheit,
            Some('C' | 'c') => Scale::Celsius,
            _ => {
                println!("Input must end with a scale (F or C)");
                continue
            }
        };

        match input_temp[..input_temp.len() - 1].trim().parse() {
            Ok(num) => break (num, input_scale),
            Err(_) => println!("Input must be a number"),
        };
    };

    match scale {
        Scale::Fahrenheit => println!("{:.2}C", convert_to_celsius(temp)),
        Scale::Celsius => println!("{:.2}F", convert_to_fahrenheit(temp)),
    }
}

fn convert_to_celsius(t: f64) -> f64 {
    (t - 32.0) * 5.0 / 9.0
}

fn convert_to_fahrenheit(t: f64) -> f64 {
    t * 9.0 / 5.0 + 32.0
}
