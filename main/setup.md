# 🛠 Rust Setup and How to Run Code

Welcome! Before jumping into the lessons in this tutorial, you’ll want to make sure you can run Rust code on your computer or in the browser. This guide shows you how to install Rust, use Cargo, and run examples — no matter what platform you’re using.

## 🔧 Installing Rust

Install Rust via the official installer:

👉 https://rustup.rs

This works on **Windows**, **macOS**, and **Linux**.

After installing, verify the installation with:

```bash
rustc --version
```

To keep Rust up to date:

```bash
rustup update
```

## 📁 Understanding a Rust Project

Here’s what a typical Rust project folder looks like:

```
my_project/
├── Cargo.toml        # Project metadata and dependencies
└── src/
    └── main.rs       # Your Rust code starts here
```

You create this structure using:

```bash
cargo new my_project
cd my_project
```

## 🚀 Running Code with Cargo

Inside your project folder:

```bash
cargo run
```

This will compile your project and run the code in `src/main.rs`.

## 🧪 Option 2: Use the Rust Playground (Browser-Based)

For quick experimentation, use:

👉 https://play.rust-lang.org

- Paste any code from this tutorial
- Click **Run**
- Share links with others

All example files include ▶️ **Run in Playground** links for convenience.

## 📦 Adding Crates (Dependencies)

If a lesson uses external crates like `rand` or `macroquad`, add them to your `Cargo.toml`:

```toml
[dependencies]
rand = "0.8"
```

For `macroquad`:

```toml
[dependencies]
macroquad = "0.4"
```

Cargo will fetch and compile them automatically.

## 🧭 What's Next

Now that your setup is complete, start with the first lesson in [`getting-started.md`](./getting-started.md), and use this file as your central reference for running and building Rust programs.
