# Glossary

This glossary provides definitions for key Rust terms and concepts used throughout this tutorial.

* **async fn**: Declares an asynchronous function, used in the `macroquad` graphics example to handle the main application loop.
* **`as`**: Keyword used for explicit type casting between different data types (e.g., converting an integer to a float).
* **Array**: A fixed-size collection of elements of the same type.
* **Basic Arithmetic**: Fundamental mathematical operations including addition (`+`), subtraction (`-`), multiplication (`*`), division (`/`), and remainder (`%`).
* **Cargo**: Rust's build tool and package manager. Used to create, build, and run Rust projects, and manage dependencies (crates).
* **`Cargo.toml`**: The manifest file for a Rust project, used by Cargo to manage project settings and dependencies.
* **Crate**: The smallest unit of code that the Rust compiler considers. Can be either a binary (executable) or a library. External crates are downloaded dependencies from Crates.io.
* **`clear_background()`**: A function (specifically a macroquad function) used to clear the drawing surface with a specified color in graphical applications.
* **Control Flow**: The order in which individual statements, instructions or function calls of an imperative program are executed or evaluated. This includes constructs like `if` statements and `match` expressions.
* **Crates.io**: The official Rust package registry, where Rust developers can publish and share their libraries (crates).
* **`draw_circle()`, `draw_rectangle()`, `draw_line()`**: Functions (specifically macroquad functions) used to draw shapes in a graphical application.
* **`.expect()`**: A method used on `Result` types for basic error handling. It will cause the program to panic (crash) with a provided message if the `Result` is an `Err`.
* **External Crate**: A library downloaded from Crates.io (or another source) and added as a dependency to your project using `Cargo.toml`.
* **`f32`**: A 32-bit floating-point number type in Rust, offering approximately 7 decimal digits of precision.
* **`f64`**: A 64-bit floating-point number type in Rust, offering approximately 15 decimal digits of precision. This is the default floating-point type in Rust when type inference is used.
* **`flush()`**: A method used on output streams (like standard output) to ensure that any buffered data is immediately written to the destination.
* **`fn main()`**: The entry point of a Rust executable program. Code execution begins here.
* **`format!`**: A macro that works like `println!` but returns a `String` containing the formatted output instead of printing it to the console.
* **Function**: A block of code that performs a specific task and can be called by name. Defined using the `fn` keyword.
* **`i32`**: A 32-bit signed integer type in Rust. This is the default integer type when type inference is used.
* **`i64`**: A 64-bit signed integer type in Rust, used for larger integer values.
* **`if` statements**: A control flow construct that executes a block of code only if a specified condition is true.
* **Immutability**: A property of variables in Rust where, by default, their value cannot be changed after being initialized.
* **`input()`**: A custom helper function created in the tutorial to simplify reading user input from the console, mimicking Python's `input()`.
* **Integer Types**: Data types in Rust used to represent whole numbers (numbers without a fractional component), such as `i32`, `u32`, `i64`, `u64`, etc.
* **Iterator**: A trait that allows for processing a sequence of elements in a collection (like a vector or array). Methods like `.iter().sum()` are used with iterators.
* **`let`**: Keyword used to declare variables in Rust. Variables declared with `let` are immutable by default.
* **`let mut`**: Declares a mutable variable in Rust, meaning its value can be changed after initialization.
* **`loop`**: A control flow construct that repeatedly executes a block of code. In the `macroquad` example, it creates the main application loop.
* **Loops**: Control flow constructs used for repeatedly executing a block of code, such as `for` loops.
* **`macroquad`**: A beginner-friendly external crate for creating 2D graphics and simple games in Rust.
* **`#[macroquad::main]`**: An attribute (specifically a macro) used with the `macroquad` crate to designate the main function for the graphics application.
* **Macro**: A feature in Rust that allows writing code that writes other code. Indicated by a `!` at the end of the name (e.g., `println!`, `format!`).
* **`match` expressions**: A powerful control flow construct that allows comparing a value against a series of patterns and executing code based on the first matching pattern.
* **Method**: A function associated with a specific data type (e.g., `.trim()` is a method of the `String` type).
* **Module**: Rust's way of organizing code. Modules can contain functions, structs, enums, and other modules. Used with the `use` keyword to bring items into scope.
* **Mutability**: The ability of a variable's value to be changed after it has been initialized, indicated by the `mut` keyword.
* **`next_frame().await`**: A function/method call (specifically in `macroquad`) that signals the end of drawing for the current frame and waits for the next frame to begin. The `.await` part is used with asynchronous functions.
* **Order of Operations**: The set of rules that dictate the sequence in which mathematical operations are performed in an expression (similar to PEMDAS/BODMAS).
* **`.parse()`**: A method often used to convert a string into a numeric type. It returns a `Result` indicating success or failure.
* **Playground (Rust Playground)**: An online tool that allows you to write and run Rust code directly in your web browser without local installation.
* **`print!`**: A macro that prints output to the standard console without adding a newline character at the end.
* **`println!`**: A macro that prints output to the standard console, followed by a newline character.
* **`rand`**: An external crate used for generating random numbers in Rust.
* **`read_line()`**: A method from `std::io` used to read a line of text from an input stream (like standard input) into a mutable string buffer.
* **Standard Library (`std`)**: The core set of modules and functionalities that are included with every Rust installation, providing essential tools for tasks like input/output (`std::io`), collections, and basic operations.
* **`std::f64::consts::PI`**: A constant value for the mathematical constant Pi ($\pi$) available in the standard library for `f64` floating-point numbers.
* **`std::io`**: The input/output module within the Rust standard library, providing functionalities for reading from and writing to streams like the console.
* **String**: A growable, mutable sequence of UTF-8 encoded characters (`String` is typically owned, while `&str` is a string slice).
* **String Literal**: An immutable sequence of characters embedded directly in the source code, enclosed in double quotes (e.g., `"Hello, world!"`).
* **Struct**: A custom data type that allows you to group related data together under a single name.
* **`.to_string()`**: A method used to convert a value into an owned `String`.
* **`.trim()`**: A method used on string types to remove leading and trailing whitespace characters (including newlines).
* **Tuple**: A simple fixed-size collection of values of potentially different types, grouped together using parentheses.
* **Type Casting**: Explicitly converting a value from one data type to another (using the `as` keyword).
* **Type Inference**: The ability of the Rust compiler to automatically determine the data type of a variable based on the value it is assigned.
* **Type Safety**: A characteristic of Rust where the compiler enforces strict rules about how different data types can be used together, preventing many common programming errors at compile time.
* **`u32`**: A 32-bit unsigned integer type in Rust, which can only hold non-negative values (0 and positive).
* **`u64`**: A 64-bit unsigned integer type in Rust, used for larger non-negative integer values.
* **`.unwrap()`**: A method used on `Result` or `Option` types for basic error handling. It will return the inner value if it's `Ok` or `Some`, and panic (crash) if it's `Err` or `None`. Use with caution in production code.
* **`use`**: Keyword used to bring items (modules, functions, types, etc.) into the current scope, making them available to use by name.
* **Variable**: A named storage location that holds a value in a program. Declared using `let`.
* **Vector**: A growable, heap-allocated array-like data structure that can hold a variable number of elements of the same type.
* **WebAssembly (WASM)**: A binary instruction format designed as a portable target for compilation of high-level languages like Rust, enabling deployment on the web.
* **`write!`**: A macro similar to `print!` but used to write formatted output to a specified destination that implements the `Write` trait (like a file or buffer) instead of directly to standard output.
* **`writeln!`**: A macro similar to `write!` but adds a newline character at the end of the output.