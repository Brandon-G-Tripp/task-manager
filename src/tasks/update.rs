use std::str::FromStr;
use regex::Regex;

use super::TaskError;

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

pub fn parse_update_fields(update_args: &str) -> Result<UpdateFields, TaskError>{
    let mut update_fields = UpdateFields::default();

    for pair in update_args.split(", ") {
        let kv: Vec<_> = pair.splitn(2, ':').collect();

        if kv.len() != 2 {
            return Err(TaskError::InvalidInput("Invalid input. Not a key/value pair.".to_string()));
        }

        let key = kv[0];
        let value = kv[1];

        match key {
            "name" => update_fields.name = Some(value.to_string()),
            "description" => update_fields.description = Some(value.to_string()),
            "due_date" => {
                if !valid_due_date_format(value) {
                    return Err(TaskError::InvalidInput("Invalid datetime format for due date".to_string()));
                } 
                update_fields.due_date = Some(value.to_string());
            },
            "completed" => {
                if !valid_completed_format(value) {
                    return Err(TaskError::InvalidInput("Invalid boolean string for completed".to_string()));
                } 
                update_fields.completed = Some(value.to_string());
            },
            _ => {
                let err_string = format!("Unknown field key: {}", key);
                return Err(TaskError::InvalidInput(err_string))
            },
        }
    }

    Ok(update_fields)
} 

fn valid_due_date_format(value: &str) -> bool {
    let reg = Regex::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\+\d{2}:\d{2}").unwrap();
    reg.is_match(value)
} 

fn valid_completed_format(value: &str) -> bool {
    value == "true" || value == "false"
} 


#[cfg(test)]
mod tests {
    use crate::tasks::TaskError;

    use super::parse_update_fields;

    #[test]
    fn test_parse_update_invalid() {
        let result = parse_update_fields("invalid");

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), TaskError::InvalidInput(_)));
    } 

    #[test]
    fn test_parse_update_invalid_completed(){
        let result = parse_update_fields("completed:invalid");
        
        let err_str = "Invalid boolean string for completed".to_string();
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), TaskError::InvalidInput(err_str)));
    }


    #[test]
    fn test_parse_update_invalid_due_date(){
        let result = parse_update_fields("due_date:invalid");

        let err_str = "Invalid datetime format for due date".to_string();
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), TaskError::InvalidInput(err_str)));
    }

} 
