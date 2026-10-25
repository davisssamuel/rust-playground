use std::io;

const SCALE: [char; 4] = ['F', 'f', 'C', 'c'];

fn main() {
    let mut index: usize;
    let mut scale: char;

    println!("Enter a temperature:");

    let input_temp: f64 = loop {
        let mut input_temp = String::new();

        io::stdin()
            .read_line(&mut input_temp)
            .expect("Failed to read line");

        index = input_temp
            .find(SCALE)
            .expect("Input must contain a temperature scale");

        scale = input_temp.chars().nth(index).unwrap().to_ascii_uppercase();

        match input_temp[..index].trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Input must be a number"),
        };
    };

    let output_temp = match scale {
        'F' => convert_to_celcius(input_temp),
        'C' => convert_to_farenheit(input_temp),
        _ => {
            println!("Not a valid temperature sign");
            return;
        }
    };

    println!("{:.2}", output_temp);
}

fn convert_to_celcius(t: f64) -> f64 {
    let conversion_factor: f64 = 5.0 / 9.0;
    (t - 32.0) * conversion_factor
}

fn convert_to_farenheit(t: f64) -> f64 {
    let conversion_factor: f64 = 9.0 / 5.0;
    (t * conversion_factor) + 32.0
}
