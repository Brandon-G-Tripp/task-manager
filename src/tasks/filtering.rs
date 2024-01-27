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

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};

    use super::*;
    use crate::tests_common::{create_tasks, create_tasks_completion};

    // Tests for DueFilter

    #[test]
    fn filter_past_due() {
        let tasks = create_tasks();
        let results = DueFilter::PastDue.filter(&tasks);
        println!("{:?}", results);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id, 1);
        assert_eq!(results[1].id, 2);
    } 

    #[test]
    fn filter_due_today() {
        let tasks = create_tasks();
        let results = DueFilter::DueToday.filter(&tasks);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 3);
    } 

    #[test]
    fn filter_due_this_week() {
        let tasks = create_tasks();
        let results = DueFilter::DueThisWeek.filter(&tasks);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].id, 3);
        assert_eq!(results[1].id, 4);
        assert_eq!(results[2].id, 5);
    }

    #[test]
    fn filter_all() {
        let tasks = create_tasks();
        let results = DueFilter::All.filter(&tasks);
        assert_eq!(results.len(), 5);
        assert_eq!(results[0].id, 1);
        assert_eq!(results[1].id, 2);
        assert_eq!(results[2].id, 3);
        assert_eq!(results[3].id, 4);
        assert_eq!(results[4].id, 5);
    }  


    // Tests for completed filter
    #[test]
    fn returns_all_tasks_when_all_filter() {
        let tasks = create_tasks_completion();

        let result = CompletionFilter::All.filter(&tasks);

        assert_eq!(result.len(), 3);
    } 

    #[test]
    fn returns_only_completed_tasks() {
        let tasks = create_tasks_completion();

        let result = CompletionFilter::Complete.filter(&tasks);

        assert_eq!(result.len(), 2);
        assert!(result[0].completed);
        assert!(result[1].completed);
    } 

    #[test]
    fn returns_only_incomplete_tasks() {
        let tasks = create_tasks_completion();

        let result = CompletionFilter::Complete.filter(&tasks);

        assert_eq!(result.len(), 1);
        assert!(!result[0].completed);
    } 
} 
