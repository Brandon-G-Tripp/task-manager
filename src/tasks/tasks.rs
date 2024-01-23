use core::fmt;
use std::{io::{Write, self}, error::Error, cell::RefCell, borrow::BorrowMut, fs};

use crate::tasks::{Task, update};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use crate::tasks::UpdateFields;

use super::TaskCommandUpdateArgs;

const TASKS_FILE: &str = "./data/tasks.yaml";

#[derive(Debug)]
pub struct Tasks {
    tasks: Vec<Task>,
    next_id: u32,
} 

#[derive(Debug)]
pub enum TaskError {
    NotFound, 
    InvalidTaskId,
    ParseUpdateError,
    ParseBoolError,
}

#[derive(StructOpt)]
pub enum TaskCommand {
    Add {name: String, description: String, due_date: String},
    List, 
    Delete {id: u32},
    Update { id: u32, fields: String },
    Show {id: u32},
    Complete {id: u32},
} 

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskError::NotFound => write!(f, "Task not found"),
            TaskError::InvalidTaskId => write!(f, "Invalid task ID"),
            TaskError::ParseUpdateError => write!(f, "Erroring in parsing update"),
            TaskError::ParseBoolError => write!(f, "Error parsing string to boolean"),
            TaskError::Io(err) => write!(f, "IO error: {}", err),
            TaskError::Yaml(err) => write!(f, "YAML error: {}", err),
        } 
    } 
} 

impl Error for TaskError {}

impl From<std::io::Error> for TaskError {

} 


#[derive(Serialize, Deserialize)]
struct TasksSchema {
    tasks: Vec<Task>,
} 

#[derive(StructOpt)]
pub enum TaskCommand {
    Add {name: String, description: String, due_date: String},
    List, 
    Delete {id: u32},
    Update { id: u32, fields: String },
    Show {id: u32},
    Complete {id: u32},
} 



pub fn run(tasks: &mut Tasks, cmd: &TaskCommand) {

    match cmd {
        TaskCommand::Add { name, description, due_date } => {
            tasks.add_task(name.to_string(), description.to_string(), due_date.to_string());
        } 
        TaskCommand::List => {
            tasks.list_tasks(&mut std::io::stdout());
        } 
        TaskCommand::Delete { id } => {
            tasks.delete_task(*id);
        } 
        TaskCommand::Update { id, fields } => {
            let update_fields = update::parse_update_fields(&fields);
            print!("we are in update branch");
            tasks.update_task(*id, update_fields);
        } 
        TaskCommand::Show{ id } => {
            tasks.show_task(*id, &mut std::io::stdout());
        } 
        TaskCommand::Complete { id } => {
            tasks.complete_task(*id);
        } 
    } 
} 

