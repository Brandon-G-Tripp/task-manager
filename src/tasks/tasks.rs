use core::fmt;
use std::io::Write;

use crate::tasks::Task;
use structopt::StructOpt;

#[derive(Debug)]
pub struct Tasks {
    tasks: Vec<Task>,
    next_id: u32,
} 

#[derive(StructOpt)]
enum TaskCommand {
    Add {name: String, description: String, due_date: String},
    List, 

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

    fn find_task_by_id(&self, id: u32) -> Option<&Task> {
        for task in &self.tasks {
            if task.id == id {
                return Some(task);
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
        let found = tasks.find_task_by_id(1);

        // Assert
        assert_eq!(found.unwrap().id, 1);
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
} 
