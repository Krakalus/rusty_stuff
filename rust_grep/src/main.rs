// mini_grep.rs - Our awesome Rust grep clone! 🦀🔍
// Run it like: cargo run -- hello poem.txt

use std::env;
use std::fs;
use std::process;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // We need exactly 3 parts: program name, pattern, filename
    if args.len() < 3 {
        eprintln!("🤔 Oops! Usage: {} <pattern> <file>", args[0]);
        eprintln!("Example: {} hello poem.txt", args[0]);
        process::exit(1);
    }

    let pattern = &args[1];
    let filename = &args[2];

    println!("🔍 Searching for: '{}'", pattern);
    println!("📄 In file: {}", filename);

    // Read the entire file (returns Result<String, Error>)
    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|err| {
            eprintln!("💥 Could not read file '{}': {}", filename, err);
            process::exit(1);
        });

    // Let's find and print matching lines!
    let mut found = 0;
    for (number, line) in contents.lines().enumerate() {
        if line.contains(pattern) {
            println!("{}: {}", number + 1, line);
            found += 1;
        }
    }

    // Victory message!
    if found == 0 {
        println!("😢 No matches found for '{}'", pattern);
    } else {
        println!("🎉 Found {} matching line{}!", found, if found == 1 { "" } else { "s" });
    }
}