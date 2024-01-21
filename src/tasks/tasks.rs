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

    fn delete_task(&mut self, id: i32) -> bool {
        let index = self.tasks.iter().position(|t| t.id == id);
        if let Some(index) = index {
            self.tasks.remove(index);
            true
        } else {
            false
        } 
    } 

    fn find_task_by_id(&self, id: i32) -> Option<&Task> {
        for task in &self.tasks {
            if task.id == id {
                return Some(task);
            } 
        } 

        None
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

    #[test]
    fn test_delete_task() {
        // Arrange 
        let mut tasks = Tasks::new();
        let task1 = Task::new(1, "Task 1".to_string(), "Text for task1".to_string(), Utc::now());
        let task2 = Task::new(2, "Task 2".to_string(), "Text for task2".to_string(), Utc::now());
        tasks.add(task1);
        tasks.add(task2);

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
        let task1 = Task::new(1, "Task 1".to_string(), "Text for task1".to_string(), Utc::now());
        tasks.add(task1);

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
        let task1 = Task::new(1, "Task 1".to_string(), "Text for task1".to_string(), Utc::now());
        tasks.add(task1);

        // Act 
        let found = tasks.find_task_by_id(1);

        // Assert
        assert_eq!(found.unwrap().id, 1);
    } 

    #[test]
    fn test_find_invalid_id() {
        // Arrange 
        let mut tasks = Tasks::new();
        let task1 = Task::new(1, "Task 1".to_string(), "Text for task1".to_string(), Utc::now());
        tasks.add(task1);


        // Act
        let found = tasks.find_task_by_id(2);

        // Assert
        assert_eq!(found, None); 

    } 
} 
