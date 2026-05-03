use std::env;
use std::process;

mod store;
mod task;

use store::Store;
use task::Priority;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let mut store = Store::load().unwrap_or_else(|e| {
        eprintln!("Failed to load store: {e}");
        process::exit(1);
    });

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: trellis add <title> [--priority high|medium|low]");
                process::exit(1);
            }
            let title = &args[2];
            let priority = parse_priority(&args[3..]);
            let id = store.add(title, priority);
            println!("Created task #{id}: {title}");
        }
        "list" => {
            let tasks = store.list();
            if tasks.is_empty() {
                println!("No tasks yet. Try: trellis add \"My first task\"");
                return;
            }
            for t in tasks {
                let marker = if t.done { "x" } else { " " };
                let pri = match t.priority {
                    Priority::High => "!!",
                    Priority::Medium => "! ",
                    Priority::Low => "  ",
                };
                println!("[{marker}] {pri} #{} {}", t.id, t.title);
            }
        }
        "done" => {
            if args.len() < 3 {
                eprintln!("Usage: trellis done <id>");
                process::exit(1);
            }
            let id: usize = args[2].parse().unwrap_or_else(|_| {
                eprintln!("Invalid task ID");
                process::exit(1);
            });
            if store.complete(id) {
                println!("Task #{id} marked done.");
            } else {
                eprintln!("Task #{id} not found.");
                process::exit(1);
            }
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("Usage: trellis remove <id>");
                process::exit(1);
            }
            let id: usize = args[2].parse().unwrap_or_else(|_| {
                eprintln!("Invalid task ID");
                process::exit(1);
            });
            if store.remove(id) {
                println!("Task #{id} removed.");
            } else {
                eprintln!("Task #{id} not found.");
                process::exit(1);
            }
        }
        "help" | "--help" | "-h" => print_usage(),
        other => {
            eprintln!("Unknown command: {other}");
            print_usage();
            process::exit(1);
        }
    }

    store.save().unwrap_or_else(|e| {
        eprintln!("Failed to save store: {e}");
        process::exit(1);
    });
}

fn parse_priority(args: &[String]) -> Priority {
    for (i, arg) in args.iter().enumerate() {
        if arg == "--priority" {
            if let Some(val) = args.get(i + 1) {
                return match val.to_lowercase().as_str() {
                    "high" | "h" => Priority::High,
                    "medium" | "m" => Priority::Medium,
                    "low" | "l" => Priority::Low,
                    _ => Priority::Medium,
                };
            }
        }
    }
    Priority::Medium
}

fn print_usage() {
    println!(
        "trellis — a small task tracker

Usage:
    trellis add <title> [--priority high|medium|low]
    trellis list
    trellis done <id>
    trellis remove <id>
    trellis help
"
    );
}
