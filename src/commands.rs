use crate::storage::Storage;
use crate::task::{Task, TaskStatus};

pub struct Commands {
    storage: Storage,
}

impl Commands {
    pub fn new(storage: Storage) -> Commands {
        Commands { storage }
    }

    pub fn add(&self, description: String) {
        let mut tasks = self.storage.load_tasks();
        let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

        let task = Task::new(next_id, description.clone());
        tasks.push(task);

        match self.storage.save_tasks(&tasks) {
            Ok(_) => {
                println!("\n\x1b[95m😸 Task added successfully, slay!\x1b[0m");
                println!("\x1b[96m   ID: {} | {}\x1b[0m\n", next_id, description);
            }
            Err(e) => {
                println!("\n\x1b[91m😿 Oops! Failed to save: {}\x1b[0m\n", e);
            }
        }
    }

    pub fn update(&self, id: u32, new_description: String) {
        let mut tasks = self.storage.load_tasks();

        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.description = new_description.clone();

            match self.storage.save_tasks(&tasks) {
                Ok(_) => {
                    println!("\n\x1b[95m✨ Task updated, you're killing it!\x1b[0m");
                    println!("\x1b[96m   ID: {} | {}\x1b[0m\n", id, new_description);
                }
                Err(e) => {
                    println!("\n\x1b[91m😿 Failed to save: {}\x1b[0m\n", e);
                }
            }
        } else {
            println!("\n\x1b[91m😿 Task not found, meow again!\x1b[0m\n");
        }
    }

    pub fn delete(&self, id: u32) {
        let mut tasks = self.storage.load_tasks();
        let initial_len = tasks.len();

        tasks.retain(|t| t.id != id);

        if tasks.len() < initial_len {
            match self.storage.save_tasks(&tasks) {
                Ok(_) => {
                    println!("\n\x1b[95m🗑️  Task deleted! Bye bye task #{}\x1b[0m\n", id);
                }
                Err(e) => {
                    println!("\n\x1b[91m😿 Failed to save: {}\x1b[0m\n", e);
                }
            }
        } else {
            println!("\n\x1b[91m😿 Task not found, meow again!\x1b[0m\n");
        }
    }

    pub fn mark(&self, id: u32, status_str: String) {
        let status = match TaskStatus::from_str(&status_str) {
            Some(s) => s,
            None => {
                println!("\n\x1b[91m😿 Invalid status! Use: todo, in-progress, or done\x1b[0m\n");
                return;
            }
        };

        let mut tasks = self.storage.load_tasks();
        let mut found = false;
        let mut description = String::new();

        for task in tasks.iter_mut() {
            if task.id == id {
                task.status = status.clone();
                description = task.description.clone();
                found = true;
                break;
            }
        }

        if found {
            match self.storage.save_tasks(&tasks) {
                Ok(_) => {
                    println!("\n\x1b[95m{} Task marked as {}! Keep going!\x1b[0m",
                        status.emoji(), status.to_string());
                    println!("\x1b[96m   ID: {} | {}\x1b[0m\n", id, description);
                }
                Err(e) => {
                    println!("\n\x1b[91m😿 Failed to save: {}\x1b[0m\n", e);
                }
            }
        } else {
            println!("\n\x1b[91m😿 Task not found, meow again!\x1b[0m\n");
        }
    }

    pub fn list(&self, filter_status: Option<String>) {
        let tasks = self.storage.load_tasks();

        let filtered_tasks: Vec<&Task> = if let Some(status_str) = filter_status {
            if let Some(status) = TaskStatus::from_str(&status_str) {
                tasks.iter().filter(|t| t.status == status).collect()
            } else {
                println!("\n\x1b[91m😿 Invalid status! Use: todo, in-progress, or done\x1b[0m\n");
                return;
            }
        } else {
            tasks.iter().collect()
        };

        if filtered_tasks.is_empty() {
            println!("\n\x1b[93m🐾 No tasks found! Time to add some vibes~\x1b[0m\n");
            return;
        }

        println!("\n\x1b[96m🐾 Listing your vibes (tasks)...\x1b[0m\n");
        println!("\x1b[90m{:<6} {:<15} {}\x1b[0m", "ID", "STATUS", "DESCRIPTION");
        println!("\x1b[90m{}\x1b[0m", "─".repeat(60));

        for task in filtered_tasks {
            let status_display = format!("{} {}", task.status.emoji(), task.status.to_string());
            let color = match task.status {
                TaskStatus::Todo => "\x1b[97m",
                TaskStatus::InProgress => "\x1b[93m",
                TaskStatus::Done => "\x1b[92m",
            };

            println!("{}{:<6} {:<15} {}\x1b[0m",
                color, task.id, status_display, task.description);
        }

        println!();
    }
}

