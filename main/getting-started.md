# Your First Steps: Output, Variables, and Input Basics

📘 Start with [`introduction.md`](./introduction.md) if you haven’t already. It explains how the tutorial is structured and how to get set up.

📘 For detailed setup instructions and how to run Rust code, see [`setup.md`](./setup.md).

## 1. Printing "Hello, world!"

`Hello, world!` is traditionally the first program you write when learning any new programming language. It’s a simple way to demonstrate the basic structure and syntax of the language. In Rust, every standalone program starts execution from a `main` function, which must always be present.

```
fn main() {
    println!("Hello, world!");
}
```
▶️ [Run in Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++println%21%28%22Hello%2C+world%21%22%29%3B%0D%0A%7D)

If you're curious, you can also explore how "Hello, world!" looks in other programming languages. [[https://en.wikipedia.org/wiki/%22Hello,_World!%22_program)]](https://en.wikipedia.org/wiki/%22Hello,_World!%22_program)

### ✏️ Exercise: Customize Your Greeting

Modify the program to print a personalized message instead of the default greeting. Try changing the string to greet yourself or someone else.

[Solve this exercise](../exercises/getting-started/01_customize_hello.rs)


## 2. Printing a String Stored in a Variable

🆕 This is the first time we're using a **variable** in Rust.

A variable allows you to assign a name to a value so it can be reused or referenced throughout your code. In Rust, variables are declared using the `let` keyword. For example:

```rust
let message = "Hello, world!";
```

* `let` declares a variable.
* `message` is the variable's name.
* `"Hello, world!"` is a **string literal** that gets stored in `message`.

Variables in Rust are **immutable by default**. This means once a value is assigned, it can't be changed unless you explicitly mark the variable as mutable using the `mut` keyword — which we don't need in this case.

You can then print the variable’s value with:

```rust
println!("{}", message);
```

Or, if you're using Rust 1.58 or newer, you can use the shorthand:

```rust
println!("{message}");
```

```rust
fn main() {
    let message = "Hello, world!";

    // Traditional style — compatible with all versions of Rust
    println!("{}", message);

    // Shorthand style — requires Rust 1.58 or newer
    println!("{message}");
}
```
▶️ [Run in Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++let+message+%3D+%22Hello%2C+world%21%22%3B%0D%0A%0D%0A++++%2F%2F+Traditional+style+%E2%80%94+compatible+with+all+versions+of+Rust%0D%0A++++println%21%28%22%7B%7D%22%2C+message%29%3B%0D%0A%0D%0A++++%2F%2F+Shorthand+style+%E2%80%94+requires+Rust+1.58+or+newer%0D%0A++++println%21%28%22%7Bmessage%7D%22%29%3B%0D%0A%7D)

🆕 This is the first example in the guide that uses **inline comments** (starting with `//`) to annotate the code. Comments will be used throughout the guide to explain syntax choices, version compatibility, and behaviors directly within the code samples.

### ✏️ Exercise: Store and Print a Custom Message

Update the `message` variable to contain a phrase of your choice. Then use both the traditional (`{}`) and shorthand (`{message}`) print styles to display it.

[Solve this exercise](../exercises/getting-started/02_variable_greeting.rs)

## 3. Asking for Your Name and Printing It

🆕 This is the first time we’re introducing **user input** in Rust.

Up until now, the programs you’ve written have only displayed messages. In this section, you’ll learn how to prompt the user to enter information and respond to what they type. Rust uses tools from its standard library to read input from the keyboard. We’ll walk through each part of this process — creating a string to store the input, reading from the keyboard, and printing a response.

```rust
use std::io;

fn main() {
    println!("What is your name?");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim();

    println!("Hello, {}!", name);
}
```
▶️ [Run in Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=407a04b6da4238c4068ae063af793a90)

🆕 This is the first time we’ve used an **import** in Rust, which is done using the `use` keyword:

```rust
use std::io;
```

This line tells Rust that we want to use the `io` (input/output) module from the `std` (standard library) namespace.

The `::` symbol is Rust’s **path separator**, similar to a dot (`.`) in some other languages. It lets us access items that are organized inside modules. Here, `std::io` means “the `io` module inside the `std` library.”

🆕 This is the first time we’ve collected **user input from the command line** in Rust.

By importing the `io` module from the standard library using `use std::io;`, we gain access to tools like `io::stdin()` — which lets us read input typed by the user during program execution.

If you're coming from Python, you might expect something as simple as:

```python
name = input("What is your name? ")
```

In Rust, the equivalent requires a few more steps — creating a mutable string buffer, reading into it, and trimming the newline. This is because Rust emphasizes **safety and explicit control**. Every potential failure (like I/O errors) must be handled, and memory ownership rules prevent unintended behaviors. While it may seem verbose at first, this approach leads to more predictable and robust code — especially as programs grow in complexity.

Here’s what each line of the Rust input code does:

```rust
let mut name = String::new();
```
Creates a new, empty `String` called `name`. It must be **mutable** (`mut`) because we'll modify it when we read the input.

