pub fn run() {
    log("Logger is working");

    // Bind fn values to variables
    let sum = add(1, 1);
    println!("Addition fn result: {}", sum);

    // Closure (global scope aware fns)
    let n3: i8 = 10;
    let add_nums = |n1: i8, n2: i8| n1 + n2 + n3;
    println!("Closure addition: {}", add_nums(2, 2));
}

fn log(var: &str) {
    println!("{}", var)
}

// no semi colons at end means like return stmt
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
