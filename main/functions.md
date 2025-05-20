# 🧩 Understanding Functions in Rust

Functions are one of the most important building blocks in Rust. They allow you to break your code into reusable parts, organize logic clearly, and give names to specific operations or processes.

Every Rust program starts with a special function called `main()`, but you can — and should — define your own functions too.

## 1. Defining a Simple Function

Here’s the most basic example:

```rust
fn say_hello() {
    println!("Hello from a function!");
}
```

To run this function, you call it from `main()`:

```rust
fn main() {
    say_hello();
}
```

▶️ [Run in Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+say_hello%28%29+%7B%0A++++println%21%28%22Hello+from+a+function%21%22%29%3B%0A%7D%0A%0Afn+main%28%29+%7B%0A++++say_hello%28%29%3B%0A%7D)

## 2. Functions with Parameters

You can pass data into functions using **parameters**:

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

```rust
fn main() {
    greet("Alice");
}
```

▶️ [Run in Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+greet%28name%3A+%26str%29+%7B%0A++++println%21%28%22Hello%2C+%7B%7D%21%22%2C+name%29%3B%0A%7D%0A%0Afn+main%28%29+%7B%0A++++greet%28%22Alice%22%29%3B%0A%7D)

## 3. Functions That Return Values

Functions can also produce and return values:

```rust
fn square(n: i32) -> i32 {
    n * n
}
```

```rust
fn main() {
    let result = square(4);
    println!("4 squared is {}", result);
}
```

▶️ [Run in Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+square%28n%3A+i32%29+-%3E+i32+%7B%0A++++n+*+n%0A%7D%0A%0Afn+main%28%29+%7B%0A++++let+result+%3D+square%284%29%3B%0A++++println%21%28%224+squared+is+%7B%7D%22%2C+result%29%3B%0A%7D)

## 4. Notes on Return Types

* Rust functions return the last expression by default (no `;`)
* You can use `return` for clarity, but it’s optional in simple cases
* If your function returns a value, you must use `->` followed by the return type
* If it doesn't return anything, just leave out the `->` — Rust will assume you're returning “nothing,” written as `()` behind the scenes

## 5. Why Use Functions?

- To avoid repeating code
- To make your logic easier to read and test
- To isolate behavior for reuse across files

## 6. What's Next?

You've now learned how to define and use functions to organize your code and make it reusable. Great job!

Ready to put those skills to practice and explore how Rust handles numbers, calculations, and common mathematical tasks? Continue your journey by learning about **Teaching Mathematics in Rust** in [`math.md`](./math.md).