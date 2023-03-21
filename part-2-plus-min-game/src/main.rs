use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Please enter a number");

        let mut supposition = String::new();

        io::stdin()
            .read_line(&mut supposition)
            .expect("Cannot read user input...");

        let supposition: u32 = match supposition.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("Your number is {}", supposition);

        match supposition.cmp(&secret_number) {
            Ordering::Less => println!("It's more!"),
            Ordering::Greater => println!("It's less!"),
            Ordering::Equal => {
                println!("Finally.. you guessed it!");
                break;
            }
        }
    }
}