impl Tasks {

    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        } 
    } 

    fn write_tasks(&self, path: &str) -> Result<(), TaskError> {
        let schema = TasksSchema {
            tasks: self.tasks.clone()
        }; 

        let yaml = serde_yaml::to_string(&schema)?;

        fs::write(TASKS_FILE, yaml)?;

        Ok(())
    } 

    fn read_tasks(path: &str) -> Result<Self, TaskError> {
        let data = fs::read_to_string(TASKS_FILE)?;

        let schema: TasksSchema = serde_yaml::from_str(&data)?;

        Ok(Tasks {
            tasks: schema.tasks,
            next_id: schema.tasks.len()
        })
    } 



    fn get_tasks(&self) -> &[Task] {
        &self.tasks
    } 

    fn add_task(&mut self, name: String, description: String, due_date: String) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let new_task = Task::new(
            id,
            name,
            description,
            due_date
        );

        self.tasks.push(new_task);

        let index = self.tasks.len() - 1;

        index
    } 

    fn delete_task(&mut self, id: u32) -> bool {
        let index = self.tasks.iter().position(|t| t.id == id);
        if let Some(index) = index {
            self.tasks.remove(index);
            true
        } else {
            false
        } 
    } 

    fn find_task_by_id(&self, id: u32) -> Option<(usize, &Task)> {
        for (index, task) in self.tasks.iter().enumerate() {
            if task.id == id {
                return Some((index, task));
            } 
        } 

        None
    } 

    fn list_tasks(&self, mut writer: impl std::io::Write) {
        let tasks = self.get_tasks();

        for task in tasks {
           writeln!(writer, "{}", task); 
        } 
    }

    fn update_task(&mut self, id: u32, fields: UpdateFields) -> Result<(), TaskError> {

            print!("we are in update tas");
        // Find existing Task 
        // let (index, task) = self.find_task_by_id(id).unwrap();
        let task_id = id;

        match self.find_task_by_id(task_id) {
            Some((index, mut task)) => {

                // new task 
                let due_date = fields.due_date.unwrap_or(task.due_date.to_string());
                let completed = fields.completed.unwrap_or(task.completed.to_string());
                let completed_bool = match completed.as_str() {
                    "true" => true,
                    "false" => false,
                    _ => return Err(TaskError::ParseBoolError),
                }; 

                let due_date = {
                    let datetime = DateTime::parse_from_str(&due_date, "%+").unwrap();
                    datetime.into()
                };

                let updated = Task {
                    id: task.id,
                    name: fields.name.unwrap_or(task.name.clone()),
                    description: fields.description.unwrap_or(task.description.clone()),
                    due_date,
                    completed: completed_bool
                }; 
                
                // replace in vector 
                self.tasks[index] = updated;
            }
            None => {
                println!("Task with ID {} not found!", task_id);
            } 
        } 

        Ok(())
    } 

    fn show_task(&self, id: u32, writer: &mut impl Write) -> Result<(), TaskError> {
        match self.find_task_by_id(id) {
            Some((_, task)) => {
                writeln!(writer, "{}", task);
                Ok(())
            },
            None => Err(TaskError::NotFound)
        } 
    } 

    pub fn complete_task(&mut self, id: u32) -> Result<(), TaskError> {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks[index].completed = true;
            Ok(())
        } else {
            Err(TaskError::NotFound)
        } 
    } 
} 


