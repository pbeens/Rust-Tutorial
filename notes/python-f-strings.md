Rust does not have a direct equivalent to Python’s f-strings, but it provides powerful and flexible formatting via the `format!` macro, which serves a similar purpose.

### Python f-string:

```python
name = "Alice"
age = 30
print(f"My name is {name} and I am {age} years old.")
```

### Equivalent in Rust using `format!`:

```rust
let name = "Alice";
let age = 30;
println!("My name is {} and I am {} years old.", name, age);
```

Or, with named arguments:

```rust
let name = "Alice";
let age = 30;
println!("My name is {name} and I am {age} years old.");
```

This named argument feature was stabilized in Rust **1.58** (January 2022), allowing Python-style clarity without positional juggling.

### Summary of Key Rust Alternatives to f-strings:

* `format!`: Returns a formatted `String`.
* `println!`, `write!`, `writeln!`: Output formatted strings directly.
* Named field formatting (since 1.58) allows `"{var}"` syntax similar to f-strings.

### Example with formatting:

```rust
let pi = 3.14159;
println!("Pi is approximately {:.2}", pi); // Output: Pi is approximately 3.14
```

Rust’s formatting macros are *compile-time checked*, providing better safety but less runtime flexibility than Python’s f-strings.

> **Note:** In this example, we use the `println!` macro to print the formatted string. However, Rust also supports `write!` and `writeln!` macros for writing to a buffer or file.