use std::fmt;

use crate::tasks::{Task, Tasks};

#[derive(Debug)]
pub struct Stats {
    total: usize, 
    completed: usize, 
    percent_completed: usize,
} 

impl Stats {
    pub fn new(tasks: &[Task]) -> Stats {
        let total = tasks.len();

        let completed = tasks
            .iter()
            .filter(|t| t.completed)
            .count();

        let percent = if total == 0 {
            0
        } else {
            let temp_float = (completed as f32 / total as f32) * 100.0;
            temp_float as usize

        };

        println!("percent: {}", percent);

        Stats { total, completed, percent_completed: percent }
    } 
} 

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Total: {}\n", self.total);
        write!(f, "Completed: {}\n", self.completed);
        write!(f, "Percent completed: {}\n", self.percent_completed);

        Ok(())

    }
} 

#[test]
fn test_stats() {
    use crate::tests_common::{create_tasks, create_tasks_completion};
    // Arrange 
    let mut tasks = create_tasks_completion();

    //Act 
    let stats = tasks.stats();

    // Assert 

    assert_eq!(stats.total, 3);
    assert_eq!(stats.completed, 2);
    assert_eq!(stats.percent_completed, 66);
} 

