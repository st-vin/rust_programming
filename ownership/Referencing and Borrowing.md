# Rust References & Borrowing

## 1. Why References Exist

Without references, ownership must be transferred into functions and then returned.

Example:

```rust
let s1 = String::from("hello");

let (s2, len) = calculate_length(s1);
```

This is cumbersome because ownership moves.

**References allow a function to use a value without taking ownership.**

---

# 2. What is a Reference?

A **reference** is an address that points to data owned by another variable.

* Can access data
* Does not own data
* Cannot drop data
* Must always point to valid data

---

## Creating a Reference

```rust
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

Function:

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### Syntax

```rust
&s1
```

means:

> Create a reference to `s1`

```rust
&String
```

means:

> Parameter expects a reference to a String

---

## Memory Model

```text
s
 |
 v
s1
 |
 v
"hello"
```

Reference `s` points to owner `s1`.

Ownership remains with `s1`.

---

# 3. Borrowing

Creating a reference is called **borrowing**.

```rust
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

Flow:

```text
s1 owns "hello"
       |
       v
calculate_length borrows it
       |
       v
ownership never moves
```

After the function call:

```rust
println!("{s1}");
```

still works because ownership was never transferred.

---

# 4. References Do Not Drop Values

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

When `s` goes out of scope:

```text
Reference destroyed
```

But:

```text
String remains
```

because the reference never owned it.

---

# 5. Dereferencing

Reference operator:

```rust
&s
```

Dereference operator:

```rust
*s
```

| Operator | Meaning          |
| -------- | ---------------- |
| `&`      | Create reference |
| `*`      | Follow reference |

---

# 6. Immutable References

By default, references are immutable.

```rust
fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

❌ Does not compile.

Error reason:

```text
Cannot modify data through an immutable reference.
```

---

# 7. Mutable References

To modify borrowed data:

```rust
let mut s = String::from("hello");

change(&mut s);
```

Function:

```rust
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Output:

```text
hello, world
```

---

## Requirements

Both sides must be mutable:

### Variable

```rust
let mut s = String::from("hello");
```

### Reference

```rust
&mut s
```

### Parameter

```rust
fn change(s: &mut String)
```

---

# 8. Mutable Reference Rule

At any moment:

```text
ONE mutable reference
```

or

```text
MANY immutable references
```

Never both.

---

## Invalid: Two Mutable References

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

❌ Compile error

Reason:

```text
Two writers to same data.
```

Diagram:

```text
r1 ----\
         ---> s
r2 ----/
```

Rust forbids this.

---

# 9. Why This Restriction Exists

Rust prevents **data races** at compile time.

A data race occurs when:

1. Two or more pointers access same data.
2. At least one writes.
3. Access is not synchronized.

```text
Multiple access
      +
Write operation
      +
No synchronization
      =
Data Race
```

Rust refuses to compile such code.

---

# 10. Mutable References in Different Scopes

This is allowed:

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
}

let r2 = &mut s;
```

Why?

```text
r1 scope ends
      |
      v
r2 created
```

The mutable borrows do not overlap.

---

# 11. Multiple Immutable References

Allowed:

```rust
let s = String::from("hello");

let r1 = &s;
let r2 = &s;
```

Diagram:

```text
r1 ----\
         ---> s
r2 ----/
```

Safe because nobody can modify the data.

---

# 12. Mixing Mutable and Immutable References

Invalid:

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
let r3 = &mut s;
```

❌ Compile error

Reason:

```text
Readers exist while writer is created.
```

Rust does not allow:

```text
Immutable refs + Mutable ref
```

at the same time.

---

# 13. Reference Scope

A reference lives until its **last use**, not necessarily until the end of the block.

Valid:

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;

println!("{r1} {r2}");

let r3 = &mut s;

println!("{r3}");
```

Why?

```text
r1 used here
r2 used here
     |
     v
references dead

r3 created afterwards
```

No overlap exists.

---

# 14. Dangling References

A dangling reference points to memory that has already been freed.

Rust prevents this entirely.

---

## Invalid Example

```rust
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

Problem:

```text
Create s
   |
Return reference to s
   |
s dropped
   |
Reference points to freed memory
```

Diagram:

```text
Reference
    |
    v
 Freed Memory
```

Rust rejects this code.

---

# 15. Correct Solution

Return ownership instead:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

Flow:

```text
s
 |
 v
returned
 |
 v
caller owns String
```

No invalid memory access.

---

# Reference Rules (Memorize)

### Rule 1

At any time, you may have:

```text
ONE mutable reference
```

OR

```text
ANY NUMBER of immutable references
```

---

### Rule 2

You cannot have:

```text
Mutable + Immutable
```

references simultaneously to the same value.

---

### Rule 3

References must always be valid.

Rust prevents:

```text
Dangling references
```

at compile time.

---

# Mental Model

### Immutable Borrow

```text
Owner
  |
  v
Value

Reader
  |
  v
Value
```

Many readers allowed.

---

### Mutable Borrow

```text
Owner
  |
  v
Value

Writer
  |
  v
Value
```

Exactly one writer allowed.

---

### Borrowing Summary

| Action           | Ownership Moves? |
| ---------------- | ---------------- |
| `s`              | Yes              |
| `&s`             | No               |
| `&mut s`         | No               |
| Return `String`  | Yes              |
| Return `&String` | No               |

Core idea:

```text
Ownership = control

Reference = temporary access
```

A reference lets code use data while ownership remains with the original owner.