#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Write;
    use std::io::sink;
    use std::io::stdout;

    use chrono::Utc;
    use assert_cmd::Command;
    use assert_cmd::prelude::*;

    use super::*;

    #[test]
    fn test_get_tasks() {
        let mut tasks = Tasks::new();

        // Add some test tasks
        tasks.add_task("Task".to_string(), "Test".to_string(), Utc::now().to_string());

        let stored_tasks = tasks.get_tasks();

        // Assert number of tasks
        assert_eq!(stored_tasks.len(), 1);

        // Assert task fields
        assert_eq!(stored_tasks[0].id, 1);
        assert_eq!(stored_tasks[0].name, "Task");
    } 

    #[test]
    fn test_add_task() {
        let mut tasks = Tasks::new();

        // Add some test tasks
        let index = tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), Utc::now().to_string());

        let stored_tasks = tasks.get_tasks();

        assert_eq!(stored_tasks.len(), 1);
        assert_eq!(stored_tasks[0], stored_tasks[index]);
    } 

    #[test]
    fn test_delete_task() {
        // Arrange 
        let mut tasks = Tasks::new();
        tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), Utc::now().to_string());
        tasks.add_task("Task 2".to_string(), "Text for task2".to_string(), Utc::now().to_string());

        // Act 
        let deleted = tasks.delete_task(1);

        // Assert 
        let stored_tasks = tasks.get_tasks();
        assert!(deleted);
        assert_eq!(stored_tasks.len(), 1);
        assert_eq!(tasks.tasks[0].id, 2);
    } 

    #[test]
    fn test_delete_invalid() {
        // Arrange 
        let mut tasks = Tasks::new();
        tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), Utc::now().to_string());

        // Act
        let deleted = tasks.delete_task(2);

        // Assert
        let stored_tasks = tasks.get_tasks();
        assert!(!deleted);
        assert_eq!(stored_tasks.len(), 1);
    } 

    #[test]
    fn test_find_task_by_id() {
        // Arrange 
        let mut tasks = Tasks::new();
        tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), Utc::now().to_string());

        // Act 
        let (_, found) = tasks.find_task_by_id(1).unwrap();

        // Assert
        assert_eq!(found.id, 1);
    } 

    #[test]
    fn test_find_invalid_id() {
        // Arrange 
        let mut tasks = Tasks::new();
        tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), Utc::now().to_string());


        // Act
        let found = tasks.find_task_by_id(2);

        // Assert
        assert_eq!(found, None); 

    } 


    // testing list tasks
    #[test]
    fn test_list_tasks_empty() {
        // Arrange
        let tasks = Tasks::new();
        let mut writer = Vec::new();

        // Act 
        tasks.list_tasks(&mut writer);

        // Read output 
        let output = String::from_utf8(writer).unwrap();

        // Assert - capture output and check empty 
        assert_eq!(
            output,
            ""
        );
    } 

     #[test] 
     fn test_list_one_task() {

       // Arrange
       let mut tasks = Tasks::new();
       let due_date = Utc::now().to_string();
       let compare_due_date = due_date.clone();
       tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), due_date);

       let mut writer = Vec::new();

       // Act 
       tasks.list_tasks(&mut writer);

       let output = String::from_utf8(writer).unwrap();

       let expected_output = format!("1 - Task 1 - Text for task1 - {}\n", compare_due_date);

       // Assert - output contains task
       assert_eq!(
           output,
           expected_output
           );

     }

    #[test]
    fn test_list_multiple() {

      //Arrange
        let mut tasks = Tasks::new();
        tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), Utc::now().to_string());
        tasks.add_task("Task 2".to_string(), "Text for task2".to_string(), Utc::now().to_string());


      // Capture output 
        let mut writer = Vec::new();

        // Act
        tasks.list_tasks(&mut writer);

        let output = String::from_utf8(writer).unwrap();

      // Assert both tasks printed  
        assert!(output.contains("1 - Task 1"));
        assert!(output.contains("2 - Task 2"));

    }

    #[test]
    fn test_delete_task_removes_it() {
        // Setup 
        let mut tasks = Tasks::new();
        tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), Utc::now().to_string());
        let mut writer = Vec::new();

        // Act
        tasks.list_tasks(&mut writer);

        let output = String::from_utf8(writer).unwrap();

      // Assert task printed  
        assert!(output.contains("1 - Task 1"));
        
        // Delete task and assert the len is 0
        tasks.delete_task(1);
        assert_eq!(tasks.tasks.len(), 0);
        

        // Setup output to check writer contains nothing
        let mut writer = Vec::new();
        tasks.list_tasks(&mut writer);
        let output = String::from_utf8(writer).unwrap();

        assert!(output.contains(""));
    } 

    #[test]
    fn test_update_task() {
        // Setup 
        let mut tasks = Tasks::new();
        tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), Utc::now().to_string());

        let (index, original_task) = tasks.find_task_by_id(1).unwrap();
        assert_eq!("Task 1", original_task.name);

        // Update task 
        let updated_fields = UpdateFields { 
            name: Some("updated name".to_string()),
            ..Default::default() 
        };
        
        tasks.update_task(1, updated_fields);

        // Validate after update 
        let (index, updated_task) = tasks.find_task_by_id(1).unwrap();
        // assert!(output.contains("1 - update name"));
        assert_eq!("updated name", updated_task.name);
        
    } 

    // Show command
    #[test]
    fn test_show_task() {
        // Setup
        let mut tasks = Tasks::new();
        let due_date = Utc::now().to_string();
        let compare_due_date = due_date.clone();
        tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), due_date); 

        let expected = format!(
            "{} - Task 1 - Text for task1 - {}\n",
            1,
            compare_due_date
        );

        // Act 
        let mut output = Vec::new();
        tasks.show_task(1, &mut output);
        
        // Assert 
        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, expected);
    } 

    // Complete Task command
    #[test]
    fn test_complete_task() {
        // Setup 
        let mut tasks = Tasks::new();
        let due_date = Utc::now().to_string();
        tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), due_date); 

        // Act
        tasks.complete_task(1);

        // Assert 
        let (_, task) = tasks.find_task_by_id(1).unwrap();
        assert_eq!(task.completed, true);
    }

    #[test]
    fn test_update_args() {
        let mut tasks = Tasks::new();
        let due_date = Utc::now().to_string();
        tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), due_date); 

        // let cmd = TaskCommand::Update {
        //     id: 1,
        //     fields: "name=new name".to_string(),
        // };
    } 

} 
