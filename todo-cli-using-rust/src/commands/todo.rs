use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;

use crate::config::file;

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Pending,
    InProgress,
    Done,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Status::Pending => "Pending",
                Status::InProgress => "In Progress",
                Status::Done => "Done",
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u8,
    title: String,
    status: Status,
}

impl Task {
    fn new(id: u8, title: &str) -> Self {
        Self {
            id: id,
            title: title.to_string(),
            status: Status::Pending,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    tasks: Vec<Task>,
}

pub trait TodoService {
    fn add(&mut self, title: &str) -> Result<(), &str>;
    fn list(&self) -> Result<Vec<String>, &str>;
    fn change_status(&mut self, task_id: u8, task_status: &str) -> Result<(), &str>;
    fn delete(&mut self, task_id: u8) -> Result<(), &str>;
}

impl TodoService for Todo {
    fn add(&mut self, title: &str) -> Result<(), &str> {
        Todo::add(self, title)
    }

    fn list(&self) -> Result<Vec<String>, &str> {
        Todo::list(self)
    }

    fn change_status(&mut self, task_id: u8, task_status: &str) -> Result<(), &str> {
        Todo::change_status(self, task_id, task_status)
    }

    fn delete(&mut self, task_id: u8) -> Result<(), &str> {
        Todo::delete(self, task_id)
    }
}

impl Drop for Todo {
    fn drop(&mut self) {
        if let Err(e) = file::save(self) {
            println!("Error: {}", e);
        }
    }
}

impl Todo {
    pub fn new() -> Self {
        Self::load()
    }

    fn load() -> Self {
        match file::load() {
            Ok(content) if !content.trim().is_empty() => {
                match serde_json::from_str::<Todo>(&content) {
                    Ok(todo) => todo,
                    Err(e) => {
                        eprintln!("Failed to parse persisted tasks: {}", e);
                        Self { tasks: vec![] }
                    }
                }
            }
            _ => Self { tasks: vec![] },
        }
    }

    pub fn add(&mut self, title: &str) -> Result<(), &str> {
        if title.is_empty() {
            return Err("Title cannot be empty");
        }

        let task_id: u8 = self.tasks.len() as u8 + 1;
        let new_task = Task::new(task_id, title);
        self.tasks.push(new_task);
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<String>, &str> {
        let tasks = self
            .tasks
            .iter()
            .map(|task| format!("{}. {} - {}", task.id, task.title, task.status))
            .collect();
        Ok(tasks)
    }

    pub fn change_status(&mut self, task_id: u8, task_status: &str) -> Result<(), &str> {
        let status = match task_status {
            "pending" => Status::Pending,
            "in-progress" => Status::InProgress,
            "done" => Status::Done,
            _ => return Err("Invalid Status"),
        };

        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == task_id) {
            task.status = status;
            Ok(())
        } else {
            Err("Task not found")
        }
    }

    pub fn delete(&mut self, task_id: u8) -> Result<(), &str> {
        if let Some(index) = self.tasks.iter().position(|task| task.id == task_id) {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err("Task not found")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut todo = Todo::new();
        let result = todo.add("Task 1");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_add_task_with_empty_title() {
        let mut todo = Todo::new();
        let result = todo.add("");
        assert_eq!(result, Err("Title cannot be empty"));
    }

    #[test]
    fn test_list_tasks() {
        let mut todo = Todo::new();
        let _ = todo.add("Task 1");
        let _ = todo.add("Task 2");
        let _ = todo.add("Task 3");

        let result = todo.list();
        assert_eq!(
            result,
            Ok(vec![
                "1. Task 1 - Pending".to_string(),
                "2. Task 2 - Pending".to_string(),
                "3. Task 3 - Pending".to_string(),
            ])
        );
    }

    #[test]
    fn test_change_status() {
        let mut todo = Todo::new();
        let _ = todo.add("Task 1");
        let _ = todo.add("Task 2");
        let _ = todo.add("Task 3");

        let result = todo.change_status(1, "done");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_change_status_with_invalid_status() {
        let mut todo = Todo::new();
        let _ = todo.add("Task 1");
        let result = todo.change_status(1, "invalid");
        assert_eq!(result, Err("Invalid Status"));
    }

    #[test]
    fn test_change_status_with_invalid_task_id() {
        let mut todo = Todo::new();
        let _ = todo.add("Task 1");
        let result = todo.change_status(2, "done");
        assert_eq!(result, Err("Task not found"));
    }

    #[test]
    fn test_delete_task() {
        let mut todo = Todo::new();
        let _ = todo.add("Task 1");
        let _ = todo.add("Task 2");
        let _ = todo.add("Task 3");

        let result = todo.delete(1);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_delete_task_with_invalid_task_id() {
        let mut todo = Todo::new();
        let _ = todo.add("Task 1");
        let result = todo.delete(2);
        assert_eq!(result, Err("Task not found"));
    }
}
