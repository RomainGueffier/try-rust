use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    println!("args {:?}", args);

    // type in terminal cargo run dev
    println!("Command: {}", command);

    if command == "greet" {
        println!("Hello Romain");
    } else {
        println!("Not a valid command bro' 🤨");
    }
}
