use std::error::Error;

use serde::{Serialize, Deserialize};
use serde_yaml;

mod task;
mod crud;
mod update;
mod filtering;
mod stats;

pub mod cli;
pub mod persistence;

pub use cli::*;
pub use task::*;
pub use crud::*;
pub use update::*;
pub use stats::*;

#[derive(Debug)]
pub enum TaskError {
    NotFound, 
    InvalidTaskId,
    ParseUpdateError,
    NoFile,
    ParseBoolError,
    Io(std::io::Error),
    Yaml(serde_yaml::Error),
    InvalidInput(String),
}


impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskError::NotFound => write!(f, "Task not found"),
            TaskError::NoFile => write!(f, "No file found"),
            TaskError::InvalidTaskId => write!(f, "Invalid task ID"),
            TaskError::ParseUpdateError => write!(f, "Erroring in parsing update"),
            TaskError::ParseBoolError => write!(f, "Error parsing string to boolean"),
            TaskError::Io(err) => write!(f, "IO error: {}", err),
            TaskError::Yaml(err) => write!(f, "YAML error: {}", err),
            TaskError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        } 
    } 
} 

impl Error for TaskError {}

impl From<std::io::Error> for TaskError {
    fn from(err: std::io::Error) -> Self {
        TaskError::Io(err)
    } 
} 


impl From<serde_yaml::Error> for TaskError {
    fn from(err: serde_yaml::Error) -> Self {
        TaskError::Yaml(err)
    } 
} 


#[cfg(test)]
mod tests {
    #[test]
    fn test_task_error_display() {
        let err = TaskError::NotFound;
        assert_eq!(err.to_string(), "Task not found");

        let err = TaskError::NoFile;
        assert_eq!(err.to_string(), "No file found");

        let err = TaskError::InvalidTaskId;
        assert_eq!(err.to_string(), "Invalid task ID");

        let err = TaskError::ParseUpdateError;
        assert_eq!(err.to_string(), "Erroring in parsing update");

        let err = TaskError::ParseBoolError;
        assert_eq!(err.to_string(), "Error parsing string to boolean");

        let err = TaskError::Io(std::io::Error::new(std::io::ErrorKind::Other, "io error"));
        assert_eq!(err.to_string(), "IO error: io error");

        let err = TaskError::Yaml(serde_yaml::Error::explicit("yaml error"));
        assert_eq!(err.to_string(), "YAML error: yaml error");

        let err = TaskError::InvalidInput("invalid input".to_string());
        assert_eq!(err.to_string(), "Invalid input: invalid input");
    }
} 
