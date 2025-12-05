# rusty_stuff 🦀  
My raw, messy, real Rust learning journey – from zero to “I can actually build stuff”

This repo is my public diary.  
Every folder = one tiny project I built when I was stuck on a concept.  
No copied tutorial code. All written by me while screaming at the borrow checker.

### Current state (Dec 2025): I know enough Rust to be **dangerous**  
 Goal: become so comfortable that I stop googling “rust ownership” every 10 minutes

### The projects (in the order I made them)

| # | Project               | What it does                                                                 | Main things I finally understood here                                                                                           | Pain level |
|---|-----------------------|------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------|------------|
| 5 | `rust_game`           | **Infinite procedural rogue-like dungeon crawler** – the world literally generates as you walk. Treasure, keys, potions, traps, health, inventory, `use potion`, `help` command, locked doors… and it never ends. | `HashMap<String, Room>` for dynamic world, **lazy procedural generation**, fixing the borrow checker in infinite loops, separating read/mutate phases, `swap_remove`, `clone()` for String keys, `match` with guards, `format!` room names, true on-the-fly world building | **11/10** (worth every tear) |
| 4 | `rust_advent`         | Advent of Code 2024 Day 1 (in progress)                                      | String parsing, algorithms, fighting the borrow checker at 3 AM                                                                   | 9/10       |
| 3 | `rust_todo`           | Full terminal todo list saved to `todo.json`                                 | `serde`, custom structs, `Vec<T>`, file persistence, lifetimes in structs, `clap` subcommands                                      | 8/10       |
| 2 | `rust_grep`           | Mini grep: `rust_grep pattern file.txt`                                      | File I/O, `BufReader`, iterators, `?` operator, `Result`, `clap` basics                                                                  | 6/10       |
| 1 | `rust_tempcheck`      | Reads temp from args → prints hot/cold/nice                                  | `fn main()`, `std::env::args()`, `match`, `if let`, basic errors                                                                         | 2/10       |

More coming every week until I stop learning new swear words.

### What I’ve actually learned so far (the real list nobody writes down)

**NEW FROM THE INFINITE DUNGEON (the big ones):**
- How to **break the borrow checker’s spirit** when you need both `&self` and `&mut self` in the same function → clone early, mutate late
- `HashMap<String, T>` as a real game world container
- Procedural generation = “only create what the player can see”
- `String` cloning is cheap when you’re using it as keys
- Separating **read phase** from **mutate phase** = the ultimate borrow-checker hack
- `swap_remove` is perfectly fine for inventory and room items
- `format!("{} {}", direction, current_room)` → infinite unique room names
- You can build a **real feeling rogue-like** in < 300 lines of pure Rust

**Everything else I already bragged about:**
- Ownership: I get it now… mostly
- Borrowing & lifetimes: Still hurts but I’m getting better
- `String` vs `&str`: Finally stopped panicking
- `Result<T, E>` and `?`: My new best friends
- `Option<T>` + `unwrap()` → slowly replacing with proper handling
- `Vec<T>`, pushing, iterating, collecting
- `match` and `if let`: I use them everywhere now
- Error handling with custom types
- `serde` + JSON: I can save and load real data!
- `clap v4` for beautiful CLI args
- Writing tests (yes I actually write tests now)
- `.gitignore` works only if you commit it BEFORE building (learned the hard way)

### Tools I use every day now
```bash
cargo check      # my new F5
cargo clippy     # catches my dumb mistakes
cargo fmt        # makes everything looks clean
rust-analyzer    # VS Code extension that reads my mind
git + good commits # because messy history = messy brain