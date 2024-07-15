use todo::{cli::DEFAULT_CATEGORY, todo::{DueDate, NewTask, Priority, TaskUpdate, TodoList}};
use chrono::NaiveDate;

#[test]
fn test_add_task_with_category() {
    let mut todo_list = TodoList::new();
    let id = todo_list.add_task(NewTask {
        description: "Learn Rust".to_string(),
        due_date: DueDate::None,
        category: "Programming".to_string(),
        priority: Priority::Medium,
    });
    assert_eq!(id, 1);
    let tasks = todo_list.list_tasks(Some("Programming"));
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].description, "Learn Rust");
    assert!(matches!(tasks[0].due_date, DueDate::None));
    assert_eq!(tasks[0].category, "Programming");
    assert_eq!(tasks[0].priority, Priority::Medium);
}

#[test]
fn test_remove_task() {
    let mut todo_list = TodoList::new();
    
    let task_update = NewTask {
        description: "Learn Rust".to_string(),
        due_date: DueDate::None,
        category: DEFAULT_CATEGORY.to_string(),
        priority: Priority::Medium,
    };

    let id = todo_list.add_task(task_update);
    assert!(todo_list.remove_task(id));
    assert_eq!(todo_list.list_tasks(None).len(), 0);
    assert!(!todo_list.remove_task(id));
}

#[test]
fn test_list_tasks() {
    let mut todo_list = TodoList::new();
    todo_list.add_task( NewTask {
        description: "Learn Rust".to_string(),
        category: "Programming".to_string(),
        due_date: DueDate::None,
        priority: Priority::Medium,
    }) ;
    todo_list.add_task( NewTask {
        description: "Write tests".to_string(),
        category: DEFAULT_CATEGORY.to_string(),
        due_date: DueDate::None,
        priority: Priority::High,
    });

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
    let id = todo_list.add_task(NewTask {
        description: "Original task".to_string(),
        due_date: DueDate::None,
        category: "General".to_string(),
        priority: Priority::Low,
    });

    let task_update = TaskUpdate {
        description: Some("Updated task".to_string()),
        due_date: Some(original_date),
        category: None,
    };

    // Update only the description
    todo_list.update_task(id, task_update);

    let updated_task = todo_list.get_task(id).unwrap();
    assert_eq!(updated_task.description, "Updated task");
    if let DueDate::On(date) = updated_task.due_date {
        assert_eq!(date, original_date);
    } else {
        panic!("Expected DueDate::On");
    }

    // Update only the due date
    let new_date = NaiveDate::from_ymd_opt(2024, 7, 15).unwrap();
    let task_update = TaskUpdate {
        description: None,
        due_date: Some(new_date),
        category: None,
    };
    todo_list.update_task(id, task_update);

    let updated_task = todo_list.get_task(id).unwrap();
    assert_eq!(updated_task.description, "Updated task");
    if let DueDate::On(date) = updated_task.due_date {
        assert_eq!(date, new_date);
    } else {
        panic!("Expected DueDate::On");
    }
    // Update neither description nor due date
    let task_update = TaskUpdate {
        description: None,
        due_date: None,
        category: None,
    };
    todo_list.update_task(id, task_update);

    let updated_task = todo_list.get_task(id).unwrap();
    assert_eq!(updated_task.description, "Updated task");
    if let DueDate::On(date) = updated_task.due_date {
        assert_eq!(date, new_date);
    } else {
        panic!("Expected DueDate::On");
    }
    // Update only the category
    let task_update = TaskUpdate {
        description: None,
        due_date: None,
        category: Some("Programming".to_string()),
    };
    todo_list.update_task(id, task_update);

    let updated_task = todo_list.get_task(id).unwrap();
    assert_eq!(updated_task.description, "Updated task");
    if let DueDate::On(date) = updated_task.due_date {
        assert_eq!(date, new_date);
    } else {
        panic!("Expected DueDate::On");
    }
    assert_eq!(updated_task.category, "Programming");
}

#[test]
fn test_list_tasks_by_category() {
    let mut todo_list = TodoList::new();
    todo_list.add_task( NewTask {
        description: "Learn Rust".to_string(), 
        category: "Programming".to_string(),
        due_date: DueDate::None,
        priority: Priority::Low,
    });
    todo_list.add_task( NewTask {
        description: "Buy groceries".to_string(), 
        category:  "Personal".to_string(),
        due_date: DueDate::None,
        priority: Priority::Medium,
    });
    todo_list.add_task( NewTask {
       description:  "Read book".to_string(), 
       category: "Personal".to_string(),
       due_date: DueDate::None,
       priority: Priority::High,
    });

    let programming_tasks = todo_list.list_tasks(Some("Programming"));
    assert_eq!(programming_tasks.len(), 1);
    assert_eq!(programming_tasks[0].description, "Learn Rust");

    let personal_tasks = todo_list.list_tasks(Some("Personal"));
    assert_eq!(personal_tasks.len(), 2);
}
