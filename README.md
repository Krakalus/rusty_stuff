# rusty_stuff 🦀  
My raw, messy, real Rust learning journey – from zero to “I can actually build stuff”

This repo is my public diary.  
Every folder = one tiny project I built when I was stuck on a concept.  
No copied tutorial code. All written by me while screaming at the borrow checker.

### Current state (Dec 2025): I know enough Rust to be dangerous  
 Goal: become so comfortable that I stop googling “rust ownership” every 10 minutes

### The projects (in the order I made them)

| # | Project           | What it does                                      | Main things I finally understood here                           | Pain level |
|---|-------------------|---------------------------------------------------|------------------------------------------------------------------|------------|
| 1 | `rust_tempcheck`  | Reads temp from args → prints hot/cold/nice       | `fn main()`, `std::env::args()`, `match`, `if let`, basic errors | 2/10       |
| 2 | `rust_grep`       | Mini grep: `rust_grep pattern file.txt`           | File I/O, `BufReader`, iterators, `?` operator, `Result`, `clap` basics | 6/10       |
| 3 | `rust_todo`       | Full terminal todo list saved to `todo.json`      | `serde`, custom structs, `Vec<T>`, file persistence, lifetimes in structs, `clap` subcommands | 8/10       |
| 4 | `rust_advent`     | Advent of Code 2024 Day 1 (in progress)           | String parsing, `split_whitespace`, algorithms, fighting the borrow checker at 3 AM | 9/10       |

More coming every week until I stop learning new swear words.

### What I’ve actually learned so far (the real list nobody writes down)

- Ownership: I get it now… mostly
- Borrowing & lifetimes Still hurts but I’m getting better
- `String` vs `&str` Finally stopped panicking
- `Result<T, E>` and `?` My new best friends
- `Option<T>` + `unwrap()` → slowly replacing with proper handling
- `Vec<T>`, pushing, iterating, collecting
- `match` and `if let` I use them everywhere now
- Error handling with custom types (in rust_todo)
- `serde` + JSON I can save and load real data!
- `clap v4` for beautiful CLI args
- Writing tests (yes I actually write tests now)
- `.gitignore` works only if you commit it BEFORE building (learned the hard way today)

### Tools I use every day now
```bash
cargo check      # my new F5
cargo clippy     # catches my dumb mistakes
cargo fmt        # makes everything looks clean
rust-analyzer    # VS Code extension that reads my mind
git + good commits # because messy history = messy brain