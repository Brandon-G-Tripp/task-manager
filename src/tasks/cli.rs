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
            tasks.update_task(*id, update_fields);
        } 
        TaskCommand::Show{ id } => {
            tasks.show_task(*id, &mut std::io::stdout());
        } 
        TaskCommand::Complete { id } => {
            tasks.complete_task(*id);
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
}
    
    
