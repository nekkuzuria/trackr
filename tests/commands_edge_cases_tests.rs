use trackr::commands::Commands;
use trackr::storage::Storage;
use trackr::task::TaskStatus;
use std::fs;
use std::path::Path;

#[test]
fn test_list_with_invalid_filter() {
    let test_file = "test_list_invalid_filter.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.list(Some("invalid-filter".to_string()));

    fs::remove_file(test_file).ok();
}

#[test]
fn test_list_empty_with_filter() {
    let test_file = "test_list_empty_filter.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.mark(1, "done".to_string());
    commands.list(Some("in-progress".to_string()));

    fs::remove_file(test_file).ok();
}

#[test]
fn test_multiple_status_transitions() {
    let test_file = "test_status_transitions.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.mark(1, "in-progress".to_string());
    commands.mark(1, "done".to_string());
    commands.mark(1, "todo".to_string());
    commands.mark(1, "in-progress".to_string());

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();
    assert_eq!(tasks[0].status, TaskStatus::InProgress);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_add_after_delete_reuses_next_id() {
    let test_file = "test_id_sequence.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.add("Task 2".to_string());
    commands.add("Task 3".to_string());
    commands.delete(2);
    commands.add("Task 4".to_string());

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks.len(), 3);
    let ids: Vec<u32> = tasks.iter().map(|t| t.id).collect();
    assert!(ids.contains(&4));

    fs::remove_file(test_file).ok();
}

#[test]
fn test_update_multiple_times() {
    let test_file = "test_multiple_updates.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Original".to_string());
    commands.update(1, "Update 1".to_string());
    commands.update(1, "Update 2".to_string());
    commands.update(1, "Final Update".to_string());

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks[0].description, "Final Update");

    fs::remove_file(test_file).ok();
}

#[test]
fn test_delete_all_tasks() {
    let test_file = "test_delete_all.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.add("Task 2".to_string());
    commands.add("Task 3".to_string());

    commands.delete(1);
    commands.delete(2);
    commands.delete(3);

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks.len(), 0);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_mark_case_insensitive_status() {
    let test_file = "test_mark_case.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task 1".to_string());
    commands.mark(1, "IN-PROGRESS".to_string());

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks[0].status, TaskStatus::InProgress);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_complex_workflow_scenario() {
    let test_file = "test_complex_workflow.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Setup project".to_string());
    commands.add("Write code".to_string());
    commands.add("Test code".to_string());
    commands.add("Deploy".to_string());

    commands.mark(1, "done".to_string());
    commands.mark(2, "in-progress".to_string());

    commands.update(3, "Write comprehensive tests".to_string());

    commands.delete(4);

    commands.add("Review code".to_string());

    commands.list(None);
    commands.list(Some("done".to_string()));
    commands.list(Some("in-progress".to_string()));
    commands.list(Some("todo".to_string()));

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks.len(), 4);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_add_empty_description() {
    let test_file = "test_empty_desc.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("".to_string());

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].description, "");

    fs::remove_file(test_file).ok();
}

#[test]
fn test_update_with_same_description() {
    let test_file = "test_same_desc.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Same description".to_string());
    commands.update(1, "Same description".to_string());

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks[0].description, "Same description");

    fs::remove_file(test_file).ok();
}

#[test]
fn test_mark_with_uppercase_status() {
    let test_file = "test_uppercase_status.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    commands.add("Task".to_string());
    commands.mark(1, "DONE".to_string());

    let storage2 = Storage::new(test_file);
    let tasks = storage2.load_tasks();

    assert_eq!(tasks[0].status, TaskStatus::Done);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_list_all_statuses_mixed() {
    let test_file = "test_mixed_statuses.json";
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let storage = Storage::new(test_file);
    let commands = Commands::new(storage);

    for i in 1..=10 {
        commands.add(format!("Task {}", i));
    }

    commands.mark(1, "done".to_string());
    commands.mark(2, "done".to_string());
    commands.mark(3, "in-progress".to_string());
    commands.mark(4, "in-progress".to_string());
    commands.mark(5, "in-progress".to_string());

    commands.list(None);
    commands.list(Some("todo".to_string()));
    commands.list(Some("in-progress".to_string()));
    commands.list(Some("done".to_string()));

    fs::remove_file(test_file).ok();
}

