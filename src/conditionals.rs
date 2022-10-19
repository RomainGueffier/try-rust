pub fn run() {
    let age: i8 = 29;
    let has_hairs = false;

    // IF ELSE
    if age >= 30 || !has_hairs {
        println!("Already in your thirties");
    } else {
        println!("Not yet in your thirties");
    }

    // short if
    let is_thirty = if age >= 30 { true } else { false };
    println!("Already in your thirties ? {}", is_thirty);
}
