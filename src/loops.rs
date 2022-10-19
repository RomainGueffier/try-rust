pub fn run() {
    let mut count = 0;

    // infinite loop
    loop {
        count += 1;
        println!("{}", count);
        if count == 10 {
            break;
        }
    }

    // while loop
    count = 1;
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }

    // for loop
    for x in 1..100 {
        if x % 15 == 0 {
            println!("tito");
        } else if x % 3 == 0 {
            println!("ti");
        } else if x % 5 == 0 {
            println!("to");
        } else {
            println!("{}", x);
        }
    }
}
