fn main() {
    println!("{}", "heyy world !!!");
    test_one();
    add_numbers(2, 44);

    let number = {
        let x = 3;
        x + 1 
    };
    println!("{}", number);
}

fn test_one() {
    println!("Test has been called...")
}

fn add_numbers(x: i32, y: i32) {
    println!("The sum is: {}", x + y);
}