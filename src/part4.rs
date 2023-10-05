fn main() {
    let x = 5;
    // i8
    // i16
    // i32
    // i64
    // i128
    println!("{}", x);
    // f32
    // f64

    // bool

    //char

    let tup: (i32, bool, char) = (1, true, 'd');
    println!("{}", tup.0);

    let mut arr: [i32; 6] = [1, 2, 3, 4, 5, 6];
    arr[4] = 10;
    println!("{}", arr[4]);
}