use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use crate::task::{Task, TaskStatus};

pub struct Storage {
    pub file_path: String,
}

impl Storage {
    pub fn new(file_path: &str) -> Storage {
        Storage {
            file_path: file_path.to_string(),
        }
    }

    pub fn load_tasks(&self) -> Vec<Task> {
        if !Path::new(&self.file_path).exists() {
            return Vec::new();
        }

        let mut file = match File::open(&self.file_path) {
            Ok(f) => f,
            Err(_) => return Vec::new(),
        };

        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_err() {
            return Vec::new();
        }

        if contents.trim().is_empty() {
            return Vec::new();
        }

        self.parse_json(&contents)
    }

    pub fn save_tasks(&self, tasks: &[Task]) -> Result<(), String> {
        let json = self.to_json(tasks);

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)
            .map_err(|e| format!("Failed to open file: {}", e))?;

        file.write_all(json.as_bytes())
            .map_err(|e| format!("Failed to write to file: {}", e))?;

        Ok(())
    }

    fn to_json(&self, tasks: &[Task]) -> String {
        let mut json = String::from("[\n");

        for (i, task) in tasks.iter().enumerate() {
            json.push_str("  {\n");
            json.push_str(&format!("    \"id\": {},\n", task.id));
            json.push_str(&format!("    \"description\": \"{}\",\n",
                self.escape_json(&task.description)));
            json.push_str(&format!("    \"status\": \"{}\"\n", task.status.to_string()));
            json.push_str("  }");

            if i < tasks.len() - 1 {
                json.push(',');
            }
            json.push('\n');
        }

        json.push_str("]\n");
        json
    }

    fn parse_json(&self, json: &str) -> Vec<Task> {
        let mut tasks = Vec::new();
        let trimmed = json.trim();

        if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
            return tasks;
        }

        let content = &trimmed[1..trimmed.len() - 1];
        let mut current_object = String::new();
        let mut brace_count = 0;

        for ch in content.chars() {
            if ch == '{' {
                brace_count += 1;
                current_object.push(ch);
            } else if ch == '}' {
                current_object.push(ch);
                brace_count -= 1;

                if brace_count == 0 {
                    if let Some(task) = self.parse_task_object(&current_object) {
                        tasks.push(task);
                    }
                    current_object.clear();
                }
            } else if brace_count > 0 {
                current_object.push(ch);
            }
        }

        tasks
    }

    fn parse_task_object(&self, obj: &str) -> Option<Task> {
        let mut id: Option<u32> = None;
        let mut description: Option<String> = None;
        let mut status: Option<TaskStatus> = None;

        for line in obj.lines() {
            let line = line.trim();

            if line.contains("\"id\"") {
                if let Some(value) = self.extract_number(line) {
                    id = Some(value);
                }
            } else if line.contains("\"description\"") {
                if let Some(value) = self.extract_string(line) {
                    description = Some(value);
                }
            } else if line.contains("\"status\"") {
                if let Some(value) = self.extract_string(line) {
                    status = TaskStatus::from_str(&value);
                }
            }
        }

        if let (Some(id), Some(description), Some(status)) = (id, description, status) {
            Some(Task::with_status(id, description, status))
        } else {
            None
        }
    }

    fn extract_number(&self, line: &str) -> Option<u32> {
        if let Some(colon_pos) = line.find(':') {
            let value_part = &line[colon_pos + 1..];
            let cleaned: String = value_part.chars()
                .filter(|c| c.is_numeric())
                .collect();
            cleaned.parse().ok()
        } else {
            None
        }
    }

    fn extract_string(&self, line: &str) -> Option<String> {
        let parts: Vec<&str> = line.split('"').collect();
        if parts.len() >= 4 {
            Some(self.unescape_json(parts[3]))
        } else {
            None
        }
    }

    fn escape_json(&self, s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
    }

    fn unescape_json(&self, s: &str) -> String {
        s.replace("\\\"", "\"")
            .replace("\\\\", "\\")
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t")
    }
}

