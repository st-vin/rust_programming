# Rust Ownership — Beginner Notes

## 1. What is Ownership?

**Ownership** is Rust's system for managing memory safely **without a garbage collector (GC)**.

Instead of:

* Garbage Collection (Java, Python)
* Manual memory management (C/C++)

Rust uses **ownership rules checked at compile time**.

### Ownership Rules

1. Every value has an **owner**.
2. A value can have **only one owner at a time**.
3. When the owner goes **out of scope**, the value is **dropped** (memory is freed).

---

## 2. Why Ownership Exists

Programs must manage memory.

Ownership helps Rust:

* Track who uses heap data
* Avoid memory leaks
* Prevent double frees
* Eliminate many memory safety bugs
* Maintain high performance

---

# 3. Stack vs Heap

Understanding ownership requires understanding memory.

## Stack

Characteristics:

* Last In First Out (LIFO)
* Fast
* Fixed-size data only
* Push and pop operations

### Example

```rust
let x = 5;
```

Memory:

```
STACK
+-----+
|  5  |
+-----+
```

---

## Heap

Characteristics:

* More flexible
* Stores data with unknown/changing size
* Requires allocation
* Accessed through pointers

### Example

```rust
let s = String::from("hello");
```

Memory:

```
STACK                    HEAP
+-----------+           +---+---+---+---+---+
| pointer ------->      | h | e | l | l | o |
| length=5 |            +---+---+---+---+---+
| cap=5    |
+-----------+
```

---

## Stack vs Heap Summary

| Stack                 | Heap             |
| --------------------- | ---------------- |
| Fast                  | Slower           |
| Fixed size            | Dynamic size     |
| Direct access         | Pointer access   |
| Automatically cleaned | Needs management |

Ownership mainly exists to manage **heap memory**.

---

# 4. Variable Scope

A variable is valid only within its scope.

```rust
{
    let s = "hello";

    // s is valid here
}

// s is no longer valid
```

### Lifecycle

```
Enter Scope
     |
Declare Variable
     |
Use Variable
     |
Exit Scope
     |
Variable Invalid
```

---

# 5. String Literals vs String

## String Literal

```rust
let s = "hello";
```

Properties:

* Hardcoded into executable
* Immutable
* Known at compile time
* Stored efficiently

---

## String

```rust
let s = String::from("hello");
```

Properties:

* Stored on heap
* Can grow
* Mutable
* Size can change at runtime

Example:

```rust
let mut s = String::from("hello");

s.push_str(", world!");

println!("{s}");
```

Output:

```text
hello, world!
```

---

# 6. Allocation and Drop

When a `String` is created:

```rust
let s = String::from("hello");
```

Rust:

1. Requests memory from heap.
2. Stores data there.

When scope ends:

```rust
{
    let s = String::from("hello");
}
```

Rust automatically calls:

```rust
drop(s);
```

Conceptually:

```
Create String
      |
Allocate Heap Memory
      |
Use String
      |
Leave Scope
      |
drop()
      |
Memory Freed
```

---

# 7. Move Semantics

This is one of the most important ownership concepts.

---

## Integers are Copied

```rust
let x = 5;
let y = x;

println!("{x}");
```

Works because integers are stored entirely on the stack.

```
x = 5
y = 5
```

Both remain valid.

---

## Strings are Moved

```rust
let s1 = String::from("hello");
let s2 = s1;
```

### Before Assignment

```
s1
 |
 v
"hello"
```

### After Assignment

```
s1  (invalid)

s2
 |
 v
"hello"
```

Ownership transfers from `s1` to `s2`.

---

## Using Moved Values

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{s1}");
```

Error:

```text
borrow of moved value: s1
```

Because:

```
s1 --> ownership moved --> s2
```

Only `s2` owns the data.

---

# 8. Why Move Exists

Imagine both variables owned the same heap memory:

```
s1 ----\
         ---> "hello"
