// variables hold primitive data or references to data
// variables are immutable by default
// Rust is a block-scoped language
pub fn run() {
    let name = "Romain";
    let mut age = 19;

    println!("My name is {} I was {} years old", name, age);

    age = 29;

    println!("My name is {} I'm {} years old", name, age);

    // constants
    const ID: i8 = 02;
    println!("ID: {}", ID);

    // multiple assignement
    let (car, plane) = ("Toyota", "Airbus");
    println!("My car is a {} but my plane is a {}", car, plane);
}
