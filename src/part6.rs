use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("The input is: {}", input);
}