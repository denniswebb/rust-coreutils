fn main() {
    let out = match std::env::args().nth(1) {
        Some(input) => input,
        None => String::from("y"),
    };

    loop {
        println!("{}", out);
    }
}
