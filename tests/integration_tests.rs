use assert_cmd::Command;
use predicates;

#[test]
fn run_task_cli() {
  // let mut cmd = Command::cargo_bin("task-manager").unwrap();
  // cmd.arg("tasks");
  
  // // Add a task
  // let assert = cmd.assert();
  // assert.success()
  //    .stdout(predicates::str::contains("Added task: "));

  // // List tasks  
  // let assert = cmd.assert();
  // assert.success()
  //    .stdout(predicates::str::contains("My new task"));
   
  // // Delete a task
  // let assert = cmd.assert();
  // assert.success()
  //    .stdout(predicates::str::contains("Deleted task: "));

  // // Error checking
  // let assert = cmd.assert();
  // assert.failure()
  //    .stderr(predicates::str::contains("Invalid task ID"));
}
