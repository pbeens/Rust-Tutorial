# Handling User Input

## 1. Why Use an `input()` Helper in Rust?

Reading user input in Rust requires several steps — creating a mutable buffer, reading from `stdin()`, handling errors, and trimming newlines. This is more verbose than languages like Python, where you can simply write:

```python
name = input("What is your name? ")
```

To make Rust input more ergonomic and beginner-friendly, it's common to create a helper function like `input()`.

## 2. The Custom `input()` Function

Here’s a reusable `input()` function that mimics Python-style input:

```rust
use std::io::{self, Write};

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure the prompt prints before input

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}
```

## 3. Example Usage

```rust
fn main() {
    let name = input("What is your name? ");
    println!("Hello, {name}!");
}
```

This simplifies your main logic and avoids boilerplate code.

👉 [Run this example in the Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=use+std%3A%3Aio%3A%3A%7Bself%2C+Write%7D%3B%0A%0Afn+input%28prompt%3A+%26str%29+-%3E+String+%7B%0A++++print%21%28%22%7B%7D%22%2C+prompt%29%3B%0A++++io%3A%3Astdout%28%29.flush%28%29.unwrap%28%29%3B%0A%0A++++let+mut+buffer+%3D+String%3A%3Anew%28%29%3B%0A++++io%3A%3Astdin%28%29.read_line%28%26mut+buffer%29.unwrap%28%29%3B%0A++++buffer.trim%28%29.to_string%28%29%0A%7D%0A%0Afn+main%28%29+%7B%0A++++let+name+%3D+input%28%22What+is+your+name%3F+%22%29%3B%0A++++println%21%28%22Hello%2C+%7Bname%7D%21%22%29%3B%0A%7D)

## 4. Common Extensions

You can build more helpers:

```rust
fn input_int(prompt: &str) -> i32 {
    input(prompt).parse().expect("Expected a number")
}
```

Or a looped validator:

```rust
fn input_until_valid<F>(prompt: &str, validate: F) -> String
where
    F: Fn(&str) -> bool,
{
    loop {
        let value = input(prompt);
        if validate(&value) {
            return value;
        }
        println!("Invalid input. Try again.");
    }
}
```

## 5. When to Use This

Use `input()` helpers:
- When writing simple scripts or CLI tools
- When teaching Rust interactively
- When you want to minimize repeated boilerplate

Avoid in:
- Production systems where fine-grained error handling is critical
- Highly modular code where low-level control is required

## 6. Summary

Encapsulating input in a function keeps your code:
- Cleaner
- More readable
- Easier to teach and maintain

And it brings Rust a little closer to the ergonomic input experience found in languages like Python, JavaScript, and Go.
