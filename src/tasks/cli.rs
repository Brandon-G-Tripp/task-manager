use structopt::StructOpt;

use crate::tasks::{crud::Tasks, update};

use super::persistence;

#[derive(StructOpt)]
pub enum TaskCommand {
    Add {name: String, description: String, due_date: String},
    List, 
    Delete {id: u32},
    Update { id: u32, fields: String },
    Show {id: u32},
    Complete {id: u32},
} 



pub fn run(tasks: &mut Tasks, cmd: &TaskCommand) {

    match cmd {
        TaskCommand::Add { name, description, due_date } => {
            tasks.add_task(name.to_string(), description.to_string(), due_date.to_string());
        } 
        TaskCommand::List => {
            tasks.list_tasks(&mut std::io::stdout());
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
    } 

    match persistence::save_tasks(tasks, None) {
        Ok(()) => (), 
        Err(e) => {
            eprint!("Failed to save tasks: {}", e);
        } 
    } 
} 