```rust
io::stdin()
    .read_line(&mut name)
    .expect("Failed to read line");
```
- `io::stdin()` accesses the standard input stream.
- `.read_line(&mut name)` reads a full line of input and stores it in `name`. It also appends a newline (`\n`).
- `.expect(...)` handles any potential I/O errors by crashing the program with a clear message if something goes wrong.

```rust
println!("Hello, {}!", name.trim());
```
- `println!` prints formatted output to the terminal.
- `name.trim()` removes any whitespace from the start and end of the input — especially the trailing newline character from `read_line`.

Altogether, this block safely and explicitly captures user input and displays a greeting.

### ✏️ Exercise: Prompt and Personalize

Ask for the user's name, then print a custom sentence like "It's great to meet you, Alice!". Use `.trim()` to clean the input.

[Solve this exercise](../exercises/getting-started/03_prompt_personalized.rs)


## 4. Asking for First and Last Name and Printing Them

In this example, we prompt the user to enter their first and last name separately and store each input in its own variable.

```rust
use std::io;

fn main() {
    let mut fname = String::new();
    let mut lname = String::new();

    println!("Enter your first name:");
    io::stdin()
        .read_line(&mut fname)
        .expect("Failed to read line");

    println!("Enter your last name:");
    io::stdin()
        .read_line(&mut lname)
        .expect("Failed to read line");

    println!("Hello, {} {}!", fname.trim(), lname.trim());
}
```

🆕 This is the first time we're inserting multiple values into a single output using **multiple placeholders** in the `println!` macro. Each `{}` acts as a slot for a value. The values are passed as arguments to the macro in order:

```rust
println!("Hello, {} {}!", fname.trim(), lname.trim());
```

The `.trim()` method removes the newline characters included by `read_line()` so that the names display cleanly.

▶️ [Run in Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=e796a9bc9e5735d128185449db6f1151)

### ✏️ Exercise: Full Name Flex

Extend the example to ask for a middle name, or experiment with printing the name in last-name-first format (e.g., "Smith, John").

[Solve this exercise](../exercises/getting-started/04_full_name_flex.rs)

## 5. Formatting Numbers and Strings

Rust allows you to control the formatting of values using format specifiers:

```rust
let pi = 3.14159;
println!("Pi to two decimals: {:.2}", pi);
```

This is equivalent to Python’s `f"{pi:.2f}"`.

▶️ [Run in Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=c0d0eabc9177e51c62c82d448da8d877)

## 6. Using `format!` to Store Formatted Strings

The `format!` macro creates a `String` instead of printing it:

```rust
let name = "Alice";
let greeting = format!("Hello, {name}!");
println!("{}", greeting);
```

This is useful when you need to build strings dynamically or return them from functions.

▶️ [Run in Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0A++++let+name+%3D+%22Alice%22%3B%0A++++let+greeting+%3D+format%21%28%22Hello%2C+%7Bname%7D%21%22%29%3B%0A++++println%21%28%22%7B%7D%22%2C+greeting%29%3B%0A%7D)

### ✏️ Exercise: Save the Message

Use `format!` to build a greeting string and store it in a variable. Then print that variable using `println!`.

[Solve this exercise](../exercises/getting-started/05_format_variable.rs)


## 7. Output Macros: Summary

Rust provides four key formatting macros:

| Macro       | Description                              |
|-------------|------------------------------------------|
| `println!`  | Prints to stdout with newline            |
| `print!`    | Prints to stdout without newline         |
| `format!`   | Returns a formatted `String`             |
| `write!`    | Writes to a buffer implementing `Write`  |
| `writeln!`  | Like `write!`, but adds a newline        |

> `write!` and `writeln!` are used for writing to buffers or files instead of the screen. They're similar in style to `print!` and `println!` but require importing `std::fmt::Write` or `std::io::Write`.

## 8. Rust Formatting vs. Python f-strings: Cheat Sheet

| Purpose                        | Python f-string         | Rust format syntax               |
|-------------------------------|--------------------------|----------------------------------|
| Insert variable               | `{name}`                | `{}` or `{name}`                 |
| Format float to 2 decimals    | `{pi:.2f}`              | `{:.2}`                          |
| Left-align in 10 spaces       | `{name:<10}`            | `{:<10}`                         |
| Right-align in 10 spaces      | `{name:>10}`            | `{:>10}`                         |
| Center-align in 10 spaces     | `{name:^10}`            | `{:^10}`                         |
| Pad with character            | `{num:0>5}`             | `{:0>5}`                         |
| Named arguments               | `{name}`                | `{name}` (Rust 1.58+)            |

These additions clarify how Rust formatting compares to Python’s powerful f-strings while reinforcing Rust's emphasis on compile-time safety and control.

## 9. What's Next?

You've successfully written your first Rust programs, handling output, variables, and basic input. Great job! The next crucial skill is organizing your code for clarity and reusability.

Continue your Rust journey by exploring **Functions**, where you'll learn how to create your own reusable blocks of code.

Ready? Head over to [`functions.md`](./functions.md).