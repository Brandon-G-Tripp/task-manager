use chrono::Utc;

use super::Task;

pub enum DueFilter {
    PastDue, 
    DueToday, 
    DueThisWeek, 
    All
} 


impl DueFilter {
    pub fn filter(self, tasks: &[Task]) -> Vec<Task> {
        match self {
            DueFilter::PastDue => {
                tasks.iter()
                    .filter(|t| t.due_date.naive_utc() < Utc::now().naive_utc())
                    .cloned()
                    .collect()
            },
            DueFilter::DueToday => {
                tasks.iter()
                    .filter(|t| t.due_date.naive_utc() == Utc::now().naive_utc())
                    .cloned()
                    .collect()
            },
            DueFilter::DueThisWeek => {
                tasks.iter()
                    .filter(|t| {
                        t.due_date.naive_utc() <= Utc::now().naive_utc() + chrono::Duration::days(7)
                    })
                    .cloned()
                    .collect()
            }, 
            DueFilter::All => tasks.to_vec()
        } 
    } 
}
