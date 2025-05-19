# Drawing Shapes and Graphics

## 1. Introduction to `macroquad`

`macroquad` is a beginner-friendly crate for creating **2D graphics in Rust**, designed to be easy to use, fast to compile, and deployable to both desktop and web using WebAssembly.

Unlike the Rust Playground, which doesn't support any graphical output, `macroquad` lets you open a window, draw shapes, respond to input, and even build simple games — all with minimal setup.

For more information, visit the official crate page:
👉 https://crates.io/crates/macroquad

## 2. 🚀 Why Use `macroquad`?

- Simple API — just draw shapes, images, or text with one line.
- No need to manage complex event loops manually.
- Runs on Windows, macOS, Linux, and in the browser (via WASM).
- Perfect for learning programming with visual feedback.

## 3. 📦 Installation

Create a new Rust project to contain any graphics projects:

```bash
cargo new graphics
cd graphics
```

Update your `Cargo.toml` to include:

```toml
[dependencies]
macroquad = "0.4"
```

## 4. 🟡 Your First `macroquad` Program

Replace the contents of `src/main.rs` with:

```rust
use macroquad::prelude::*;

#[macroquad::main("Basic Shapes")]
async fn main() {
    loop {
        clear_background(WHITE);

        draw_circle(200.0, 150.0, 50.0, RED);
        draw_rectangle(300.0, 100.0, 120.0, 80.0, BLUE);
        draw_line(100.0, 400.0, 500.0, 400.0, 2.0, BLACK);

        next_frame().await;
    }
}
```

Then run:

```bash
cargo run
```

You’ll see a window with a red circle, a blue rectangle, and a black line — drawn using just a few lines of code.

For a complete list of shape-drawing functions, visit:
👉 https://docs.rs/macroquad/0.4.14/macroquad/shapes/index.html

