use std::fs;
use std::io;
use std::io::Write;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    done: bool,
}

fn main() {
    // 1. load existing todos or start empty
    let mut todos: Vec<Task> = load_todos();

    println!("rusty todo list! type 'help' for commands");

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let cmd = input.trim();

        if cmd.is_empty() {
            continue;
        }

        // quit
        if cmd == "quit" || cmd == "exit" {
            println!("byee~ see u next rust adventurre!");
            break;
        }

        // help
        if cmd == "help" {
            println!(
                "commands:\n  list\n  add <task>\n  done <number>\n  delete <number>\n  quit"
            );
            continue;
        }

        // list all
        if cmd == "list" {
            if todos.is_empty() {
                println!("nothing here yet! add some tasks :3");
            } else {
                for (i, task) in todos.iter().enumerate() {
                    let checkbox = if task.done { "✓" } else { "○" };
                    println!("{}. {} {}", i + 1, checkbox, task.description);
                }
            }
            continue;
        }

        // add new task
        if cmd.starts_with("add ") {
            let desc = cmd[4..].trim();
            if desc.is_empty() {
                println!("cant add empty task silly!");
            } else {
                todos.push(Task {
                    description: desc.to_string(),
                    done: false,
                });
                println!("added: {}", desc);
            }
            save_todos(&todos);
            continue;
        }

        // mark as done
        if cmd.starts_with("done ") {
            if let Ok(num) = cmd[5..].trim().parse::<usize>() {
                if num > 0 && num <= todos.len() {
                    todos[num - 1].done = true;
                    println!("nice! task {} is done~", num);
                } else {
                    println!("no task with that number :c");
                }
            } else {
                println!("usage: done <number>");
            }
            save_todos(&todos);
            continue;
        }

        // delete task
        if cmd.starts_with("delete ") {
            if let Ok(num) = cmd[7..].trim().parse::<usize>() {
                if num > 0 && num <= todos.len() {
                    let removed = todos.remove(num - 1);
                    println!("deleted: {}", removed.description);
                } else {
                    println!("no task with that number :c");
                }
            } else {
                println!("usage: delete <number>");
            }
            save_todos(&todos);
            continue;
        }

        // if we get here, unknown command
        println!("unknown command! type 'help' to see options");
    }
}

// helper functions – super clean!
fn load_todos() -> Vec<Task> {
    fs::read_to_string("todo.json")
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(|| {
            println!("no todo.json found – starting fresh!");
            vec![]
        })
}

fn save_todos(todos: &[Task]) {
    let json = serde_json::to_string_pretty(todos).unwrap(); // pretty print yay
    fs::write("todo.json", json).unwrap();
}