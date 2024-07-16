use crate::todo::DueDate;
use crate::todo::NewTask;
use crate::todo::Priority;
use crate::todo::Task;
use crate::todo::TaskUpdate;
use crate::todo::TodoList;
use chrono::NaiveDate;
use std::io::{self, Write};

pub const DEFAULT_CATEGORY: &str = "General";

pub fn run_cli(todo_list: &mut TodoList) {
    loop {
        print!("Enter command (add/remove/list/get/update/categories/quit): ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        match command {
            "add" => add_task(todo_list),
            "remove" => remove_task(todo_list),
            "list" => list_tasks(todo_list),
            "get" => get_task(todo_list),
            "update" => update_task(todo_list),
            "categories" => list_categories(todo_list),
            "quit" => {
                quit();
                break;
            }
            _ => println!("Unknown command."),
        }
    }
}

fn add_task(todo_list: &mut TodoList) {
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

    let due_date = if date_string.is_empty() {
        DueDate::None
    } else {
        match NaiveDate::parse_from_str(date_string, "%Y-%m-%d") {
            Ok(date) => DueDate::On(date),
            Err(_) => {
                println!("Invalid date format. Setting due date to none.");
                DueDate::None
            }
        }
    };

    print!("Enter category (leave blank for general): ");
    io::stdout().flush().unwrap();
    let mut category = String::new();
    io::stdin().read_line(&mut category).unwrap();
    let category_string = category.trim().to_string();
    if category_string.is_empty() {
        category = DEFAULT_CATEGORY.to_string();
    } else {
        category = category_string;
    }
    let id = todo_list.add_task(NewTask {
        description,
        due_date,
        category,
        priority: Priority::Low,
    });
    println!("Task added with ID {}.", id);
}

fn remove_task(todo_list: &mut TodoList) {
    print!("Enter task ID: ");
    io::stdout().flush().unwrap();
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id: u32 = id.trim().parse().unwrap();
    if todo_list.remove_task(id).is_ok() {
        println!("Task removed.");
    } else {
        println!("Task not found.");
    }
}

fn list_tasks(todo_list: &TodoList) {
    print!("Enter category to list (leave blank for all): ");
    io::stdout().flush().unwrap();
    let mut category = String::new();
    io::stdin().read_line(&mut category).unwrap();
    let category = category.trim();
    let category = if category.is_empty() {
        None
    } else {
        Some(category)
    };

    let tasks = todo_list.list_tasks(category);
    for task in tasks {
        print_task_details(task)
    }
}

fn update_task(todo_list: &mut TodoList) {
    print!("Enter task ID: ");
    io::stdout().flush().unwrap();
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id: u32 = id.trim().parse().unwrap();

    if let Some(task) = todo_list.get_task(id) {
        print_task_details(task)
    } else {
        println!("Task not found.");
        return;
    }

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
                println!("Invalid date format. Task will be updated without a due date.");
                None
            }
        }
    } else {
        None
    };

    print!("Enter new category (leave blank to keep current): ");
    io::stdout().flush().unwrap();
    let mut category = String::new();
    io::stdin().read_line(&mut category).unwrap();
    let category_string = category.trim().to_string();
    let category = if category_string.is_empty() {
        None
    } else {
        Some(category_string)
    };

    let task_update = TaskUpdate {
        description,
        due_date,
        category,
    };

    if todo_list.update_task(id, task_update).is_ok() {
        println!("Task id {} updated.", id);
    }
}

fn list_categories(todo_list: &TodoList) {
    let categories = todo_list.get_categories();
    for category in categories {
        println!("{}", category);
    }
}

fn get_task(todo_list: &TodoList) {
    print!("Enter task ID: ");
    io::stdout().flush().unwrap();
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id: u32 = id.trim().parse().unwrap();

    if let Some(task) = todo_list.get_task(id) {
        print_task_details(task)
    } else {
        println!("Task not found.");
    }
}

fn print_task_details(task: &Task) {
    println!("Task details:");
    println!("ID: {}", task.id);
    println!("Description: {}", task.description);

    let due_date_str = match &task.due_date {
        DueDate::On(d) | DueDate::Before(d) => d.format("%Y-%m-%d").to_string(),
        DueDate::None => "None".to_string(),
    };
    println!("Due Date: {}", due_date_str);

    println!("Category: {}", task.category);
}

fn quit() {
    println!("Goodbye!");
}
