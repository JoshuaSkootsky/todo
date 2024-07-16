use crate::todo::DueDate;
use crate::todo::NewTask;
use crate::todo::Priority;
use crate::todo::Task;
use crate::todo::TaskUpdate;
use crate::todo::TodoError;
use crate::todo::TodoList;
use chrono::NaiveDate;
use crossterm::{
    cursor::MoveUp,
    event::{self, Event, KeyCode},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{self, stdout, BufRead, BufReader, Write};
use std::path::Path;

// DEFAULT_CATEGORY is the default category for new tasks
pub const DEFAULT_CATEGORY: &str = "General";

// FILENAME_HISTORY is the name of the file that stores the history of filenames
const FILENAME_HISTORY: &str = ".todo_filenames.txt";

const AFFIRMATIVE_RESPONSES: [&str; 9] = [
    "y", "yes", "yeah", "yep", "ok", "sure", "true", "accept", "aff",
];

struct FilenameTracker {
    filenames: HashSet<String>,
    tracking_enabled: bool,
}

impl FilenameTracker {
    fn new() -> io::Result<Self> {
        let mut filenames = HashSet::new();
        if Path::new(FILENAME_HISTORY).exists() {
            let file = File::open(FILENAME_HISTORY)?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                filenames.insert(line?);
            }
        }
        Ok(FilenameTracker {
            filenames,
            tracking_enabled: false,
        })
    }

    fn add(&mut self, filename: &str) -> io::Result<()> {
        if self.filenames.insert(filename.to_string()) {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(FILENAME_HISTORY)?;
            writeln!(file, "{}", filename)?;
        }
        Ok(())
    }

    fn list(&self) -> &HashSet<String> {
        &self.filenames
    }

    fn enable_tracking(&mut self) {
        self.tracking_enabled = true;
    }
}

