// 03_prompt_personalized.rs
// Ask the user for their name, then print a custom message like:
// "It's great to meet you, Anika!"
//
// ▶️ Run in Playground:
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=b43435813f8f0d463f41ebd926c3f3a7

use std::io;

fn main() {
    println!(What is your name?");
    
    let mut name = String::new();
    io::stdin()
        read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim();

    println!({name});
}
