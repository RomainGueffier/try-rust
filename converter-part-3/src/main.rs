use std::io;

fn convert_celsius_into_fahrenheit(deg: f32) -> f32 {
    deg * 1.8 + 32.0
}

fn main() {
    println!("Hello, would you like to convert things?");

    loop {
        println!("Please enter a Celcius degree, and we will convert it into fahrenheit");

        let mut deg = String::new();

        io::stdin()
            .read_line(&mut deg)
            .expect("Cannot read user input...");

        let deg: f32 = match deg.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("{}°C gives {}°F", deg, convert_celsius_into_fahrenheit(deg));
        break;
    }
}
