use trackr::commands::Commands;
use trackr::storage::Storage;
use trackr::task::TaskStatus;
use std::fs;
use std::path::Path;

#[test]
fn test_add_single_task() {
    let test_file = "test_cmd_add_single.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("My first task".to_string());

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[0].description, "My first task");
    assert_eq!(tasks[0].status, TaskStatus::Todo);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_add_multiple_tasks_incremental_ids() {
    let test_file = "test_cmd_add_multiple.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.add("Task 2".to_string());
    commands.add("Task 3".to_string());
    commands.add("Task 4".to_string());

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks.len(), 4);
    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[1].id, 2);
    assert_eq!(tasks[2].id, 3);
    assert_eq!(tasks[3].id, 4);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_update_existing_task() {
    let test_file = "test_cmd_update.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Original description".to_string());
    commands.update(1, "Updated description".to_string());

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].description, "Updated description");
    assert_eq!(tasks[0].id, 1);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_update_nonexistent_task() {
    let test_file = "test_cmd_update_nonexistent.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.update(999, "This should not work".to_string());

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].description, "Task 1");

    fs::remove_file(test_file).ok();
}

#[test]
fn test_delete_existing_task() {
    let test_file = "test_cmd_delete.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.add("Task 2".to_string());
    commands.add("Task 3".to_string());
    commands.delete(2);

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[1].id, 3);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_delete_first_task() {
    let test_file = "test_cmd_delete_first.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("First".to_string());
    commands.add("Second".to_string());
    commands.delete(1);

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].id, 2);
    assert_eq!(tasks[0].description, "Second");

    fs::remove_file(test_file).ok();
}

#[test]
fn test_delete_last_task() {
    let test_file = "test_cmd_delete_last.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("First".to_string());
    commands.add("Last".to_string());
    commands.delete(2);

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].id, 1);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_delete_nonexistent_task() {
    let test_file = "test_cmd_delete_nonexistent.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.delete(999);

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks.len(), 1);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_mark_task_in_progress() {
    let test_file = "test_cmd_mark_progress.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Test task".to_string());
    commands.mark(1, "in-progress".to_string());

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].status, TaskStatus::InProgress);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_mark_task_done() {
    let test_file = "test_cmd_mark_done.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Test task".to_string());
    commands.mark(1, "done".to_string());

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].status, TaskStatus::Done);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_mark_task_back_to_todo() {
    let test_file = "test_cmd_mark_todo.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Test task".to_string());
    commands.mark(1, "done".to_string());
    commands.mark(1, "todo".to_string());

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].status, TaskStatus::Todo);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_mark_invalid_status() {
    let test_file = "test_cmd_mark_invalid.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Test task".to_string());
    commands.mark(1, "invalid-status".to_string());

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks[0].status, TaskStatus::Todo);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_mark_nonexistent_task() {
    let test_file = "test_cmd_mark_nonexistent.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.mark(999, "done".to_string());

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks[0].status, TaskStatus::Todo);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_list_empty_tasks() {
    let test_file = "test_cmd_list_empty.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.list(None);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_list_all_tasks() {
    let test_file = "test_cmd_list_all.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.add("Task 2".to_string());
    commands.add("Task 3".to_string());
    commands.list(None);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_list_filtered_by_todo() {
    let test_file = "test_cmd_list_todo.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.add("Task 2".to_string());
    commands.mark(2, "done".to_string());
    commands.list(Some("todo".to_string()));

    fs::remove_file(test_file).ok();
}

#[test]
fn test_list_filtered_by_done() {
    let test_file = "test_cmd_list_done.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.add("Task 2".to_string());
    commands.mark(1, "done".to_string());
    commands.list(Some("done".to_string()));

    fs::remove_file(test_file).ok();
}

#[test]
fn test_list_filtered_by_in_progress() {
    let test_file = "test_cmd_list_progress.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.add("Task 2".to_string());
    commands.mark(1, "in-progress".to_string());
    commands.list(Some("in-progress".to_string()));

    fs::remove_file(test_file).ok();
}

#[test]
fn test_complete_workflow() {
    let test_file = "test_cmd_workflow.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    // Add tasks
    commands.add("Learn Rust".to_string());
    commands.add("Build trackr".to_string());
    commands.add("Write tests".to_string());

    // Mark tasks
    commands.mark(1, "done".to_string());
    commands.mark(2, "in-progress".to_string());

    // Update a task
    commands.update(3, "Write comprehensive tests".to_string());

    // Delete a task
    commands.add("Temporary task".to_string());
    commands.delete(4);

    // Verify final state
    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].status, TaskStatus::Done);
    assert_eq!(tasks[1].status, TaskStatus::InProgress);
    assert_eq!(tasks[2].description, "Write comprehensive tests");

    fs::remove_file(test_file).ok();
}

#[test]
fn test_add_task_with_special_characters() {
    let test_file = "test_cmd_special.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Fix bug #123 in main.rs".to_string());
    commands.add("Update docs: README.md & CONTRIBUTING.md".to_string());
    commands.add("Add emoji support 🎉🚀".to_string());

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].description, "Fix bug #123 in main.rs");
    assert_eq!(tasks[1].description, "Update docs: README.md & CONTRIBUTING.md");
    assert_eq!(tasks[2].description, "Add emoji support 🎉🚀");

    fs::remove_file(test_file).ok();
}

