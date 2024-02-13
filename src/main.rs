fn main() {
    println!("What is your name?");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    println!("Hello, {}!", name);
}
