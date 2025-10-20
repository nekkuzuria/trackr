use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl TaskStatus {
    pub fn from_str(s: &str) -> Option<TaskStatus> {
        match s.to_lowercase().as_str() {
            "todo" => Some(TaskStatus::Todo),
            "in-progress" => Some(TaskStatus::InProgress),
            "done" => Some(TaskStatus::Done),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TaskStatus::Todo => "todo".to_string(),
            TaskStatus::InProgress => "in-progress".to_string(),
            TaskStatus::Done => "done".to_string(),
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            TaskStatus::Todo => "📝",
            TaskStatus::InProgress => "⚡",
            TaskStatus::Done => "✨",
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(id: u32, description: String) -> Task {
        Task {
            id,
            description,
            status: TaskStatus::Todo,
        }
    }

    pub fn with_status(id: u32, description: String, status: TaskStatus) -> Task {
        Task {
            id,
            description,
            status,
        }
    }
}


