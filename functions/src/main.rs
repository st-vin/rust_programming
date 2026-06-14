fn main() {
    println!("Hello, world!");

    another_function(7);

    print_labeled_measurement(77, 'h');

    let x = plus_one(8);
    println!("{x}")
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
// multiple parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// functions with return values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}