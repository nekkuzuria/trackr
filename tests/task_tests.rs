use trackr::task::{Task, TaskStatus};

#[test]
fn test_task_creation() {
    let task = Task::new(1, "Learn Rust".to_string());
    assert_eq!(task.id, 1);
    assert_eq!(task.description, "Learn Rust");
    assert_eq!(task.status, TaskStatus::Todo);
}

#[test]
fn test_task_with_status() {
    let task = Task::with_status(2, "Deploy app".to_string(), TaskStatus::InProgress);
    assert_eq!(task.id, 2);
    assert_eq!(task.description, "Deploy app");
    assert_eq!(task.status, TaskStatus::InProgress);
}

#[test]
fn test_task_status_from_str() {
    assert_eq!(TaskStatus::from_str("todo"), Some(TaskStatus::Todo));
    assert_eq!(TaskStatus::from_str("in-progress"), Some(TaskStatus::InProgress));
    assert_eq!(TaskStatus::from_str("done"), Some(TaskStatus::Done));
    assert_eq!(TaskStatus::from_str("invalid"), None);
}

#[test]
fn test_task_status_from_str_case_insensitive() {
    assert_eq!(TaskStatus::from_str("TODO"), Some(TaskStatus::Todo));
    assert_eq!(TaskStatus::from_str("In-Progress"), Some(TaskStatus::InProgress));
    assert_eq!(TaskStatus::from_str("DONE"), Some(TaskStatus::Done));
    assert_eq!(TaskStatus::from_str("ToDo"), Some(TaskStatus::Todo));
}

#[test]
fn test_task_status_to_string() {
    assert_eq!(TaskStatus::Todo.to_string(), "todo");
    assert_eq!(TaskStatus::InProgress.to_string(), "in-progress");
    assert_eq!(TaskStatus::Done.to_string(), "done");
}

#[test]
fn test_task_status_emoji() {
    assert_eq!(TaskStatus::Todo.emoji(), "📝");
    assert_eq!(TaskStatus::InProgress.emoji(), "⚡");
    assert_eq!(TaskStatus::Done.emoji(), "✨");
}

#[test]
fn test_task_status_display() {
    let status = TaskStatus::Todo;
    assert_eq!(format!("{}", status), "todo");

    let status2 = TaskStatus::InProgress;
    assert_eq!(format!("{}", status2), "in-progress");

    let status3 = TaskStatus::Done;
    assert_eq!(format!("{}", status3), "done");
}

#[test]
fn test_task_clone() {
    let task1 = Task::new(1, "Original".to_string());
    let task2 = task1.clone();
    assert_eq!(task1.id, task2.id);
    assert_eq!(task1.description, task2.description);
}

#[test]
fn test_task_status_equality() {
    let status1 = TaskStatus::Todo;
    let status2 = TaskStatus::Todo;
    let status3 = TaskStatus::Done;
    assert_eq!(status1, status2);
    assert_ne!(status1, status3);
}

#[test]
fn test_task_with_different_ids() {
    let task1 = Task::new(1, "Task 1".to_string());
    let task2 = Task::new(2, "Task 2".to_string());
    let task3 = Task::new(100, "Task 100".to_string());

    assert_eq!(task1.id, 1);
    assert_eq!(task2.id, 2);
    assert_eq!(task3.id, 100);
}

#[test]
fn test_task_with_long_description() {
    let long_desc = "This is a very long task description that contains multiple words and should be handled correctly by the task system without any issues".to_string();
    let task = Task::new(1, long_desc.clone());
    assert_eq!(task.description, long_desc);
}

#[test]
fn test_task_with_empty_description() {
    let task = Task::new(1, "".to_string());
    assert_eq!(task.description, "");
    assert_eq!(task.id, 1);
    assert_eq!(task.status, TaskStatus::Todo);
}

#[test]
fn test_task_status_transitions() {
    let mut task = Task::new(1, "Test task".to_string());
    assert_eq!(task.status, TaskStatus::Todo);

    task.status = TaskStatus::InProgress;
    assert_eq!(task.status, TaskStatus::InProgress);

    task.status = TaskStatus::Done;
    assert_eq!(task.status, TaskStatus::Done);
}

#[test]
fn test_multiple_tasks_creation() {
    let tasks: Vec<Task> = (1..=10)
        .map(|i| Task::new(i, format!("Task {}", i)))
        .collect();

    assert_eq!(tasks.len(), 10);
    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[9].id, 10);
    assert_eq!(tasks[4].description, "Task 5");
}

