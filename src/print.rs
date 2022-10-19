pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("{} is the best {} developer", "I'm", "Rust");

    // Positional arguments
    println!("{0} is 10 {1} with a reduction of 1 {1}", "Banana", "€");

    // Named arguments
    println!(
        "{name} is good at {sport}",
        name = "Romain",
        sport = "Running"
    );

    // placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 2, 2, 2);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
    println!("2 + 2 = {}", 2 + 2);
}
