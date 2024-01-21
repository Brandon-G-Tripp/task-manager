use core::fmt;

use chrono::{DateTime, Utc};

#[derive(PartialEq, Debug)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub due_date: DateTime<Utc>,
} 

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} - {} - {} - {}", self.id, self.name, self.description, self.due_date)
    } 
} 

impl Task {
    pub fn new(id: u32, name: String, description: String, due_date: String) -> Self {
        let due_date = {
            let datetime = DateTime::parse_from_str(&due_date, "%+").unwrap();
            datetime.into()
        };
            
        Self {
            id,
            name,
            description,
            due_date
        } 
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task() {
        let task = Task::new(
            1, 
            "My Task".to_string(), 
            "Description".to_string(), 
            "2023-03-01T12:00:00Z".to_string(),
        );

        assert_eq!(task.id, 1);
        assert_eq!(task.name, "My Task");

    } 
} 
