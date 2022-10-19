pub fn run() {
    // primitive strings are immutable fixed length in memory
    let _message = "Hello world";

    // Growable heap allocated data structure
    let mut text = String::from("Hello");

    // get length
    println!("{:?}", text.len());

    // mutate string
    // push a char
    text.push(' ');
    // push a string
    text.push_str("world");

    println!("Original string: {}", text);

    // capacity
    println!("string capacity: {}", text.capacity());
    // is empty ?
    println!("string is empty: {}", text.is_empty());
    // contains ?
    println!("string contains {}: {}", "toto", text.contains("toto"));
    // replace
    println!("string replacement: {}", text.replace("world", "Romain"));

    // loop over strings
    for word in text.split_whitespace() {
        println!("{}", word);
    }

    // assertion stop program when assert condition fails
    assert_eq!(11, text.len());
    assert_eq!(20, text.capacity());
}
