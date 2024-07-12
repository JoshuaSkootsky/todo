use std::collections::HashMap;
use std::io::{self, Write};

struct TodoList {
    tasks: HashMap<u32, String>,
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
    fn add_task(&mut self, task: String) -> u32 {
        let id = self.next_id;
        self.tasks.insert(self.next_id, task);
        self.next_id += 1;
        id // return the id of the new task
    }

    // remove a task from the todo list by id
    fn remove_task(&mut self, id: u32) -> bool {
        self.tasks.remove(&id).is_some()
    }

    // return a list of tasks
    fn list_tasks(&self) -> Vec<(u32, String)> {
        self.tasks.iter().map(|(&id, task)| (id, task.clone())).collect() // TODO understand this
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        print!("Enter command (add/remove/list/quit): ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        match command {
            "add" => {
                print!("Enter task: ");
                io::stdout().flush().unwrap();
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                todo_list.add_task(task.trim().to_string());
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
                for (id, task) in tasks {
                    println!("{}: {}", id, task);
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
        let id = todo_list.add_task("Learn Rust".to_string());
        assert_eq!(id, 1);
        assert_eq!(todo_list.tasks.len(), 1);
        assert_eq!(todo_list.tasks.get(&id,), Some(&"Learn Rust".to_string()));
    }

    #[test]
    fn test_remove_task() {
        let mut todo_list = TodoList::new();
        let id = todo_list.add_task("Learn Rust".to_string());
        assert!(todo_list.remove_task(id));
        assert_eq!(todo_list.tasks.len(), 0);
        assert!(!todo_list.remove_task(id));
    }

    #[test]
    fn test_list_tasks() {
        let mut todo_list = TodoList::new();
        todo_list.add_task("Learn Rust".to_string());
        todo_list.add_task("Write tests".to_string());
        let tasks = todo_list.list_tasks();
        assert_eq!(tasks.len(), 2);
        assert!(tasks.iter().any(|(id, task)| *id == 1 && task == "Learn Rust"));
        assert!(tasks.iter().any(|(id, task)| *id == 2 && task == "Write tests"));
    }
}
