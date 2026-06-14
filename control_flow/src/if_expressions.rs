fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    zero_check(0)
}

// fn to check for zero
fn zero_check(x:i32) {
    if x == 0 {
        println!("Zero Alert!");
    }
    else {
        println!("Lets Go!");
    }
}