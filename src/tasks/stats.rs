use crate::tasks::{Task, Tasks};

pub struct Stats {
    total: usize, 
    completed: usize, 
    percent_completed: u8,
} 

impl Stats {
    pub fn new(tasks: &[Task]) -> Stats {
        let total = tasks.len();

        let completed = tasks
            .iter()
            .filter(|t| t.completed)
            .count();

        let percent = completed as u8 / total as u8 * 100;

        Stats { total, completed, percent_completed: percent }
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

    assert_eq!(stats.total, 2);
    assert_eq!(stats.completed, 1);
    assert_eq!(stats.percent_completed, 50);
} 

