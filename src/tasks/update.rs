use std::str::FromStr;

use crate::tasks::TaskError;

#[derive(Debug)]
pub enum TaskCommandUpdateArgs {
    Fields(String)
} 

#[derive(Debug, Default)]
pub struct UpdateFields {
    pub name: Option<String>,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub completed: Option<String>,
}

impl UpdateFields {
    pub fn default() -> Self {
        Self {
            name: None,
            description: None,
            due_date: None,
            completed: None,
        } 
    } 
} 

impl FromStr for UpdateFields {
    type Err = crate::tasks::TaskError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.splitn(3, ' ').collect();

        Ok(Self {
            name: fields.get(0).map(|s| s.to_string()),
            description: fields.get(1).map(|s| s.to_string()),
            due_date: fields.get(2).map(|s| s.to_string()),
            completed: fields.get(3).map(|s| s.to_string()),
        })
    }
} 

pub fn parse_update_fields(update_args: &str) -> UpdateFields {
    let mut update_fields = UpdateFields::default();

    for pair in update_args.split(", ") {
        let kv: Vec<_> = pair.splitn(2, ':').collect();

        if kv.len() == 2 {
            let key = kv[0];
            let value = kv[1];

            match key {
                "name" => update_fields.name = Some(value.to_string()),
                "description" => update_fields.description = Some(value.to_string()),
                "due_date" => update_fields.due_date = Some(value.to_string()),
                "completed" => update_fields.completed = Some(value.to_string()),
                _ => println!("Unknown field key: {}", key),
            }
        } 
    }

    update_fields
} 



