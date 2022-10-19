use std::mem;

pub fn run() {
    // vectors are length mutable vectors
    let mut chars = vec!['z', 'b', 'c', 'd'];

    println!("{:?}", chars);

    // print single value
    println!("First vector element: {}", chars[0]);

    // mutate a existing key (can't add or delete)
    chars[0] = 'a';
    println!("{:?}", chars);

    // vector length
    println!("vector length: {}", chars.len());

    // stack allocation size
    println!("vector stack size: {}", mem::size_of_val(&chars));

    // slice vectors
    println!("vector get first 2: {:?}", &chars[0..2]);

    // Add on to vectors
    chars.push('e');
    println!("add a item: {:?}", &chars);

    // remove from vectors
    chars.pop();
    println!("remove a item: {:?}", &chars);

    // loop through vectors
    for letter in chars.iter() {
        println!("Letter in loop is: {}", letter);
    }

    // loop and mutate vector
    let mut numbers = [1, 2, 3];
    for x in numbers.iter_mut() {
        *x += 1
    }
    println!("Mutate vector over loop: {:?}", numbers);
}
