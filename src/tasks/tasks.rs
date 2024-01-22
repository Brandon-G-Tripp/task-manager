use core::fmt;
use std::{io::Write, error::Error, cell::RefCell, borrow::BorrowMut};

use crate::tasks::Task;
use chrono::DateTime;
use structopt::StructOpt;
use crate::tasks::UpdateFields;

#[derive(Debug)]
pub struct Tasks {
    tasks: Vec<Task>,
    next_id: u32,
} 

#[derive(Debug)]
pub enum TaskError {
    NotFound, 
    InvalidTaskId,
    ParseUpdateError
}

#[derive(StructOpt)]
enum TaskCommand {
    Add {name: String, description: String, due_date: String},
    List, 
    Delete {id: u32},
    Update { id: u32, fields: UpdateFields },
} 

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskError::NotFound => write!(f, "Task not found"),
            TaskError::InvalidTaskId => write!(f, "Invalid task ID"),
            TaskError::ParseUpdateError => write!(f, "Erroring in parsing update"),

        } 
    } 
} 

pub fn run(tasks: &mut Tasks) {
    let cmd = TaskCommand::from_args();

    match cmd {
        TaskCommand::Add { name, description, due_date } => {
            tasks.add_task(name, description, due_date);
        } 
        TaskCommand::List => {
            tasks.list_tasks(&mut std::io::stdout());
        } 
        TaskCommand::Delete { id } => {
            tasks.delete_task(id);
        } 
        TaskCommand::Update {id, fields } => {
            println!("Updating...");
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
        // Find existing Task 
        let (index, task) = self.find_task_by_id(id).unwrap();
        
        // new task 
        let due_date = fields.due_date.unwrap_or(task.due_date.to_string());

        let due_date = {
            let datetime = DateTime::parse_from_str(&due_date, "%+").unwrap();
            datetime.into()
        };

        let updated = Task {
            id: task.id,
            name: fields.name.unwrap_or(task.name.clone()),
            description: fields.description.unwrap_or(task.description.clone()),
            due_date
        }; 
        
        // replace in vector 
        self.tasks[index] = updated;

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
    fn test_show_command() {
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
} 
