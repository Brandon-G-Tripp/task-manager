use std::{io::{Write, self}, error::Error, cell::RefCell, borrow::BorrowMut, fs, path::Path};
use chrono::{DateTime, Utc};

use crate::tasks::{Task, update};

use super::{TaskError, persistence, UpdateFields, filtering::{DueFilter, CompletionFilter}};

#[cfg(test)]
mod tests;


#[derive(Debug)]
pub struct Tasks {
    pub tasks: Vec<Task>,
    pub next_id: u32,
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

    pub fn add_task(&mut self, name: String, description: String, due_date: String) -> usize {
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

    pub fn delete_task(&mut self, id: u32) -> bool {
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

    pub fn list_tasks(&self, mut writer: impl std::io::Write) {
        let tasks = self.get_tasks();

        for task in tasks {
           writeln!(writer, "{}", task); 
        } 
    }

    pub fn update_task(&mut self, id: u32, fields: UpdateFields) -> Result<(), TaskError> {
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

    pub fn show_task(&self, id: u32, writer: &mut impl Write) -> Result<(), TaskError> {
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

    pub fn filter_tasks(tasks: &[Task], due_filter: DueFilter, completion_filter: CompletionFilter) -> Vec<Task> {
        let mut filtered = due_filter.filter(tasks);
        filtered = completion_filter.filter(&filtered);
        filtered
    }
} 
