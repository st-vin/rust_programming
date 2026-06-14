fn main() {
    // optional type annotations
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup_one = (500, 6.4, 1);

    let (x, y, z) = tup_one;

    // tuple element access by index
    let one = tup.0;
    println!("{one}");

    // destructuring a tuple
    println!("The value of y is: {y} ");
}