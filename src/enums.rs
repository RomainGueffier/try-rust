enum Movement {
    // variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // perform action depending on movement

    // match is like switch conditions
    match m {
        Movement::Down => println!("Avatar moving down"),
        Movement::Up => println!("Avatar moving up"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    };
}

pub fn run() {
    let avatar_christine = Movement::Left;
    let avatar_romain = Movement::Down;

    move_avatar(avatar_christine);
    move_avatar(avatar_romain);
    move_avatar(Movement::Right);
    move_avatar(Movement::Up);
}
