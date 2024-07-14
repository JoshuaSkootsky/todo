use chrono::NaiveDate;
use std::collections::HashMap;
use std::io::{self, Write};

struct Task {
    id: u32,
    description: String,
    due_date: Option<NaiveDate>,
}

struct TodoList {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    // add a new task to the list and return its id
    // return the id to make it testable, and to be able to remove  the task by id
    fn add_task(&mut self, description: String, due_date: Option<NaiveDate>) -> u32 {
        let id = self.next_id;
        self.tasks.insert(
            self.next_id,
            Task {
                id,
                description,
                due_date,
            },
        );
        self.next_id += 1;
        id // return the id of the new task
    }

    // remove a task from the todo list by id
    fn remove_task(&mut self, id: u32) -> bool {
        self.tasks.remove(&id).is_some()
    }

    // return a list of tasks
    fn list_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn get_task(&self, id: u32) -> Option<&Task> {
        self.tasks.get(&id)
    }

    fn update_task(
        &mut self,
        id: u32,
        description: &Option<String>,
        due_date: Option<NaiveDate>,
    ) -> bool {
        if let Some(task) = self.tasks.get_mut(&id) {
            if let Some(desc) = description {
                task.description = desc.to_string();
            }
            if let Some(date) = due_date {
                task.due_date = Some(date);
            }
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        print!("Enter command (add/remove/list/update/quit): ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        match command {
            "add" => {
                print!("Enter task: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();

                let description = description.trim().to_string();

                print!("Enter due date (YYYY-MM-DD, leave blank for no date): ");
                io::stdout().flush().unwrap();
                let mut date_string = String::new();

                io::stdin().read_line(&mut date_string).unwrap();
                let date_string = date_string.trim();

                let due_date = if !date_string.is_empty() {
                    match NaiveDate::parse_from_str(date_string, "%Y-%m-%d") {
                        Ok(date) => Some(date),
                        Err(_) => {
                            println!("Invalid date format. Task will be added without a due date.");
                            None
                        }
                    }
                } else {
                    None
                };

                let id = todo_list.add_task(description, due_date);
                println!("Task added with ID {}.", id);
            }
            "remove" => {
                print!("Enter task ID: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: u32 = id.trim().parse().unwrap();
                if todo_list.remove_task(id) {
                    println!("Task removed.");
                } else {
                    println!("Task not found.");
                }
            }
            "list" => {
                let tasks = todo_list.list_tasks(); // TODO how would this work with multiple todo lists
                for task in tasks {
                    let date_str = task.due_date.map_or("No due date".to_string(), |d| {
                        d.format("%Y-%m-%d").to_string()
                    });
                    println!(
                        "ID: {}, Description: {}, Due Date: {}",
                        task.id, task.description, date_str
                    );
                }
            }
            "update" => {
                print!("Enter task ID: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: u32 = id.trim().parse().unwrap();

                print!("Enter new description (leave blank to keep current): ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                let description = if description.trim().is_empty() {
                    None
                } else {
                    Some(description.trim().to_string())
                };

                print!("Enter new due date (YYYY-MM-DD, leave blank to keep current): ");
                io::stdout().flush().unwrap();
                let mut date_string = String::new();
                io::stdin().read_line(&mut date_string).unwrap();
                let date_string = date_string.trim();
                let due_date = if !date_string.is_empty() {
                    match NaiveDate::parse_from_str(date_string, "%Y-%m-%d") {
                        Ok(date) => Some(date),
                        Err(_) => {
                            println!(
                                "Invalid date format. Task will be updated without a due date."
                            );
                            None
                        }
                    }
                } else {
                    None
                };

                if todo_list.update_task(id, &description, due_date) {
                    println!("Task id {} updated. {:?}", id, description);
                } else {
                    println!("Task not found.");
                }
            }
            "quit" => break,
            _ => println!("Unknown command."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut todo_list = TodoList::new(); // TODO why is this mutable?
        let id = todo_list.add_task("Learn Rust".to_string(), None);
        assert_eq!(id, 1);
        assert_eq!(todo_list.tasks.len(), 1);
        let task = todo_list.get_task(id).unwrap();
        assert_eq!(task.id, id);
        assert_eq!(task.description, "Learn Rust");
        assert_eq!(task.due_date, None);
    }

    #[test]
    fn test_remove_task() {
        let mut todo_list = TodoList::new();
        let id = todo_list.add_task("Learn Rust".to_string(), None);
        assert!(todo_list.remove_task(id));
        assert_eq!(todo_list.tasks.len(), 0);
        assert!(!todo_list.remove_task(id));
    }

    #[test]
    fn test_list_tasks() {
        let mut todo_list = TodoList::new();
        todo_list.add_task("Learn Rust".to_string(), None);
        todo_list.add_task("Write tests".to_string(), None);
        let tasks = todo_list.list_tasks();
        assert_eq!(tasks.len(), 2);
        assert!(tasks
            .iter()
            .any(|task| task.id == 1 && task.description == "Learn Rust"));
        assert!(tasks
            .iter()
            .any(|task| task.id == 2 && task.description == "Write tests"));
    }

    #[test]
    fn test_update_task_preserve_due_date() {
        let mut todo_list = TodoList::new();
        let original_date = NaiveDate::from_ymd_opt(2023, 6, 1).unwrap();
        let id = todo_list.add_task("Original task".to_string(), Some(original_date));

        // Update only the description
        todo_list.update_task(id, &Some("Updated task".to_string()), None);

        let updated_task = todo_list.get_task(id).unwrap();
        assert_eq!(updated_task.description, "Updated task");
        assert_eq!(updated_task.due_date, Some(original_date));

        // Update only the due date
        let new_date = NaiveDate::from_ymd_opt(2023, 7, 1).unwrap();
        todo_list.update_task(id, &None, Some(new_date));

        let updated_task = todo_list.get_task(id).unwrap();
        assert_eq!(updated_task.description, "Updated task");
        assert_eq!(updated_task.due_date, Some(new_date));

        // Update neither description nor due date
        todo_list.update_task(id, &None, None);

        let updated_task = todo_list.get_task(id).unwrap();
        assert_eq!(updated_task.description, "Updated task");
        assert_eq!(updated_task.due_date, Some(new_date));
    }
}
