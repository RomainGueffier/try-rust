// Pointer and refs (memory access)

pub fn run() {
    // Primitive array
    let numbers = [1, 2, 3];
    let numbers_copy = numbers;

    println!("Primitive copy: {:?}", (numbers, numbers_copy));

    // with vector / non primitive when copy the first we can only copy the reference, not the value!!
    let chars = vec!['a', 'b'];
    let chars_copy = &chars; // use & for referencing, now chars is a reference, the value is in the copy

    println!(
        "Non primitive copy with & sign reference: {:?}",
        (&chars, chars_copy)
    );
}
