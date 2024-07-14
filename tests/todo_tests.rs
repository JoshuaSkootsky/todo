use todo::{cli::DEFAULT_CATEGORY, todo::TodoList};
use chrono::NaiveDate;

#[test]
fn test_add_task_with_category() {
    let mut todo_list = TodoList::new();
    let id = todo_list.add_task("Learn Rust".to_string(), None, "Programming".to_string());
    assert_eq!(id, 1);
    let tasks = todo_list.list_tasks(Some("Programming"));
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].description, "Learn Rust");
}

#[test]
fn test_add_task() {
    let mut todo_list = TodoList::new(); // TODO why is this mutable?
    let id = todo_list.add_task("Learn Rust".to_string(), None, "Programming".to_string());
    assert_eq!(id, 1);
    let category ="Programming";
    assert_eq!(todo_list.list_tasks(Some(category)).len(), 1);
    let task = todo_list.get_task(id).unwrap();
    assert_eq!(task.id, id);
    assert_eq!(task.description, "Learn Rust");
    assert_eq!(task.due_date, None);
}

#[test]
fn test_remove_task() {
    let mut todo_list = TodoList::new();
    let id = todo_list.add_task("Learn Rust".to_string(), None, DEFAULT_CATEGORY.to_string());
    assert!(todo_list.remove_task(id));
    assert_eq!(todo_list.list_tasks(None).len(), 0);
    assert!(!todo_list.remove_task(id));
}

#[test]
fn test_list_tasks() {
    let mut todo_list = TodoList::new();
    todo_list.add_task("Learn Rust".to_string(), None, "Programming".to_string());
    todo_list.add_task("Write tests".to_string(), None, DEFAULT_CATEGORY.to_string());
    let tasks = todo_list.list_tasks(None);
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
    let id = todo_list.add_task(
        "Original task".to_string(),
        Some(original_date),
        "General".to_string(),
    );

    // Update only the description
    todo_list.update_task(id, &Some("Updated task".to_string()), &None, &None);

    let updated_task = todo_list.get_task(id).unwrap();
    assert_eq!(updated_task.description, "Updated task");
    assert_eq!(updated_task.due_date, Some(original_date));

    // Update only the due date
    let new_date = NaiveDate::from_ymd_opt(2023, 7, 1).unwrap();
    todo_list.update_task(id, &None, &Some(new_date), &None);

    let updated_task = todo_list.get_task(id).unwrap();
    assert_eq!(updated_task.description, "Updated task");
    assert_eq!(updated_task.due_date, Some(new_date));

    // Update neither description nor due date
    todo_list.update_task(id, &None, &None, &None);

    let updated_task = todo_list.get_task(id).unwrap();
    assert_eq!(updated_task.description, "Updated task");
    assert_eq!(updated_task.due_date, Some(new_date));

    // Update only the category
    todo_list.update_task(id, &None, &None, &Some("Programming".to_string()));

    let updated_task = todo_list.get_task(id).unwrap();
    assert_eq!(updated_task.description, "Updated task");
    assert_eq!(updated_task.due_date, Some(new_date));
    assert_eq!(updated_task.category, "Programming");
}

#[test]
fn test_list_tasks_by_category() {
    let mut todo_list = TodoList::new();
    todo_list.add_task("Learn Rust".to_string(), None, "Programming".to_string());
    todo_list.add_task("Buy groceries".to_string(), None, "Personal".to_string());
    todo_list.add_task("Read book".to_string(), None, "Personal".to_string());

    let programming_tasks = todo_list.list_tasks(Some("Programming"));
    assert_eq!(programming_tasks.len(), 1);
    assert_eq!(programming_tasks[0].description, "Learn Rust");

    let personal_tasks = todo_list.list_tasks(Some("Personal"));
    assert_eq!(personal_tasks.len(), 2);
}
