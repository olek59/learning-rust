use std::io::stdin;

fn what_is_your_name() -> String {
    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line from stdin.");
    your_name.trim().to_ascii_lowercase()
}

fn main() {
    println!("What is your name?");
    let name = what_is_your_name();
    println!("Hello, {}!", name);
}
