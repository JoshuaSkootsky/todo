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

    fn add_task(&mut self, task: String) {
        self.tasks.insert(self.next_id, task);
        self.next_id += 1;
    }

    fn remove_task(&mut self, id: u32) -> bool {
        self.tasks.remove(&id).is_some()
    }

    fn list_tasks(&self) {
        for (id, task) in &self.tasks {
            println!("{}: {}", id, task);
        }
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
            "list" => todo_list.list_tasks(),
            "quit" => break,
            _ => println!("Unknown command."),
        }
    }
}