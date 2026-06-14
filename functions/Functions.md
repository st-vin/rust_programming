#  Functions in Rust: Crucial Concepts

## 1. Function Definition and Naming

*   **Keyword:** Functions are defined using the `fn` keyword.
    *   *Example:* `fn function_name(...) { ... }`
*   **Naming Convention:** Rust uses **snake_case** for function and variable names (all lowercase, words separated by underscores).
*   **Calling Functions:** Functions are called by using their name followed by parentheses: `function_name()`.
*   **Definition Order:** The order in which functions are defined does not matter, as long as they are defined within a visible scope.

## 2. Parameters and Arguments

*   **Parameters:** Special variables declared in the function signature.
*   **Arguments:** The concrete values passed to the function when it is called. (Terms are often used interchangeably.)
*   **Type Annotations (Crucial!):** You **must** declare the type of each parameter (e.g., `x: i32`). This is a deliberate design choice that helps the compiler and provides better error messages.
*   **Multiple Parameters:** Separate multiple parameters with commas:
    *   *Example:* `fn func(value: i32, unit: char)`

## 3. Statements vs. Expressions

This is a key distinction in Rust:

| Concept | Definition | Behavior | Semicolon (`;`) | Example |
| :--- | :--- | :--- | :--- | :--- |
| **Statement** | Instructions that perform an action. | Does **not** return a value. | Usually ends with `;`. | `let x = 5;` |
| **Expression** | Code that evaluates to a resultant value. | Evaluates to a value. | Does **not** end with `;`. | `5 + 6` or `my_func()` |

*   **Importance:** Rust emphasizes that functions must return a value, meaning the function body must consist primarily of **expressions**.

## 4. Functions with Return Values

*   **Declaring the Return Type:** Use an arrow (`->`) followed by the desired return type in the function signature.
    *   *Example:* `fn five() -> i32` (This function returns an integer.)
*   **The Return Value:** The function's return value is the result of the **final expression** in the function body.
*   **Return Value Requirement:** For a function to return a value, the body must contain an expression that evaluates to that value (i.e., **no trailing semicolon**).

###  Key Takeaway on Return Values:

*   **Correct (Expression):** `fn plus_one(x: i32) -> i32 { x + 1 }` (Returns the result of the calculation)
*   **Incorrect (Statement):** `fn plus_one(x: i32) -> i32 { x + 1; }` (Causes a compilation error because a statement does not return a value.)