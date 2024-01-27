use chrono::Utc;

use crate::tasks::{Task, Tasks};

pub fn create_tasks() -> Vec<Task> {
    vec![
        Task::new(1, "Task 1".to_string(), "Description".to_string(), (Utc::now() - chrono::Duration::days(3)).to_string()),
        Task::new(2, "Task 2".to_string(), "Description".to_string(), (Utc::now() - chrono::Duration::days(2)).to_string()),
        Task::new(3, "Task 3".to_string(), "Description".to_string(), (Utc::now() + chrono::Duration::hours(1)).to_string()),
        Task::new(4, "Task 4".to_string(), "Description".to_string(), (Utc::now() + chrono::Duration::days(1)).to_string()),
        Task::new(5, "Task 5".to_string(), "Description".to_string(), (Utc::now() + chrono::Duration::days(2)).to_string())
    ]
}

pub fn create_tasks_completion() -> Tasks {
    // Setup 
    let mut tasks = Tasks::new();
    let due_date = Utc::now().to_string();
    tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), due_date); 
    tasks.add_task("Task 2".to_string(), "Description".to_string(), (Utc::now() + chrono::Duration::days(3)).to_string());
    tasks.add_task("Task 3".to_string(), "Description".to_string(), (Utc::now() + chrono::Duration::days(2)).to_string());

    tasks.complete_task(1);
    tasks.complete_task(2);

    tasks
} 