pub fn run_cli(todo_list: &mut TodoList) -> io::Result<()> {
    let mut filename_tracker = FilenameTracker::new()?;

    println!("Welcome to the Todo List CLI!");
    print!("Would you like to enable filename tracking? (y/n): ");
    io::stdout().flush()?;
    let mut response = String::new();
    io::stdin().read_line(&mut response)?;

    if AFFIRMATIVE_RESPONSES.contains(&response.trim().to_lowercase().as_str()) {
        filename_tracker.enable_tracking();
        println!("Filename tracking enabled.");
    } else {
        println!("Filename tracking disabled. You can enable it later by using the 'enable_tracking' command.");
    }

    loop {
        print!("Enter command (add/remove/list/get/update/categories/save/load/enable_tracking/quit): ");
        io::stdout().flush()?;

        let mut command = String::new();
        io::stdin().read_line(&mut command)?;
        let command = command.trim();

        match command {
            "add" => add_task(todo_list),
            "remove" => remove_task(todo_list),
            "list" => list_tasks(todo_list),
            "get" => get_task(todo_list),
            "update" => update_task(todo_list),
            "categories" => list_categories(todo_list),
            "save" => save_list(todo_list, &mut filename_tracker)?,
            "load" => match load_list(&filename_tracker) {
                Ok(loaded_list) => *todo_list = loaded_list,
                Err(e) => println!("Error loading list: {}. Continuing with current list.", e),
            },
            "enable_tracking" => {
                filename_tracker.enable_tracking();
                println!("Filename tracking enabled.");
            }
            "quit" => {
                quit();
                break;
            }
            _ => println!("Unknown command."),
        }
    }
    Ok(())
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

// save_list to local file storage
fn save_list(todo_list: &TodoList, filename_tracker: &mut FilenameTracker) -> io::Result<()> {
    let filenames: Vec<_> = filename_tracker.list().iter().cloned().collect();

    if filename_tracker.tracking_enabled && !filenames.is_empty() {
        match select_file(&filenames, "Select a file to save to:") {
            Ok(Some(filename)) => return save_to_file(todo_list, &filename, filename_tracker),
            Ok(None) => {} // User chose to enter a new filename
            Err(e) => return Err(e),
        }
    }

    // If tracking is disabled, the list is empty, or user chose to enter a new filename
    print!("Enter filename to save: ");
    io::stdout().flush()?;
    let mut filename = String::new();
    io::stdin().read_line(&mut filename)?;
    let filename = filename.trim();

    save_to_file(todo_list, filename, filename_tracker)
}

// load_list from local file storage
fn load_list(filename_tracker: &FilenameTracker) -> Result<TodoList, TodoError> {
    let filenames: Vec<_> = filename_tracker.list().iter().cloned().collect();

    if filename_tracker.tracking_enabled && !filenames.is_empty() {
        match select_file(&filenames, "Select a file to load:") {
            Ok(Some(filename)) => return TodoList::load_from_file(&filename),
            Ok(None) => {} // User chose to enter a new filename
            Err(e) => return Err(TodoError::Io(e)),
        }
    }

    // If tracking is disabled, the list is empty, or user chose to enter a new filename
    print!("Enter filename to load: ");
    io::stdout().flush().unwrap();
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();

    TodoList::load_from_file(filename)
}

fn quit() {
    println!("Goodbye!");
}

// select_file prompts the user to select a file from a list of filenames
fn select_file(filenames: &[String], prompt: &str) -> io::Result<Option<String>> {
    let mut selected = 0;
    let mut stdout = stdout();

    println!("{}", prompt);
    for filename in filenames.iter() {
        println!("  {}", filename);
    }
    println!("  Enter new filename");

    loop {
        // Move cursor to the start of the list
        stdout.execute(MoveUp(filenames.len() as u16 + 1))?;

        // Redraw the list with the current selection
        for (i, filename) in filenames.iter().enumerate() {
            stdout.execute(Clear(ClearType::CurrentLine))?;
            if i == selected {
                println!("> {}", filename);
            } else {
                println!("  {}", filename);
            }
        }
        stdout.execute(Clear(ClearType::CurrentLine))?;
        println!(
            "{}",
            if selected == filenames.len() {
                "> Enter new filename"
            } else {
                "  Enter new filename"
            }
        );

        stdout.flush()?;

        // Handle key events
        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < filenames.len() {
                        selected += 1;
                    }
                }
                KeyCode::Enter => {
                    // Clear the selection UI
                    stdout.execute(MoveUp(filenames.len() as u16 + 1))?;
                    for _ in 0..=filenames.len() {
                        stdout.execute(Clear(ClearType::CurrentLine))?;
                        println!();
                    }
                    stdout.execute(MoveUp(filenames.len() as u16 + 2))?;
                    stdout.execute(Clear(ClearType::CurrentLine))?;
                    println!("{}", prompt);

                    return if selected < filenames.len() {
                        println!("Selected: {}", filenames[selected]);
                        Ok(Some(filenames[selected].clone()))
                    } else {
                        println!("Enter new filename:");
                        Ok(None)
                    };
                }
                KeyCode::Esc => {
                    // Clear the selection UI
                    stdout.execute(MoveUp(filenames.len() as u16 + 1))?;
                    for _ in 0..=filenames.len() {
                        stdout.execute(Clear(ClearType::CurrentLine))?;
                        println!();
                    }
                    stdout.execute(MoveUp(filenames.len() as u16 + 2))?;
                    stdout.execute(Clear(ClearType::CurrentLine))?;
                    println!("Operation cancelled.");
                    return Ok(None);
                }
                _ => {}
            }
        }
    }
}

// save_to_file saves the todo list to a file
fn save_to_file(
    todo_list: &TodoList,
    filename: &str,
    filename_tracker: &mut FilenameTracker,
) -> io::Result<()> {
    match todo_list.save_to_file(filename) {
        Ok(_) => {
            println!("Todo list saved successfully to {}.", filename);
            filename_tracker.add(filename)?;
            Ok(())
        }
        Err(e) => {
            println!("Failed to save todo list: {}", e);
            Err(io::Error::new(io::ErrorKind::Other, e))
        }
    }
}
