use std::env::args;

fn main() {
    let out = match args().nth(1) {
        Some(input) => input,
        None => String::from("y"),
    };

    loop {
        println!("{}", out);
    }
}
