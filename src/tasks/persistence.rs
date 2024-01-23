use std::{path::Path, fs};

use serde::{Deserialize, Serialize};

use super::{Task, Tasks, TaskError};

pub const TASKS_FILE: &str = "./data/tasks.yaml";

#[derive(Serialize, Deserialize)]
struct TasksSchema {
    tasks: Vec<Task>,
} 

pub fn save_tasks(tasks: &Tasks, path: Option<&Path>) -> Result<(), TaskError> {
    let path = path.unwrap_or(Path::new(TASKS_FILE));
    write_tasks(&tasks, path)?;
    Ok(())
} 


pub fn load_from_file(task_file_path: Option<&Path>) -> Result<Tasks, TaskError> {
    let path = task_file_path.unwrap_or(Path::new(TASKS_FILE));
    match read_tasks(path) {
        Ok(tasks) => Ok(tasks),
        Err(_) => Err(TaskError::NoFile)
    } 
} 

fn write_tasks(tasks: &Tasks, path: &Path) -> Result<(), TaskError> {
    let schema = TasksSchema {
        tasks: tasks.tasks.clone()
    }; 

    let yaml = serde_yaml::to_string(&schema)?;

    fs::write(path, yaml)?;

    Ok(())
} 

fn read_tasks(path: &Path) -> Result<Tasks, TaskError> {
    let data = fs::read_to_string(path)?;

    let schema: TasksSchema = serde_yaml::from_str(&data)?;

    let next_id = schema.tasks.len() as u32;

    Ok(Tasks {
        tasks: schema.tasks,
        next_id
    })
} 

#[cfg(test)]
mod tests {
    use std::env;

    use chrono::Utc;

    use super::*;

    // Testing read/write io 
    #[test]
    fn test_write_tasks() {
        // Setup 
        let mut tasks = Tasks::new();
        let tmp_dir = env::temp_dir();
        let tmp_file = tmp_dir.join("test_write_tasks.yml");

        // Create task and save tasks 
        let due_date = Utc::now().to_string();
        tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), due_date); 

        write_tasks(&tasks, &tmp_file).unwrap();

        let data = fs::read_to_string(tmp_file).unwrap();
        let saved_tasks: TasksSchema = serde_yaml::from_str(&data).unwrap();

        assert_eq!(saved_tasks.tasks.len(), 1);
    } 

    #[test]
    fn test_read_tasks() {
        // Setup
        let mut tasks = Tasks::new();
        let tmp_dir = env::temp_dir();
        let tmp_file = tmp_dir.join("test_read_tasks.yml");

        // Create task and add 
        let due_date = Utc::now().to_string();
        tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), due_date); 

        // Write tasks
        write_tasks(&tasks, &tmp_file).unwrap();

        let tasks = read_tasks(&tmp_file).unwrap();

        assert_eq!(tasks.tasks.len(), 1);
        assert_eq!(tasks.tasks[0].description, "Text for task1");
    }

    // Testing load from file and write to file methods 
    #[test]
    fn test_load_from_file() {
        let mut tasks = Tasks::new();
        let tmp_dir = env::temp_dir();
        let tmp_file = tmp_dir.join("test_load_file.yml");

        let due_date = Utc::now().to_string();
        tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), due_date); 

        write_tasks(&tasks, &tmp_file).unwrap();

        let tasks = load_from_file(Some(&tmp_file)).unwrap();

        assert_eq!(tasks.tasks.len(), 1);
    } 


    #[test]
    fn test_save_tasks() {
        // Setup 
        let mut tasks = Tasks::new();
        let tmp_dir = env::temp_dir();
        let tmp_file = tmp_dir.join("test_save_tasks.yml");

        // Create task and save tasks 
        let due_date = Utc::now().to_string();
        tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), due_date); 

        save_tasks(&tasks, Some(&tmp_file)).unwrap();

        // Read from file use direct methods to isolate test 
        let data = fs::read_to_string(tmp_file).unwrap();
        let saved_tasks: TasksSchema = serde_yaml::from_str(&data).unwrap();

        assert_eq!(saved_tasks.tasks.len(), 1);
    } 

    #[test]
    fn test_read_invalid_file() {
        let tmp_dir = env::temp_dir();
        let invalid_path = tmp_dir.join("invalid.yml");

        let result = read_tasks(&invalid_path);
        assert!(matches!(result, Err(_)));
    } 
}
