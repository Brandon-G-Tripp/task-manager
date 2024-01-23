use std::error::Error;

use serde::{Serialize, Deserialize};
use serde_yaml;

mod task;
mod crud;
mod update;

pub mod cli;
pub mod persistence;

pub use cli::*;
pub use task::*;
pub use crud::*;
pub use update::*;

#[derive(Debug)]
pub enum TaskError {
    NotFound, 
    InvalidTaskId,
    ParseUpdateError,
    NoFile,
    ParseBoolError,
    Io(std::io::Error),
    Yaml(serde_yaml::Error),
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
