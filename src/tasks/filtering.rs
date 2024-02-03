use std::{str::FromStr, fmt};

use chrono::Utc;

use super::Task;

#[derive(Debug)]
pub enum DueFilter {
    PastDue, 
    DueToday, 
    DueThisWeek, 
    All
} 


impl DueFilter {
    pub fn filter(&self, tasks: &[Task]) -> Vec<Task> {
        match *self {
            DueFilter::PastDue => {
                let today = Utc::now().naive_utc();
                tasks.iter()
                    .filter(|t| t.due_date.naive_utc() < today )
                    .cloned()
                    .collect()
            },
            DueFilter::DueToday => {
                let today = Utc::now().naive_utc().date();

                tasks.iter()
                    .filter(|t| {
                        let due_date = t.due_date.naive_utc().date();
                        due_date == today
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

impl FromStr for DueFilter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "today" => Ok(DueFilter::DueToday),
            "week" => Ok(DueFilter::DueThisWeek),
            "past" => Ok(DueFilter::PastDue),
            _ => Err("Error parsing flag from string".to_string()),
        } 
    }
} 

impl fmt::Display for DueFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DueFilter::PastDue => write!(f, "past_due"),
            DueFilter::DueToday => write!(f, "due_today"),
            DueFilter::DueThisWeek => write!(f, "due_this_week"),
            DueFilter::All => write!(f, "all")
        }
    }
}

#[derive(Debug)]
pub enum CompletionFilter {
    All,
    Complete,
    Incomplete,
} 

impl CompletionFilter {
    pub fn filter(&self, tasks: &[Task]) -> Vec<Task> {
        match *self {
            CompletionFilter::All => tasks.to_vec(),
            CompletionFilter::Complete => {
                tasks.iter()
                    .filter(|t| t.completed)
                    .cloned()
                    .collect()
            },
            CompletionFilter::Incomplete => {
                tasks.iter()
                    .filter(|t| !t.completed)
                    .cloned()
                    .collect()
            },
        } 
    } 
} 

impl FromStr for CompletionFilter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(CompletionFilter::All),
            "complete" => Ok(CompletionFilter::Complete),
            "incomplete" => Ok(CompletionFilter::Incomplete),
            _ => Err("Error parsing flag from string".to_string()),
        } 
    }
} 

impl fmt::Display for CompletionFilter { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompletionFilter::All => write!(f, "all"),
            CompletionFilter::Complete => write!(f, "complete"),
            CompletionFilter::Incomplete => write!(f, "incomplete")
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
        println!("{:?}", results);
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

        let result = CompletionFilter::All.filter(&tasks.tasks);

        assert_eq!(result.len(), 3);
    } 

    #[test]
    fn returns_only_completed_tasks() {
        let tasks = create_tasks_completion();

        let result = CompletionFilter::Complete.filter(&tasks.tasks);

        assert_eq!(result.len(), 2);
        assert!(result[0].completed);
        assert!(result[1].completed);
    } 

    #[test]
    fn returns_only_incomplete_tasks() {
        let tasks = create_tasks_completion();

        let result = CompletionFilter::Incomplete.filter(&tasks.tasks);

        assert_eq!(result.len(), 1);
        assert!(!result[0].completed);
    } 

    #[test]
    fn parse_due_filter_from_str() {
        let today_filter = DueFilter::from_str("today");
        assert!(matches!(today_filter, Ok(DueFilter::DueToday)));
        let week_filter = DueFilter::from_str("week");
        assert!(matches!(week_filter, Ok(DueFilter::DueThisWeek)));
        let past_filter = DueFilter::from_str("past");
        assert!(matches!(past_filter, Ok(DueFilter::PastDue)));
        assert!(DueFilter::from_str("invalid").is_err());
    }

    #[test]
    fn format_due_filter() {
        assert_eq!(format!("{}", DueFilter::DueToday), "due_today");
        assert_eq!(format!("{}", DueFilter::DueThisWeek), "due_this_week");
        assert_eq!(format!("{}", DueFilter::PastDue), "past_due");
        assert_eq!(format!("{}", DueFilter::All), "all");
    }

    #[test]
    fn parse_completion_filter_from_str() {
        let all_filter = CompletionFilter::from_str("all");
        let complete_filter = CompletionFilter::from_str("complete");
        let incomplete_filter = CompletionFilter::from_str("incomplete");

        assert!(matches!(all_filter, Ok(CompletionFilter::All)));
        assert!(matches!(complete_filter, Ok(CompletionFilter::Complete)));
        assert!(matches!(incomplete_filter, Ok(CompletionFilter::Incomplete)));
        assert!(CompletionFilter::from_str("invalid").is_err());
    } 
    
} 
