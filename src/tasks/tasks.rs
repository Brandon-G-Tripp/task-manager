use crate::tasks::Task;

#[derive(Debug)]
pub struct Tasks {
    tasks: Vec<Task>,
} 

impl Tasks {
    fn new() -> Self {
        Self {
            tasks: Vec::new()
        } 
    } 

    fn get_tasks(&self) -> &[Task] {
        &self.tasks
    } 

    fn add(&mut self, task: Task) -> usize {
        self.tasks.push(task);
        let index = self.tasks.len() - 1;

        index
    } 
} 

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use super::*;

    #[test]
    fn test_get_tasks() {
        let mut tasks = Tasks::new();

        // Add some test tasks
        tasks.add(Task::new(1, "Task".to_string(), "Test".to_string(), Utc::now()));

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
        
        let task = Task::new(1, "Task".to_string(), "Test".to_string(), Utc::now());

        // Add some test tasks
        let index = tasks.add(task);

        let stored_tasks = tasks.get_tasks();

        assert_eq!(stored_tasks.len(), 1);
        assert_eq!(stored_tasks[0], stored_tasks[index]);
    } 
} 
