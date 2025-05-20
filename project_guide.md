# 🧭 Rust-Tutorial Project Guide

This document explains the structure, goals, conventions, and workflow used in the `Rust-Tutorial` project. It's designed to make it easy to resume work, onboard collaborators, or rebuild from scratch.

## 📌 Project Overview

This is a beginner-focused tutorial for learning Rust. The project is organized into:

- **Markdown-based lessons** in `main/` (e.g. `print.md`, `input.md`)
- **Linked exercises** in `exercises/<topic>/`
- **Solutions** in `solutions/<topic>/` (optional)
- **Meta files** like `README.md`, `ToDo.md`, `setup.md`, and `introduction.md`

Each section introduces one or more concepts followed by at least one exercise. Exercises are written in `.rs` files and linked inline from the lesson content.

## 🗂 Directory Structure

```
Rust-Tutorial/
├── main/                    # Lesson content (Markdown)
│   ├── print.md
│   ├── input.md
│   ├── math.md
│   ├── functions.md
│   └── ...
├── exercises/               # Exercises grouped by topic
│   ├── getting-started/
│   │   ├── 01_customize_hello.rs
│   │   ├── 02_variable_greeting.rs
│   │   └── ...
├── solutions/               # Optional solutions (same structure as exercises/)
├── setup.md                 # Setup guide: installing Rust, using Playground, etc.
├── introduction.md          # Overview and structure of the tutorial
├── ToDo.md                  # Author-facing planning file
├── README.md                # Project entry point
```

## 📝 Writing Lessons (`*.md`)

### ✅ Style Guide

- Use Markdown `##` headings (no horizontal rules before headings — ever)
- Keep tone **clear**, **friendly**, and **progressive**
- Use **🆕** icon for the *first appearance* of a concept across lessons
- Include ▶️ **Run in Playground** links where helpful
- Refer to `setup.md` instead of repeating Cargo or install instructions
- Refer to `introduction.md` at the start of each lesson

### 🔣 Code Block Syntax

Use fenced code blocks with `rust`:

    ```rust
    fn main() {
        println!("Hello!");
    }
    ```

## ✏️ Exercise Conventions

### 🔢 File Naming

- Place each exercise in `exercises/<topic>/`
- Prefix the file with a two-digit number (e.g. `01_`, `02_`) so they stay in order
- Use a short snake_case description after the number
- Numbering restarts within each topic
- All exercise files use the `.rs` extension


### 🔗 Markdown Embed Format

```markdown
### ✏️ Exercise: Short Descriptive Title

One-line instruction or short paragraph explaining the task.

[Solve this exercise](../exercises/<topic>/<filename>.rs)
```

Place each exercise:
- Immediately after the concept it practices
- Do *not* leave all exercises for the end of a lesson

### 📄 Rust File Format

Each `.rs` exercise file should include:

- A short comment block explaining the task
- A ▶️ Run in Playground link
- A starting skeleton or functional starter code

```rust
// 01_customize_hello.rs
// Modify this message to greet someone else.
//
// ▶️ Run in Playground:
// https://play.rust-lang.org/?...

fn main() {
    println!("Hello, world!");
}
```

## 🔁 Workflow

1. **Write the concept** in Markdown
2. **Design an exercise** that matches it
3. **Create the `.rs` file** in `exercises/<topic>/`
4. **Generate a Run in Playground link**
5. **Link it inline in the `.md` file**
6. Use `ToDo.md` to track ideas and future content

## 💡 Tip

If you’re restarting from a new session:
- Reload `setup.md` and `introduction.md` first
- Follow file naming conventions exactly
- You can regenerate Playground links on-demand if needed

