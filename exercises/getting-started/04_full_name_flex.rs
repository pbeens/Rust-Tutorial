// 04_full_name_flex.rs
// Extend the program to also ask for a middle name,
// OR change the final output to print in "Last, First" format.
//
// ▶️ Run in Playground:
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=1548445aa86f6938036cf68c738606a2

use std::io;

fn main() {
    let mut fname = String::new();
    let mut lname = String::new();

    println!("Enter your first name:");
    io::stdin().read_line(&mut fname).expect("Failed to read line");

    println!("Enter your last name:");
    io::stdin().read_line(&mut lname).expect("Failed to read line");

    println!("Hello, {} {}!", fname.trim(), lname.trim());
}
