// src/main.rs
mod cli;
mod todo;

use todo::TodoList;

fn main() {
    let mut todo_list = TodoList::new();
    cli::run_cli(&mut todo_list);
}
