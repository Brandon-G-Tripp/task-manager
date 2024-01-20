use chrono::{DateTime, Utc};

#[derive(PartialEq, Debug)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub due_date: DateTime<Utc>,
} 

impl Task {
    pub fn new(id: i32, name: String, description: String, due_date: DateTime<Utc>) -> Self {
        Self {
            id,
            name,
            description,
            due_date
        } 
    } 
}

pub fn run() {
    println!("Tasks placeholder");
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task() {
        let due_date = {
            let datetime = DateTime::parse_from_str("2023-03-01T12:00:00Z", "%+").unwrap();
            datetime.into()
            
        };
        let task = Task::new(
            1, 
            "My Task".to_string(), 
            "Description".to_string(), 
            due_date
        );

        assert_eq!(task.id, 1);
        assert_eq!(task.name, "My Task");

    } 
} 
