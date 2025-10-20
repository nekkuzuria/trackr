use trackr::storage::Storage;
use trackr::task::{Task, TaskStatus};
use std::fs;
use std::path::Path;

#[test]
fn test_storage_new() {
    let storage = Storage::new("test_storage.json");
    assert_eq!(storage.file_path, "test_storage.json");
}

#[test]
fn test_load_tasks_from_nonexistent_file() {
    let test_file = "test_nonexistent.json";

    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let tasks = storage.load_tasks();
    assert_eq!(tasks.len(), 0);
}

#[test]
fn test_save_and_load_single_task() {
    let test_file = "test_single_task.json";
    let storage = Storage::new(test_file);

    let mut tasks = Vec::new();
    tasks.push(Task::new(1, "Single task".to_string()));

    storage.save_tasks(&tasks).expect("Failed to save tasks");

    let loaded_tasks = storage.load_tasks();
    assert_eq!(loaded_tasks.len(), 1);
    assert_eq!(loaded_tasks[0].id, 1);
    assert_eq!(loaded_tasks[0].description, "Single task");
    assert_eq!(loaded_tasks[0].status, TaskStatus::Todo);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_save_and_load_multiple_tasks() {
    let test_file = "test_multiple_tasks.json";
    let storage = Storage::new(test_file);

    let mut tasks = Vec::new();
    tasks.push(Task::new(1, "First task".to_string()));
    tasks.push(Task::with_status(2, "Second task".to_string(), TaskStatus::InProgress));
    tasks.push(Task::with_status(3, "Third task".to_string(), TaskStatus::Done));
    tasks.push(Task::new(4, "Fourth task".to_string()));

    storage.save_tasks(&tasks).expect("Failed to save tasks");

    let loaded_tasks = storage.load_tasks();
    assert_eq!(loaded_tasks.len(), 4);
    assert_eq!(loaded_tasks[0].id, 1);
    assert_eq!(loaded_tasks[0].status, TaskStatus::Todo);
    assert_eq!(loaded_tasks[1].id, 2);
    assert_eq!(loaded_tasks[1].status, TaskStatus::InProgress);
    assert_eq!(loaded_tasks[2].id, 3);
    assert_eq!(loaded_tasks[2].status, TaskStatus::Done);
    assert_eq!(loaded_tasks[3].id, 4);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_save_empty_task_list() {
    let test_file = "test_empty_list.json";
    let storage = Storage::new(test_file);

    let tasks: Vec<Task> = Vec::new();
    let result = storage.save_tasks(&tasks);
    assert!(result.is_ok());

    let loaded_tasks = storage.load_tasks();
    assert_eq!(loaded_tasks.len(), 0);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_overwrite_existing_tasks() {
    let test_file = "test_overwrite.json";
    let storage = Storage::new(test_file);

    let mut tasks1 = Vec::new();
    tasks1.push(Task::new(1, "Original task".to_string()));
    storage.save_tasks(&tasks1).ok();

    let mut tasks2 = Vec::new();
    tasks2.push(Task::new(1, "Updated task".to_string()));
    tasks2.push(Task::new(2, "New task".to_string()));
    storage.save_tasks(&tasks2).ok();

    let loaded_tasks = storage.load_tasks();
    assert_eq!(loaded_tasks.len(), 2);
    assert_eq!(loaded_tasks[0].description, "Updated task");
    assert_eq!(loaded_tasks[1].description, "New task");

    fs::remove_file(test_file).ok();
}

#[test]
fn test_task_with_special_characters() {
    let test_file = "test_special_chars.json";
    let storage = Storage::new(test_file);

    let mut tasks = Vec::new();
    tasks.push(Task::new(1, "Task with backslashes\\here".to_string()));
    tasks.push(Task::new(2, "Task: with, punctuation!".to_string()));
    tasks.push(Task::new(3, "Task with émojis 🎉🚀".to_string()));

    storage.save_tasks(&tasks).ok();
    let loaded_tasks = storage.load_tasks();

    assert_eq!(loaded_tasks.len(), 3);
    assert_eq!(loaded_tasks[0].description, "Task with backslashes\\here");
    assert_eq!(loaded_tasks[1].description, "Task: with, punctuation!");
    assert_eq!(loaded_tasks[2].description, "Task with émojis 🎉🚀");

    fs::remove_file(test_file).ok();
}

#[test]
fn test_large_number_of_tasks() {
    let test_file = "test_large_tasks.json";
    let storage = Storage::new(test_file);

    let mut tasks = Vec::new();
    for i in 1..=100 {
        tasks.push(Task::new(i, format!("Task number {}", i)));
    }

    storage.save_tasks(&tasks).ok();
    let loaded_tasks = storage.load_tasks();

    assert_eq!(loaded_tasks.len(), 100);
    assert_eq!(loaded_tasks[0].id, 1);
    assert_eq!(loaded_tasks[99].id, 100);
    assert_eq!(loaded_tasks[49].description, "Task number 50");

    fs::remove_file(test_file).ok();
}

#[test]
fn test_json_format_validity() {
    let test_file = "test_json_format.json";
    let storage = Storage::new(test_file);

    let mut tasks = Vec::new();
    tasks.push(Task::new(1, "Test".to_string()));

    storage.save_tasks(&tasks).ok();

    let contents = fs::read_to_string(test_file).expect("Failed to read file");
    assert!(contents.contains("\"id\": 1"));
    assert!(contents.contains("\"description\": \"Test\""));
    assert!(contents.contains("\"status\": \"todo\""));
    assert!(contents.starts_with('['));
    assert!(contents.trim().ends_with(']'));

    fs::remove_file(test_file).ok();
}

#[test]
fn test_persistence_across_multiple_loads() {
    let test_file = "test_persistence.json";
    let storage = Storage::new(test_file);

    let mut tasks = Vec::new();
    tasks.push(Task::new(1, "Persistent task".to_string()));
    storage.save_tasks(&tasks).ok();

    let loaded1 = storage.load_tasks();
    let loaded2 = storage.load_tasks();
    let loaded3 = storage.load_tasks();

    assert_eq!(loaded1.len(), 1);
    assert_eq!(loaded2.len(), 1);
    assert_eq!(loaded3.len(), 1);
    assert_eq!(loaded1[0].description, loaded2[0].description);
    assert_eq!(loaded2[0].description, loaded3[0].description);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_task_with_unicode_characters() {
    let test_file = "test_unicode.json";
    let storage = Storage::new(test_file);

    let mut tasks = Vec::new();
    tasks.push(Task::new(1, "学习 Rust 编程".to_string()));
    tasks.push(Task::new(2, "Учить Rust".to_string()));
    tasks.push(Task::new(3, "Apprendre Rust".to_string()));

    storage.save_tasks(&tasks).ok();
    let loaded_tasks = storage.load_tasks();

    assert_eq!(loaded_tasks.len(), 3);
    assert_eq!(loaded_tasks[0].description, "学习 Rust 编程");
    assert_eq!(loaded_tasks[1].description, "Учить Rust");
    assert_eq!(loaded_tasks[2].description, "Apprendre Rust");

    fs::remove_file(test_file).ok();
}

#[test]
fn test_all_task_statuses() {
    let test_file = "test_all_statuses.json";
    let storage = Storage::new(test_file);

    let mut tasks = Vec::new();
    tasks.push(Task::with_status(1, "Todo task".to_string(), TaskStatus::Todo));
    tasks.push(Task::with_status(2, "In progress task".to_string(), TaskStatus::InProgress));
    tasks.push(Task::with_status(3, "Done task".to_string(), TaskStatus::Done));

    storage.save_tasks(&tasks).ok();
    let loaded_tasks = storage.load_tasks();

    assert_eq!(loaded_tasks[0].status, TaskStatus::Todo);
    assert_eq!(loaded_tasks[1].status, TaskStatus::InProgress);
    assert_eq!(loaded_tasks[2].status, TaskStatus::Done);

    fs::remove_file(test_file).ok();
}

