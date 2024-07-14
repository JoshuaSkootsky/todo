// src/todo.rs
use chrono::NaiveDate;
use std::collections::HashMap;

pub struct Task {
    pub id: u32,
    pub description: String,
    pub due_date: Option<NaiveDate>,
    pub category: String,
}

pub struct TodoList {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn get_task(&self, id: u32) -> Option<&Task> {
        self.tasks.get(&id)
    }

    pub fn add_task(
        &mut self,
        description: String,
        due_date: Option<NaiveDate>,
        category: String,
    ) -> u32 {
        let id = self.next_id;
        self.tasks.insert(
            id,
            Task {
                id,
                description,
                due_date,
                category,
            },
        );
        self.next_id += 1;
        id
    }

    pub fn remove_task(&mut self, id: u32) -> bool {
        self.tasks.remove(&id).is_some()
    }

    pub fn list_tasks(&self, category: Option<&str>) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| category.map_or(true, |c| task.category == c))
            .collect()
    }

    pub fn update_task(
        &mut self,
        id: u32,
        description: &Option<String>,
        due_date: &Option<NaiveDate>,
        category: &Option<String>,
    ) -> bool {
        if let Some(task) = self.tasks.get_mut(&id) {
            if let Some(desc) = description {
                task.description = desc.to_string();
            }
            if let Some(date) = &due_date {
                task.due_date = Some(*date);
            }
            if let Some(cat) = category {
                task.category = cat.to_string();
            }
            true
        } else {
            false
        }
    }

    pub fn get_categories(&self) -> Vec<String> {
        self.tasks
            .values()
            .map(|task| task.category.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut list = TodoList::new();
        let id = list.add_task("Test task".to_string(), None, "Test".to_string());
        assert_eq!(id, 1);
    }

    #[test]
    fn test_get_task() {
        let mut list = TodoList::new();
        let id = list.add_task("Test task".to_string(), None, "Test".to_string());
        let task = list.get_task(id).unwrap();
        assert_eq!(task.id, id);
        assert_eq!(task.description, "Test task");
        assert_eq!(task.due_date, None);
        assert_eq!(task.category, "Test");
    }
}
