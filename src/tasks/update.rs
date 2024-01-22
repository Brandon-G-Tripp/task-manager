use std::str::FromStr;

use crate::tasks::TaskError;

#[derive(Debug, Default)]
pub struct UpdateFields {
    pub name: Option<String>,
    pub description: Option<String>,
    pub due_date: Option<String>
}

impl UpdateFields {
    pub fn default() -> Self {
        Self {
            name: None,
            description: None,
            due_date: None,
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
        })
    }
    // type Err = ParseUpdateError;
    //

} 

