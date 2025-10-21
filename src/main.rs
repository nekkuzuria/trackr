mod task;
mod storage;
mod commands;
mod quotes;

use std::env;
use storage::Storage;
use commands::Commands;

fn print_banner() {
    println!("\x1b[95m");
    println!("");
    println!("           /\\_/\\  ");
    println!("          ( o.o ) ");
    println!("           > ^ <  ");
    println!("");
    println!("    ████████╗██████╗  █████╗  ██████╗██╗  ██╗██████╗ ");
    println!("    ╚══██╔══╝██╔══██╗██╔══██╗██╔════╝██║ ██╔╝██╔══██╗");
    println!("       ██║   ██████╔╝███████║██║     █████╔╝ ██████╔╝");
    println!("       ██║   ██╔══██╗██╔══██║██║     ██╔═██╗ ██╔══██╗");
    println!("       ██║   ██║  ██║██║  ██║╚██████╗██║  ██╗██║  ██║");
    println!("       ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝");
    println!("");
    println!("              \x1b[96mstay pawsitive 😸🐾\x1b[0m");
    println!();
}

fn print_help() {
    println!("\x1b[96m📝 How to use trackr:\x1b[0m\n");
    println!("\x1b[93m  trackr add <description>\x1b[0m");
    println!("    Add a new task\n");
    println!("\x1b[93m  trackr update <id> <new_description>\x1b[0m");
    println!("    Update an existing task\n");
    println!("\x1b[93m  trackr delete <id>\x1b[0m");
    println!("    Delete a task\n");
    println!("\x1b[93m  trackr mark <id> <status>\x1b[0m");
    println!("    Mark task status (todo, in-progress, done)\n");
    println!("\x1b[93m  trackr list [status]\x1b[0m");
    println!("    List all tasks or filter by status\n");
}

fn main() {
    print_banner();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let storage = Storage::new("tasks.json");
    let commands = Commands::new(storage);

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("\x1b[91m😿 Error: Please provide a task description!\x1b[0m");
                println!("\x1b[93m   Usage: trackr add <description>\x1b[0m\n");
                return;
            }
            let description = args[2..].join(" ");
            commands.add(description);
        }
        "update" => {
            if args.len() < 4 {
                println!("\x1b[91m😿 Error: Please provide task ID and new description!\x1b[0m");
                println!("\x1b[93m   Usage: trackr update <id> <new_description>\x1b[0m\n");
                return;
            }
            match args[2].parse::<u32>() {
                Ok(id) => {
                    let new_description = args[3..].join(" ");
                    commands.update(id, new_description);
                }
                Err(_) => {
                    println!("\x1b[91m😿 Error: Invalid task ID! Must be a number.\x1b[0m\n");
                }
            }
        }
        "delete" => {
            if args.len() < 3 {
                println!("\x1b[91m😿 Error: Please provide a task ID!\x1b[0m");
                println!("\x1b[93m   Usage: trackr delete <id>\x1b[0m\n");
                return;
            }
            match args[2].parse::<u32>() {
                Ok(id) => commands.delete(id),
                Err(_) => {
                    println!("\x1b[91m😿 Error: Invalid task ID! Must be a number.\x1b[0m\n");
                }
            }
        }
        "mark" => {
            if args.len() < 4 {
                println!("\x1b[91m😿 Error: Please provide task ID and status!\x1b[0m");
                println!("\x1b[93m   Usage: trackr mark <id> <status>\x1b[0m");
                println!("\x1b[93m   Status options: todo, in-progress, done\x1b[0m\n");
                return;
            }
            match args[2].parse::<u32>() {
                Ok(id) => {
                    let status = args[3].clone();
                    commands.mark(id, status);
                }
                Err(_) => {
                    println!("\x1b[91m😿 Error: Invalid task ID! Must be a number.\x1b[0m\n");
                }
            }
        }
        "list" => {
            let filter = if args.len() >= 3 {
                Some(args[2].clone())
            } else {
                None
            };
            commands.list(filter);
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        _ => {
            println!("\x1b[91m😿 Unknown command: {}\x1b[0m\n", command);
            print_help();
        }
    }
}

