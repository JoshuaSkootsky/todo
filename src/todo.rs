// src/todo.rs
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use thiserror::Error;

// Task is a thing to do and its details
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub due_date: DueDate,
    pub category: String,
    pub priority: Priority,
}

// NewTask is the information required to make a new Task
pub struct NewTask {
    pub description: String,
    pub due_date: DueDate,
    pub category: String,
    pub priority: Priority,
}

// TaskUpdate represents an update from a user to a task
pub struct TaskUpdate {
    pub description: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub category: Option<String>,
}
// Priority is a priority level for a task
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DueDate {
    On(NaiveDate),
    Before(NaiveDate),
    None,
}

// Status is a status for a task
#[derive(Debug, PartialEq)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Task not found")]
    TaskNotFound,
}

type Result<T> = std::result::Result<T, TodoError>;

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }
    // get_task returns a reference to a task with the given ID
    pub fn get_task(&self, id: u32) -> Option<&Task> {
        self.tasks.get(&id)
    }

    // add_task adds a new task to the list
    pub fn add_task(&mut self, new_task: NewTask) -> u32 {
        let id = self.next_id;

        self.tasks.insert(
            id,
            Task {
                id,
                description: new_task.description,
                due_date: new_task.due_date,
                category: new_task.category,
                priority: new_task.priority,
            },
        );
        self.next_id += 1;
        id
    }

    pub fn remove_task(&mut self, id: u32) -> Result<()> {
        self.tasks.remove(&id).ok_or(TodoError::TaskNotFound)?;
        Ok(())
    }

    pub fn list_tasks(&self, category: Option<&str>) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| category.map_or(true, |c| task.category == c))
            .collect()
    }

    pub fn update_task(&mut self, id: u32, task_update: TaskUpdate) -> Result<()> {
        let task = self.tasks.get_mut(&id).ok_or(TodoError::TaskNotFound)?;

        if let Some(desc) = task_update.description {
            task.description = desc.to_string();
        }
        if let Some(date) = task_update.due_date {
            task.due_date = DueDate::On(date)
        }
        if let Some(cat) = task_update.category {
            task.category = cat;
        }

        Ok(())
    }

    pub fn get_categories(&self) -> Vec<String> {
        self.tasks
            .values()
            .map(|task| task.category.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }

    pub fn save_to_file(&self, filename: &str) -> Result<()> {
        let json = serde_json::to_string(self)?;
        fs::write(filename, json)?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> Result<Self> {
        let json = fs::read_to_string(filename)?;
        let todo_list: TodoList = serde_json::from_str(&json)?;
        Ok(todo_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut list = TodoList::new();
        let id = list.add_task(NewTask {
            description: "Test task".to_string(),
            due_date: DueDate::None,
            category: "Test".to_string(),
            priority: Priority::Low,
        });
        assert_eq!(id, 1);
    }

    #[test]
    fn test_get_task() {
        let mut list = TodoList::new();
        let id = list.add_task(NewTask {
            description: "Test task".to_string(),
            due_date: DueDate::None,
            category: "Test".to_string(),
            priority: Priority::High,
        });
        let task = list.get_task(id).unwrap();
        assert_eq!(task.id, id);
        assert_eq!(task.description, "Test task");
        match task.due_date {
            DueDate::None => {}
            _ => panic!("Expected DueDate::None"),
        }
        assert_eq!(task.category, "Test");
    }

    #[test]
    fn test_remove_task() {
        let mut list = TodoList::new();
        let id = list.add_task(NewTask {
            description: "Test task 1".to_string(),
            due_date: DueDate::None,
            category: "Test".to_string(),
            priority: Priority::Low,
        });

        assert!(list.remove_task(id).is_ok());
        assert!(list.get_task(id).is_none());
        assert!(!list.remove_task(id).is_ok());
    }

    #[test]
    fn test_update_task() {
        let mut list = TodoList::new();
        let id = list.add_task(NewTask {
            description: "Test task".to_string(),
            due_date: DueDate::None,
            category: "Test".to_string(),
            priority: Priority::Low,
        });

        let new_description = Some("Updated task".to_string());
        let new_due_date = Some(NaiveDate::from_ymd(2023, 12, 31));
        let new_category = Some("Updated".to_string());

        let new_task_update = TaskUpdate {
            description: new_description,
            due_date: new_due_date,
            category: new_category,
        };

        assert!(list.update_task(id, new_task_update).is_ok());

        let task = list.get_task(id).unwrap();
        assert_eq!(task.description, "Updated task");
        match task.due_date {
            DueDate::On(date) => assert_eq!(date, NaiveDate::from_ymd(2023, 12, 31)),
            _ => panic!("Expected DueDate::On"),
        }
        assert_eq!(task.category, "Updated");
    }

    #[test]
    fn test_get_categories() {
        let mut list = TodoList::new();
        list.add_task(NewTask {
            description: "Test task 1".to_string(),
            due_date: DueDate::None,
            category: "Work".to_string(),
            priority: Priority::Low,
        });
        list.add_task(NewTask {
            description: "Test task 2".to_string(),
            due_date: DueDate::None,
            category: "Personal".to_string(),
            priority: Priority::High,
        });
        list.add_task(NewTask {
            description: "Test task 3".to_string(),
            due_date: DueDate::None,
            category: "Work".to_string(),
            priority: Priority::Medium,
        });

        let categories = list.get_categories();
        assert_eq!(categories.len(), 2);
        assert!(categories.contains(&"Work".to_string()));
        assert!(categories.contains(&"Personal".to_string()));
    }
}
