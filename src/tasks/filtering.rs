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
                let today = Utc::now().naive_utc();
                tasks.iter()
                    .filter(|t| t.due_date.naive_utc() < today )
                    .cloned()
                    .collect()
            },
            DueFilter::DueToday => {
                let today_start = Utc::now().naive_utc().date().and_hms_opt(0, 0, 0).unwrap();
                let today_end = Utc::now().naive_utc().date().and_hms_opt(23, 59, 59).unwrap();

                tasks.iter()
                    .filter(|t| {
                        t.due_date.naive_utc() >= today_start && t.due_date.naive_utc() <= today_end
                    })
                    .cloned()
                    .collect()
            },
            DueFilter::DueThisWeek => {
                let start = Utc::now().naive_utc(); 
                let end = start + chrono::Duration::days(7);
                tasks.iter()
                    .filter(|t| {
                        let due_date = t.due_date.naive_utc();  
                        due_date >= start && due_date <= end
                    })
                    .cloned()
                    .collect()
            }, 
            DueFilter::All => tasks.to_vec()
        } 
    } 
}
