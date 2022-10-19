use std::mem;

pub fn run() {
    // Arrays are fixed length same type in Rust
    let mut numbers = [0, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // print single value
    println!("First array element: {}", numbers[0]);

    // mutate a existing key (can't add or delete)
    numbers[0] = 1;
    println!("{:?}", numbers);

    // array length
    println!("array length: {}", numbers.len());

    // stack allocation size
    println!("array stack size: {}", mem::size_of_val(&numbers));

    // slice arrays
    println!("Array get first 2: {:?}", &numbers[0..2]);
}
