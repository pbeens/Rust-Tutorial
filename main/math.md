# Teaching Mathematics in Rust

This document is a hands-on guide to learning **mathematics using the Rust programming language**. Whether you're a beginner in Rust, mathematics, or both, this file is designed to walk you through essential math concepts by writing and running real code.

You’ll learn how to:

* Perform calculations like addition, division, and powers.
* Understand Rust’s type system, precision, and arithmetic behavior.
* Solve common math problems like calculating averages, working with percentages, and generating random numbers.
* Write reusable functions for mathematical tasks.

This file is structured progressively. Each section introduces new concepts with **executable examples**, highlights common mistakes, and shows best practices.

## 1. Why Use Rust for Learning Math?

Rust might seem like a systems language — and it is — but it also offers:

* **Strict type safety**, which prevents you from making silent math errors (unlike some scripting languages).
* **Clear error messages**, making it easier to understand why things go wrong.
* **Speed and reliability**, which are important when transitioning into more advanced math or simulations.
* **No runtime surprises**, thanks to Rust’s philosophy of making implicit behaviors explicit.

By learning math in Rust, you’re also learning how to write **correct, fast, and scalable code** — valuable skills for any domain, from engineering to finance to data science.

## 2. **Basic Arithmetic**

* Addition, Subtraction, Multiplication, Division, Remainder

```rust
let a = 10;
let b = 3;
println!("a + b = {}", a + b);
println!("a - b = {}", a - b);
println!("a * b = {}", a * b);
println!("a / b = {}", a / b);
println!("a % b = {}", a % b);
```

## 3. **Data Types for Math**

* Rust uses strictly typed numeric values, such as:

  * `i32`, `i64`: signed integers
  * `u32`, `u64`: unsigned integers
  * `f32`, `f64`: floating-point numbers
* Rust will not automatically convert between types (e.g., `i32` to `f64`) — you must do it explicitly.
* You can use `as` to cast between types.

### ✅ Correct Declarations

```rust
let int_val: i32 = 7;
let float_val: f64 = 7.5;
```

### ❌ Example of Type Mismatch

```rust
let int_val: i32 = 5;
let float_val: f64 = 2.5;

let result = int_val + float_val; // ⚠️ ERROR: mismatched types
```

### 💡 Error Message (abridged)

```text
error[E0277]: cannot add `f64` to `i32`
 --> src/main.rs:5:21
  |
5 |     let result = int_val + float_val;
  |                         ^ no implementation for `i32 + f64`
```

### ✅ Fixes Using Type Casting

#### 🠖 Cast Integer to Float

```rust
let result = int_val as f64 + float_val;
```

#### 🠖 Cast Float to Integer

```rust
let result = int_val + float_val as i32;
```

Be careful! Casting a float to an integer **truncates the decimal** (e.g., `2.9 → 2`).

## 🔢 Common Numeric Types in Rust

| Type  | Signed? | Size   | Range                            | Use Case                             |
| ----- | ------- | ------ | -------------------------------- | ------------------------------------ |
| `i8`  | Yes     | 8-bit  | −128 to 127                      | Very small values                    |
| `u8`  | No      | 8-bit  | 0 to 255                         | Byte manipulation                    |
| `i32` | Yes     | 32-bit | −2,147,483,648 to 2,147,483,647  | Default integer type                 |
| `u32` | No      | 32-bit | 0 to 4,294,967,295               | When values can't be negative        |
| `i64` | Yes     | 64-bit | Huge range of signed integers    | Large number computations            |
| `f32` | Yes     | 32-bit | \~7 decimal digits of precision  | Fast floating-point ops              |
| `f64` | Yes     | 64-bit | \~15 decimal digits of precision | Default float; high precision needed |

> 📝 Note: `i32` and `f64` are the default types when type inference is used.


## 4. **Floating Point Arithmetic & Precision**

* Differences between integer and float division
* Formatting output to control decimal places

## 5. **Order of Operations**

* Precedence (PEMDAS-like)
* Using parentheses

```rust
let result = (2 + 3) * 4;
```

## 6. **Variables and Mutability**

* `let` vs `let mut`
* Reassignment

## 7. **Math Functions**

* `f64::sqrt`, `f64::powf`, `f64::abs`
* Importing `std::f64::consts` for π, e

```rust
let area = std::f64::consts::PI * radius.powf(2.0);
```

## 8. **Random Numbers**

* Importing `rand` crate
* Generating random integers or floats

```rust
use rand::Rng;

let mut rng = rand::thread_rng();
let x: i32 = rng.gen_range(1..=10);
```

## 9. **User Input for Math**

* Using `std::io` to read numbers and do math interactively

## 10. **Control Flow with Math**

* `if` statements to check even/odd, divisibility
* `match` expressions for categories

## 11. **Loops for Math**

* Counting, factorials, summations

```rust
let mut sum = 0;
for i in 1..=10 {
    sum += i;
}
```

## 12. **Functions**

* Defining reusable math functions

```rust
fn square(n: i32) -> i32 {
    n * n
}
```

## 13. **Tuples and Structs for Multi-Value Results**

* Return `(mean, median)` from a function

## 14. **Math with Arrays and Vectors**

* Summing, averaging
* Using iterators: `.iter().sum()`

## 15. **Using External Crates**

* `num`, `rand`, `statrs`, `ndarray`
* Intro to plotting with `plotters`

## 16. **Mini Projects**

* Tip calculator
* Prime checker
* Quadratic formula
* Basic statistical summary

## 17. **Challenges**

* Exercises for the reader