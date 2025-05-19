# 📝 To-Do List for Rust Teaching Markdown Files

## 🖨️ print.md

- [ ] 🆕 Mark first-time use of `.expect()`, `.trim()`, `mut`, `use`, variables, inline comments
- [ ] Add explanations or examples for `write!` and `writeln!` using `std::fmt::Write`
- [ ] Add common error blocks: missing semicolon, missing `!` in `println!`, type mismatches
- [ ] Add optional callout about `Result`, `unwrap()`, and `expect()` error handling
- [ ] Extend formatting section with alignment, padding, and formatting examples
- [ ] 📌 Create a visual glossary of first-time terms with links to where they appear
- [ ] ▶️ Update all "Run in Playground" links to include ▶️ emoji

## 🧮 math.md

- [1] Many sections need to be expanded with text
- [ ] Add a numeric type hierarchy chart: signedness, size, casting behavior
- [ ] Add examples using `.checked_add`, `.wrapping_add` for safe math
- [ ] 🆕 Tag first-time uses of `as` casting, `consts::PI`, and `use rand::Rng`
- [ ] Add branching control flow using `match` and `if let` examples
- [ ] 📊 Stub example using `plotters` crate for graphing (even if not executable in Playground)

## 🧾 input.md

- [ ] Add complete example for `input_int()` and `input_until_valid()` functions
- [ ] 🆕 Mark first-time appearance of `.unwrap()` in helper context
- [ ] Add notes on `.parse()` error cases and how to loop until valid input
- [ ] Show how to use `match` or `Result` inside an input loop

## 🎨 graphics.md

- [ ] Add examples using keyboard input like `is_key_down()`
- [ ] Add animation timing with `get_frame_time()` to show movement per frame
- [ ] 🐢 Begin turtle-style API section (e.g., `pen.forward(10)`)
- [ ] 📖 Add comparison table: `macroquad` vs `minifb` vs `ggez` vs `bevy`
- [ ] 🆕 Tag first-time use of attribute macros like `#[macroquad::main]`
- [ ] Explain what `async fn main()` means and why it's used in `macroquad`
- [ ] Move shapes around interactively
- [ ] Simulate basic physics (e.g. gravity, collisions)
- [ ] Animate graphics
- [ ] Create a simple turtle-graphics-like drawing interface

## functions.md

- [ ] Optional parameters (simulated with Option types)
- [ ] Multiple return values using tuples
- [ ] Passing by reference for performance and safety
