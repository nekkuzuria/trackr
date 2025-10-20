use trackr::storage::Storage;
use trackr::task::{Task, TaskStatus};
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

#[test]
fn test_save_tasks_to_readonly_directory() {
    #[cfg(unix)]
    {
        let test_dir = "readonly_test_dir";
        fs::create_dir(test_dir).ok();

        let test_file = format!("{}/test.json", test_dir);
        let storage = Storage::new(&test_file);

        let mut perms = fs::metadata(test_dir).unwrap().permissions();
        perms.set_mode(0o444);
        fs::set_permissions(test_dir, perms).ok();

        let mut tasks = Vec::new();
        tasks.push(Task::new(1, "Test".to_string()));

        let result = storage.save_tasks(&tasks);
        assert!(result.is_err());

        let mut perms = fs::metadata(test_dir).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(test_dir, perms).ok();

        fs::remove_dir_all(test_dir).ok();
    }
}

#[test]
fn test_load_from_directory_not_file() {
    let test_dir = "test_is_dir";
    fs::create_dir(test_dir).ok();

    let storage = Storage::new(test_dir);
    let tasks = storage.load_tasks();

    assert_eq!(tasks.len(), 0);

    fs::remove_dir(test_dir).ok();
}

#[test]
fn test_parse_json_with_multiple_objects() {
    let test_file = "test_multiple_objects.json";

    let json = r#"[
        {
            "id": 1,
            "description": "First",
            "status": "todo"
        },
        {
            "id": 2,
            "description": "Second",
            "status": "in-progress"
        },
        {
            "id": 3,
            "description": "Third",
            "status": "done"
        }
    ]"#;

    let mut file = File::create(test_file).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    drop(file);

    let storage = Storage::new(test_file);
    let tasks = storage.load_tasks();

    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[1].id, 2);
    assert_eq!(tasks[2].id, 3);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_json_with_extra_whitespace() {
    let test_file = "test_whitespace_json.json";

    let json = r#"

    [

        {
            "id"    :    1   ,
            "description"   :   "Task"   ,
            "status"   :   "todo"
        }

    ]

    "#;

    let mut file = File::create(test_file).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    drop(file);

    let storage = Storage::new(test_file);
    let tasks = storage.load_tasks();

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].description, "Task");

    fs::remove_file(test_file).ok();
}

#[test]
fn test_empty_json_object() {
    let test_file = "test_empty_object.json";

    let json = r#"[
        {}
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
fn test_json_with_only_id() {
    let test_file = "test_only_id.json";

    let json = r#"[
        {
            "id": 1
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
fn test_json_with_only_description() {
    let test_file = "test_only_desc.json";

    let json = r#"[
        {
            "description": "Task without ID"
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
fn test_json_with_id_and_description_only() {
    let test_file = "test_no_status.json";

    let json = r#"[
        {
            "id": 1,
            "description": "No status field"
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
fn test_to_json_with_multiple_tasks() {
    let test_file = "test_json_multiple.json";
    let storage = Storage::new(test_file);

    let mut tasks = Vec::new();
    for i in 1..=5 {
        tasks.push(Task::with_status(
            i,
            format!("Task {}", i),
            if i % 2 == 0 { TaskStatus::Done } else { TaskStatus::Todo }
        ));
    }

    storage.save_tasks(&tasks).ok();

    let content = fs::read_to_string(test_file).unwrap();
    assert!(content.contains("\"id\": 1"));
    assert!(content.contains("\"id\": 5"));
    assert!(content.contains("\"status\": \"todo\""));
    assert!(content.contains("\"status\": \"done\""));

    fs::remove_file(test_file).ok();
}

#[test]
fn test_escape_all_characters() {
    let test_file = "test_all_escapes.json";
    let storage = Storage::new(test_file);

    let mut tasks = Vec::new();
    tasks.push(Task::new(1, "Backslash: \\ Newline: \n Tab: \t Return: \r".to_string()));

    storage.save_tasks(&tasks).ok();
    let loaded = storage.load_tasks();

    assert_eq!(loaded.len(), 1);
    assert!(loaded[0].description.contains("Backslash: \\"));
    assert!(loaded[0].description.contains("Newline: \n"));
    assert!(loaded[0].description.contains("Tab: \t"));

    fs::remove_file(test_file).ok();
}

#[test]
fn test_very_long_description() {
    let test_file = "test_long_desc.json";
    let storage = Storage::new(test_file);

    let long_desc = "A".repeat(10000);
    let mut tasks = Vec::new();
    tasks.push(Task::new(1, long_desc.clone()));

    storage.save_tasks(&tasks).ok();
    let loaded = storage.load_tasks();

    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].description.len(), 10000);

    fs::remove_file(test_file).ok();
}

#[test]
fn test_tasks_with_same_id() {
    let test_file = "test_duplicate_id.json";

    let json = r#"[
        {
            "id": 1,
            "description": "First",
            "status": "todo"
        },
        {
            "id": 1,
            "description": "Duplicate ID",
            "status": "done"
        }
    ]"#;

    let mut file = File::create(test_file).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    drop(file);

    let storage = Storage::new(test_file);
    let tasks = storage.load_tasks();

    assert_eq!(tasks.len(), 2);

    fs::remove_file(test_file).ok();
}

