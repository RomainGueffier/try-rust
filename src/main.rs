mod arrays;
mod cli;
mod conditionals;
mod enums;
mod functions;
mod loops;
mod pointers;
mod print;
mod strings;
mod structs;
mod tuples;
mod types;
mod variables;
mod vectors;

fn main() {
    print::run();
    variables::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointers::run();
    structs::run();
    enums::run();
    cli::run();
}
