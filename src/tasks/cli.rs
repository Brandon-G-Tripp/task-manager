use structopt::StructOpt;

use crate::tasks::{crud::Tasks, update};

use super::{persistence, filtering::{DueFilter, CompletionFilter}};

#[derive(StructOpt)]
pub enum TaskCommand {
    Add {name: String, description: String, due_date: String},
    List {
        #[structopt(short, long)]
        due: Option<DueFilter>,
        #[structopt(short, long)]
        status: Option<CompletionFilter>,
    }, 
    Delete {id: u32},
    Update { id: u32, fields: String },
    Show {id: u32},
    Complete {id: u32},
    Stats,
} 

pub fn run(tasks: &mut Tasks, cmd: &TaskCommand) {

    match cmd {
        TaskCommand::Add { name, description, due_date } => {
            tasks.add_task(name.to_string(), description.to_string(), due_date.to_string());
        } 
        TaskCommand::List { due, status } => {
            tasks.list_tasks(&mut std::io::stdout(), due, status);
        } 
        TaskCommand::Delete { id } => {
            tasks.delete_task(*id);
        } 
        TaskCommand::Update { id, fields } => {
            let update_fields = update::parse_update_fields(&fields);
            print!("we are in update branch");
            let _ = tasks.update_task(*id, update_fields);
        } 
        TaskCommand::Show{ id } => {
            let _ = tasks.show_task(*id, &mut std::io::stdout());
        } 
        TaskCommand::Complete { id } => {
            let _ = tasks.complete_task(*id);
        } 
        TaskCommand::Stats => {
            let stats = tasks.stats();
            println!("{}", stats);
        } 
    } 

    match persistence::save_tasks(tasks, None) {
        Ok(()) => (), 
        Err(e) => {
            eprint!("Failed to save tasks: {}", e);
        } 
    } 
} 


#[cfg(test)]
mod tests{
    use chrono::Utc;

    use crate::tests_common::create_tasks;

    use super::*;

    #[test]
    fn test_add_command_parsing() {
        let mut tasks = Tasks::new();

        let cmd = TaskCommand::Add {
            name: "Task 1".to_string(),
            description: "Description 1".to_string(), 
            due_date: "2023-03-01T12:00:00Z".to_string()
        };

        run(&mut tasks, &cmd);
        
        assert_eq!(tasks.tasks.len(), 1);
        assert_eq!(tasks.tasks[0].name, "Task 1");
        assert_eq!(tasks.tasks[0].description, "Description 1");
        assert_eq!(tasks.tasks[0].due_date.to_string(), "2023-03-01 12:00:00 UTC");
    }

    #[test]
    fn test_list_command() {
        let mut tasks = Tasks::new();
        tasks.add_task("Task 1".to_string(), "".to_string(), "2023-03-01T12:00:00Z".to_string());

        let cmd = TaskCommand::List { due: None, status: None };
        
        let mut writer = Vec::new();
        run(&mut tasks, &cmd);
        tasks.list_tasks(&mut writer, &None, &None);

        let output = String::from_utf8(writer).unwrap();
        
        assert!(output.contains("Task 1"));
    }

    #[test]
    fn test_delete_command() {
        let mut tasks = Tasks::new();
        tasks.add_task("Task 1".to_string(), "".to_string(), "2023-03-01T12:00:00Z".to_string());

        let cmd = TaskCommand::Delete { id: 1 };
        run(&mut tasks, &cmd);

        assert_eq!(tasks.tasks.len(), 0);
    } 

    #[test]
    fn test_update_command() {
        let mut tasks = Tasks::new();
        tasks.add_task("Task 1".to_string(), "".to_string(), "2023-03-01T12:00:00Z".to_string());

        let cmd = TaskCommand::Update {
            id: 1,
            fields: "name:New Name, description:Update desc, completed:true".to_string(),
        }; 

        run(&mut tasks, &cmd);

        assert_eq!(tasks.tasks[0].name, "New Name");
        assert!(tasks.tasks[0].completed);
    } 

    fn test_stats_command() {
        let mut tasks = Tasks::new();
        tasks.add_task("Task 1".to_string(), "".to_string(), "2023-03-01T12:00:00Z".to_string());
        tasks.add_task("Task 2".to_string(), "".to_string(), "2023-03-01T12:00:00Z".to_string());
        tasks.add_task("Task 3".to_string(), "".to_string(), "2023-03-01T12:00:00Z".to_string());

        tasks.complete_task(1);

        let cmd = TaskCommand::Stats; 
        let mut writer = Vec::new();
        run(&mut tasks, &cmd);
        tasks.stats();

        let output = String::from_utf8(writer).unwrap();
        assert!(output.contains("Tasks: 3"));
        assert!(output.contains("Completed: 1")); 
    }

    #[test]
    #[should_panic]
    fn test_add_command_invalid_due_date() {
        let mut tasks = Tasks::new();

        let cmd = TaskCommand::Add {
            name: "Task 1".to_string(),
            description: "Description 1".to_string(),
            due_date: "invalid date".to_string() 
        };

        run(&mut tasks, &cmd);
    }

    #[test]
    fn test_list_command_with_due_filter() {
        let mut tasks = Tasks::new();
        tasks.add_task("Task 1".to_string(), "Text for task1".to_string(), Utc::now().to_string());
        tasks.add_task("Task 2".to_string(), "".to_string(), "2023-03-01T12:00:00Z".to_string());


        let cmd = TaskCommand::List { 
            due: Some(DueFilter::DueToday), 
            status: None 
        };
        
        let mut writer = Vec::new();
        run(&mut tasks, &cmd);
        tasks.list_tasks(&mut writer, &Some(DueFilter::DueToday), &None);

        let output = String::from_utf8(writer).unwrap();
        println!("{:?}", output);
        assert!(output.contains("Task 1"));
        assert!(!output.contains("Task 2"));
    }

    #[test]
    fn test_list_command_filters() { 
        let mut tasks = create_tasks();

        tasks.complete_task(1).expect("There was an error updating the task's completion status.");

        let cmd = TaskCommand::List { due: Some(DueFilter::PastDue), status: Some(CompletionFilter::Complete) };

        let mut writer = Vec::new();
        run(&mut tasks, &cmd);
        tasks.list_tasks(&mut writer, &Some(DueFilter::PastDue), &Some(CompletionFilter::Complete));

        let output = String::from_utf8(writer).unwrap();
        assert!(output.contains("Overdue Task 1"));
        
    }
    
    // #[test] 
    // fn test_show_command() {
    //     let mut tasks = create_tasks();

    //     let cmd = TaskCommand::Show { id: 1 };

    //     let mut writer = Vec::new();
    //     run(&mut tasks, &cmd);
    //     tasks.show_task(1, &mut writer);

    //     let output = String::from_utf8(writer).unwrap();
    //     assert!(output.contains("Overdue Task 1"));
    // } 

    #[test]
    #[should_panic]
    fn test_update_command_invalid() {
        let tasks = create_tasks();

        let cmd = TaskCommand::Update { id: 999, fields: "invalid".to_string() };
    } 

}
    
    
