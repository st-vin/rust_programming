# Control Flow in Rust: Concise Notes

Control flow allows code execution to depend on conditions (branching) and to repeat code (looping). The primary constructs in Rust are **if expressions** and **loops**.

---

## 1. If Expressions (Conditional Branching)

An `if` expression allows code to execute based on whether a condition is true or false.

### Basic Structure
*   The condition must evaluate to a **boolean (`bool`)**.
*   Code blocks are defined using curly braces `{}`.
*   An optional `else` block provides an alternative path.

**Example:**
```rust
let number = 3;

if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
// Output: condition was true
```

### Handling Multiple Conditions (`else if`)
Use `else if` to check multiple, mutually exclusive conditions. Execution stops after the first true condition is met.

**Example:**
```rust
let number = 6;

if number % 4 == 0 {
    println!("divisible by 4");
} else if number % 3 == 0 {
    println!("divisible by 3"); // This block executes
} else {
    println!("not divisible by 4 or 3");
}
// Output: divisible by 3
```

### `if` in `let` Statements (Assignment)
The result of an `if` expression can be assigned directly to a variable.

**Example:**
```rust
let condition = true;
let number = if condition { 5 } else { 6 };
// number is 5
```
***Note on Types:*** The expressions in the `if` and `else` arms must result in the same type (type compatibility is required).

---

## 2. Repetition with Loops

Rust provides three main types of loops: `loop`, `while`, and `for`.

### A. The `loop` Keyword (Infinite Loop)
The `loop` keyword creates an execution block that runs indefinitely unless explicitly stopped.

**Stopping Loops:**
*   **`break`**: Exits the loop immediately.
*   **`continue`**: Skips the rest of the current iteration and moves to the next one.

**Example with `break`:**
```rust
loop {
    println!("again!");
    if counter == 10 {
        break; // Exits the loop
    }
}
```

**Returning Values from Loops:**
Use `break` with a value to exit the loop and return a result:
```rust
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2; // Returns 20
    }
};
// result is 20
```

**Loop Labels (Nested Loops):**
Labels (using `'label: loop { ... }`) allow `break` or `continue` to target specific, labeled loops, not just the innermost one.

### B. `while` Loop (Conditional Repetition)
The `while` loop executes a block of code **while** a condition is true. This is often preferred over a simple `loop` when the exit condition is clear.

**Example:**
```rust
let mut number = 3;
while number != 0 {
    println!("{number}!");
    number -= 1;
}
// Prints 3, 2, 1
```

### C. `for` Loop (Iteration over Collections)
The `for` loop is the most common and safest way to iterate over elements in a collection (like arrays or ranges). It eliminates manual index management, reducing the chance of bugs.

**Iterating over an Array:**
```rust
let a = [10, 20, 30];
for element in a {
    println!("Value: {}", element);
}
```

**Using Ranges for Iteration (Recommended):**
Ranges allow concise iteration over a sequence of numbers. Use `.rev()` to iterate in reverse.

**Example (Countdown using Range):**
```rust
for number in (1..4).rev() {
    println!("{number}!");
}
// Prints: 4!, 3!, 2!, 1!
```