s2 ----/
```

When scope ends:

```text
s1 frees memory
s2 frees memory again
```

This causes a:

### Double Free Error

A serious memory safety bug.

Rust prevents this by invalidating the old owner.

---

# 9. Clone for Deep Copies

If you actually want two independent Strings:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

Memory:

```
s1 --> "hello"
s2 --> "hello"
```

Each has its own heap allocation.

Both are valid:

```rust
println!("{s1}");
println!("{s2}");
```

### Important

`clone()` copies heap data and may be expensive.

Use it only when you need a true duplicate.

---

# 10. Reassignment and Drop

```rust
let mut s = String::from("hello");

s = String::from("ahoy");
```

What happens?

### Step 1

```
s --> "hello"
```

### Step 2

Assign new String:

```
s --> "ahoy"
```

The original `"hello"` has no owner.

Rust immediately:

```text
drop("hello")
```

Memory is freed.

---

# 11. Copy Trait

Some types are so cheap to copy that Rust automatically duplicates them.

Examples:

```rust
let x = 5;
let y = x;
```

Both remain valid.

---

## Types that implement Copy

* `i32`, `u32`, etc.
* `bool`
* `f64`
* `char`
* Tuples containing only Copy types

Examples:

```rust
let a = true;
let b = a;

println!("{a}");
```

Valid.

---

## Types that DO NOT implement Copy

```rust
String
```

Because it owns heap memory.

---

# 12. Ownership and Functions

Passing values to functions follows the same rules as assignment.

---

## Ownership Transfer

```rust
fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    // s no longer valid
}
```

Flow:

```
main owns s
      |
      v
takes_ownership(s)
      |
ownership moved
      |
function ends
      |
drop()
```

---

## Copy Types in Functions

```rust
fn makes_copy(x: i32) {
    println!("{x}");
}

fn main() {
    let x = 5;

    makes_copy(x);

    println!("{x}");
}
```

Works because `i32` implements `Copy`.

---

# 13. Ownership Through Return Values

Functions can transfer ownership back.

```rust
fn gives_ownership() -> String {
    let s = String::from("yours");
    s
}
```

Usage:

```rust
let s1 = gives_ownership();
```

Ownership moves:

```
Function
   |
returns String
   |
   v
s1 owns it
```

---

## Taking and Returning Ownership

```rust
fn takes_and_gives_back(s: String) -> String {
    s
}
```

Usage:

```rust
let s2 = String::from("hello");
let s3 = takes_and_gives_back(s2);
```

Flow:

```
s2
 |
 v
function
 |
 v
s3
```

`s2` becomes invalid.

---

# 14. Returning Ownership with Tuples

One way to return both a value and additional data:

```rust
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
```

Usage:

```rust
let s1 = String::from("hello");

let (s2, len) = calculate_length(s1);
```

Result:

```text
s2 = "hello"
len = 5
```

Ownership is returned alongside the computed value.

---

# Ownership Cheat Sheet

### Move

```rust
let s1 = String::from("hello");
let s2 = s1;
```

✅ Fast
❌ `s1` invalid

---

### Clone

```rust
let s2 = s1.clone();
```

✅ Both valid
❌ Copies heap data

---

### Copy

```rust
let x = 5;
let y = x;
```

✅ Both valid
✅ Cheap stack copy

---

### Drop

```rust
{
    let s = String::from("hello");
}
```

Scope ends:

```rust
drop(s);
```

called automatically.

---

# Mental Model

Think of ownership as a **single ownership certificate** attached to every heap value.

```
Value
  ^
  |
Owner
```

When ownership is moved:

```
Old Owner ----X

New Owner ----> Value
```

When the owner disappears:

```
Owner leaves scope
       |
       v
     drop()
       |
       v
Memory freed
```

This single idea explains:

* Scope
* Moves
* Cloning
* Function arguments
* Function returns
* Automatic memory cleanup

References (the next concept in Rust) are introduced to allow code to **use a value without taking ownership**.
