fn main() {
    let x: i32 = 4;
    println!("x is: {}", x);

    {
        let x = x - 5;
        println!("x is: {}", x);
    }
    let x = x + 65;
    println!("x is: {}", x);

}
