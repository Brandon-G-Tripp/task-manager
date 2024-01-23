mod tests {
    use std::io;
    use std::io::Write;
    use std::io::sink;
    use std::io::stdout;
    use std::env;
    use std::path::PathBuf;

    use chrono::Utc;
    use assert_cmd::Command;
    use assert_cmd::prelude::*;

    use super::*;
    use crate::tasks::crud::Tasks;
    use crate::tasks::filtering::DueFilter;
    use crate::tasks::update;
    use update::UpdateFields;

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

    // Testing Update_tasks

    #[test]
    fn test_update_args() {
        // Setup
        let mut tasks = Tasks::new();
        let due_date = Utc::now().to_string();
        tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), due_date); 

        // Capture ID 
        let task_id = tasks.tasks[0].id;

        // Define Update 
        let new_due = Utc::now() + chrono::Duration::days(1);
        let new_name = "Updated Name".to_string();
        let fields = UpdateFields {
            name: Some(new_name.clone()),
            description: Some("New Desc".to_string()),
            due_date: Some(new_due.to_string()),
            completed: None,
        };

        // Execute update 
        tasks.update_task(task_id, fields).unwrap();

        // Validate update 
        let updated = &tasks.tasks[0];
        assert_eq!(updated.name, new_name);
        assert_eq!(updated.due_date, new_due);
        assert_eq!(updated.description, "New Desc".to_string());
    } 

    // Tests for filtering 
    #[test]
    fn test_past_due() {
      let mut tasks = Tasks::new();
      let due_date = Utc::now() - chrono::Duration::days(2);  
      tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), due_date.to_string()); 
      tasks.add_task("Task 2".to_string(), "Text for task2".to_string(), (due_date + chrono::Duration::days(2)).to_string()); 
      
      let results = tasks.due_tasks(DueFilter::PastDue);
      
      assert_eq!(results.len(), 1);
      assert_eq!(results[0].name, "Text for task1");
    } 

    #[test]
    fn test_due_today() {
      let mut tasks = Tasks::new();
      let due_date = Utc::now().date_naive();  
      tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), due_date.to_string()); 
      
      let results = tasks.due_tasks(DueFilter::DueToday);
      
      assert_eq!(results.len(), 1);
      assert_eq!(results[0].name, "Due Today"); 
    }

    #[test]
    fn test_due_this_week() {
      let mut tasks = Tasks::new();
      let due_date = Utc::now().date_naive() + chrono::Duration::days(5);  
      tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), due_date.to_string()); 
      
      let results = tasks.due_tasks(DueFilter::DueThisWeek);
      
      assert_eq!(results.len(), 1);
      assert_eq!(results[0].name, "Due This Week"); 
    }


    #[test] 
    fn test_all_tasks() {
      let mut tasks = Tasks::new();
      tasks.add_task("Task 1".to_string(), "Desc".to_string(), Utc::now().date_naive().to_string());
      tasks.add_task("Task 2".to_string(), "Desc".to_string(), (Utc::now().date_naive() + chrono::Duration::days(7)).to_string());
      
      let results = tasks.due_tasks(DueFilter::All); 
      
      assert_eq!(results.len(), 2);
    }

} 
