// Rust can infer the type from the value of the var, but you can precise the type. Then it can't be changed

pub fn run() {
    // Integers: u=> unsigned i=> integer {n}=> byte (i8, u16 etc...)
    // default is i32
    let _i = 1;

    // Floats: same as integers but with f
    // default is f64
    let _f = 2.3;

    // explicit type
    let _e: i8 = 5;

    // Find max size
    println!("Max i32 is {}", std::i32::MAX);
    println!("Min i8 is {}", std::i8::MAX);

    // Boolean
    let is_active: bool = true;

    println!("{:?}", (is_active));

    // Char (defined with single quotes, single unicode char)
    let a = 'A';
    let emoji = '😎';
    println!("{:?}", (a, emoji))
}
