use trackr::storage::Storage;
use trackr::task::Task;
use std::fs::{self, File};
use std::io::Write;

#[test]
fn test_load_tasks_with_corrupted_json() {
    let test_file = "test_corrupted.json";

    let mut file = File::create(test_file).unwrap();
    file.write_all(b"{ this is not valid json }").unwrap();
    drop(file);

    let storage = Storage::new(test_file);
    let tasks = storage.load_tasks();

    assert_eq!(tasks.len(), 0);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_load_tasks_with_empty_array() {
    let test_file = "test_empty_array.json";

    let mut file = File::create(test_file).unwrap();
    file.write_all(b"[]").unwrap();
    drop(file);

    let storage = Storage::new(test_file);
    let tasks = storage.load_tasks();

    assert_eq!(tasks.len(), 0);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_load_tasks_with_whitespace_only() {
    let test_file = "test_whitespace.json";

    let mut file = File::create(test_file).unwrap();
    file.write_all(b"   \n   \t   ").unwrap();
    drop(file);

    let storage = Storage::new(test_file);
    let tasks = storage.load_tasks();

    assert_eq!(tasks.len(), 0);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_load_tasks_with_invalid_array_format() {
    let test_file = "test_invalid_array.json";

    let mut file = File::create(test_file).unwrap();
    file.write_all(b"[{incomplete").unwrap();
    drop(file);

    let storage = Storage::new(test_file);
    let tasks = storage.load_tasks();

    assert_eq!(tasks.len(), 0);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_load_tasks_with_missing_fields() {
    let test_file = "test_missing_fields.json";

    let json = r#"[
        {
            "id": 1
        },
        {
            "description": "No ID"
        },
        {
            "id": 2,
            "description": "No status"
        }
    ]"#;

    let mut file = File::create(test_file).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    drop(file);

    let storage = Storage::new(test_file);
    let tasks = storage.load_tasks();

    assert_eq!(tasks.len(), 0);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_load_tasks_with_invalid_status() {
    let test_file = "test_invalid_status.json";

    let json = r#"[
        {
            "id": 1,
            "description": "Task",
            "status": "invalid-status"
        }
    ]"#;

    let mut file = File::create(test_file).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    drop(file);

    let storage = Storage::new(test_file);
    let tasks = storage.load_tasks();

    assert_eq!(tasks.len(), 0);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_parse_json_with_nested_braces() {
    let test_file = "test_nested_braces.json";

    let json = r#"[
        {
            "id": 1,
            "description": "Task with {braces}",
            "status": "todo"
        }
    ]"#;

    let mut file = File::create(test_file).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    drop(file);

    let storage = Storage::new(test_file);
    let tasks = storage.load_tasks();

    assert_eq!(tasks.len(), 1);
    assert!(tasks[0].description.contains("{braces}"));

    fs::remove_file(test_file).ok();
}

#[test]
fn test_escape_and_unescape_all_special_chars() {
    let test_file = "test_all_escapes.json";
    let storage = Storage::new(test_file);

    let mut tasks = Vec::new();
    tasks.push(Task::new(1, "Line\nbreak".to_string()));
    tasks.push(Task::new(2, "Tab\there".to_string()));
    tasks.push(Task::new(3, "Return\rchar".to_string()));

    storage.save_tasks(&tasks).ok();
    let loaded = storage.load_tasks();

    assert_eq!(loaded.len(), 3);
    assert_eq!(loaded[0].description, "Line\nbreak");
    assert_eq!(loaded[1].description, "Tab\there");
    assert_eq!(loaded[2].description, "Return\rchar");

    fs::remove_file(test_file).ok();
}

#[test]
fn test_extract_number_with_no_colon() {
    let test_file = "test_malformed_id.json";

    let json = r#"[
        {
            "id" 123,
            "description": "Missing colon",
            "status": "todo"
        }
    ]"#;

    let mut file = File::create(test_file).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    drop(file);

    let storage = Storage::new(test_file);
    let tasks = storage.load_tasks();

    assert_eq!(tasks.len(), 0);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_extract_string_with_insufficient_quotes() {
    let test_file = "test_bad_quotes.json";

    let json = r#"[
        {
            "id": 1,
            "description": "Only one quote,
            "status": "todo"
        }
    ]"#;

    let mut file = File::create(test_file).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    drop(file);

    let storage = Storage::new(test_file);
    let _tasks = storage.load_tasks();

    fs::remove_file(test_file).ok();
}

#[test]
fn test_task_with_newlines_and_tabs() {
    let test_file = "test_newlines_tabs.json";
    let storage = Storage::new(test_file);

    let mut tasks = Vec::new();
    tasks.push(Task::new(1, "Multi\nline\ntask".to_string()));
    tasks.push(Task::new(2, "Tab\t\tseparated".to_string()));

    storage.save_tasks(&tasks).ok();
    let loaded = storage.load_tasks();

    assert_eq!(loaded[0].description, "Multi\nline\ntask");
    assert_eq!(loaded[1].description, "Tab\t\tseparated");

    fs::remove_file(test_file).ok();
}

#[test]
fn test_json_not_starting_with_bracket() {
    let test_file = "test_no_bracket.json";

    let mut file = File::create(test_file).unwrap();
    file.write_all(b"{ not an array }").unwrap();
    drop(file);

    let storage = Storage::new(test_file);
    let tasks = storage.load_tasks();

    assert_eq!(tasks.len(), 0);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_json_not_ending_with_bracket() {
    let test_file = "test_no_end_bracket.json";

    let mut file = File::create(test_file).unwrap();
    file.write_all(b"[{ incomplete").unwrap();
    drop(file);

    let storage = Storage::new(test_file);
    let tasks = storage.load_tasks();

    assert_eq!(tasks.len(), 0);

    fs::remove_file(test_file).ok();
}

