use trackr::commands::Commands;
use trackr::storage::Storage;
use std::fs;

#[test]
fn test_reset_empty_list() {
    let test_file = "test_reset_empty.json";
    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.reset();

    let loaded_storage = Storage::new(test_file);
    let tasks = loaded_storage.load_tasks();
    assert_eq!(tasks.len(), 0);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_reset_with_tasks() {
    let test_file = "test_reset_with_tasks.json";
    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.add("Task 2".to_string());
    commands.add("Task 3".to_string());

    let loaded_storage = Storage::new(test_file);
    let tasks_before = loaded_storage.load_tasks();
    assert_eq!(tasks_before.len(), 3);

    commands.reset();

    let tasks_after = loaded_storage.load_tasks();
    assert_eq!(tasks_after.len(), 0);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_reset_then_add_new_task() {
    let test_file = "test_reset_then_add.json";
    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.add("Task 2".to_string());
    commands.reset();

    commands.add("New Task".to_string());

    let loaded_storage = Storage::new(test_file);
    let tasks = loaded_storage.load_tasks();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[0].description, "New Task");

    fs::remove_file(test_file).ok();
}

#[test]
fn test_reset_multiple_times() {
    let test_file = "test_reset_multiple.json";
    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.reset();

    let loaded_storage = Storage::new(test_file);
    let tasks_first = loaded_storage.load_tasks();
    assert_eq!(tasks_first.len(), 0);

    commands.reset();

    let tasks_second = loaded_storage.load_tasks();
    assert_eq!(tasks_second.len(), 0);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_reset_clears_all_task_data() {
    let test_file = "test_reset_all_data.json";
    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Todo task".to_string());
    commands.add("In progress task".to_string());
    commands.add("Done task".to_string());

    commands.mark(2, "in-progress".to_string());
    commands.mark(3, "done".to_string());

    let loaded_storage = Storage::new(test_file);
    let tasks_before = loaded_storage.load_tasks();
    assert_eq!(tasks_before.len(), 3);

    commands.reset();

    let tasks_after = loaded_storage.load_tasks();
    assert_eq!(tasks_after.len(), 0);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_reset_preserves_file_structure() {
    let test_file = "test_reset_file_structure.json";
    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.reset();

    assert!(std::path::Path::new(test_file).exists());

    let content = fs::read_to_string(test_file).unwrap();
    let trimmed = content.trim();
    assert!(trimmed == "[]" || trimmed == "[\n]");

    fs::remove_file(test_file).ok();
}

