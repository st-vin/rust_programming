### **1. Project Setup & Cargo**

Cargo is Rust’s build system and package manager.

* **Create Project:** `cargo new project_name`
* **Compile & Run:** `cargo run` (rapidly iterates by compiling and executing in one step).
* **Compile Only:** `cargo build`
* **Dependencies:** Managed in `Cargo.toml` under `[dependencies]`.
* **Reproducible Builds:** Cargo creates a `Cargo.lock` file on the first build, locking dependency versions. Use `cargo update` to upgrade versions within your specified SemVer bounds.

### **2. Variables & Mutability**

Variables in Rust are immutable by default.

* **Immutable:** `let apples = 5;` (Cannot be changed once bound).
* **Mutable:** `let mut bananas = 5;` (Can be modified).
* **Associated Functions:** Functions implemented on a type rather than an instance.
* *Example:* `String::new()` creates a new, growable, UTF-8 encoded string.



### **3. Standard I/O & References**

By default, Rust brings a small set of items into scope called the *prelude*. Anything else must be explicitly imported.

* **Importing IO:** `use std::io;`
* **Receiving Input:**
```rust
let mut guess = String::new();
io::stdin().read_line(&mut guess);

```


* **References (`&`):** Allow multiple parts of code to access data without copying memory. Like variables, references are immutable by default. `&mut guess` explicitly makes the reference mutable so `read_line` can append to the string.
* **Printing Placeholders:** Use `{}` to inject variables or expressions into `println!`.
* *Example:* `println!("You guessed: {guess}");`



### **4. Error Handling (`Result` Enum)**

Methods like `read_line` and `parse` return a `Result` enum, which encodes error-handling information.

* **Variants:** `Ok` (contains the successful value) and `Err` (contains failure info).
* **`.expect()`:** A method on the `Result` type.
* If `Err`: Crashes the program and displays the provided message.
* If `Ok`: Unwraps and returns the successful value.
* *Note:* The compiler will issue a warning if a `Result` is ignored.



### **5. External Crates & Randomness**

Crates are packages of Rust code (binary = executable; library = meant for use in other programs).

* **Adding a Crate:** Add `rand = "0.8.5"` to `Cargo.toml`.
* **Using Traits:** A trait defines shared behavior. You must bring a trait into scope to use its methods.
```rust
use rand::Rng; // Brings the Rng trait into scope

// Generates an inclusive number between 1 and 100
let secret_number = rand::thread_rng().gen_range(1..=100); 

```

### **6. Shadowing & Type Conversion**

Rust allows you to declare a new variable with the same name as a previous variable. This is called **shadowing** and is frequently used for type conversions without needing to invent new variable names (like `guess_str`).

* **Type Inference:** Rust infers types where possible, but exact types must be annotated when ambiguity exists (e.g., `let guess: u32`).
* **Parsing Input:**
```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");

```


* `.trim()`: Removes leading/trailing whitespace and newline characters (`\n` or `\r\n`) generated when the user hits "Enter".
* `.parse()`: Converts the string into the specified numerical type (`u32` = unsigned 32-bit integer).



### **7. Control Flow: Comparisons (`match`)**

The `match` expression is used to check a value against a series of patterns (called *arms*).

* **The `Ordering` Enum:** Contains variants `Less`, `Greater`, and `Equal`.
* **Comparing Values:**
```rust
use std::cmp::Ordering;

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}

```


*Rust executes the code in the first arm that successfully matches.*

### **8. Control Flow: Looping**

The `loop` keyword executes a block of code infinitely. It is ideal for giving the user multiple chances to guess until a break condition is met.

```rust
loop {
    println!("Please input your guess.");
    // Input processing and match logic goes here...
}

```