fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
// Here’s an example of a constant declaration:
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("Inner scope x: {x}");

    }

    println!("Outer scope x: {x}");
}
