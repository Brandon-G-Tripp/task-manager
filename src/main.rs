#![allow(unused_imports, dead_code, unused_variables)]

use structopt::StructOpt;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

mod tasks;
mod snippets;
mod timers;
mod config;

use tasks::TaskCommand;

#[cfg(test)]
mod tests_common;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    command: Option<AppCommand>
} 

#[derive(StructOpt)]
enum AppCommand {
    #[structopt(name = "tasks")]
    Tasks(TaskCommand),
    Snippets,
    Timers,
}

fn main() {
    let cli = Cli::from_args();
    let mut tasks = tasks::Tasks::new();

    match &cli.command {
        Some(AppCommand::Tasks(subcommand)) => {
            tasks::run(&mut tasks, subcommand)
        },
        Some(AppCommand::Snippets) => {
            snippets::run()
        },
        Some(AppCommand::Timers) => {
            timers::run()
        },
        None => {
            println!("No command passed");
        }, 
    } 
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates;

    use self::tasks::{TaskCommandUpdateArgs, Tasks};

    use super::*;

    use super::tests_common::test_setup;

    

    #[test]
    fn test_cli_no_command() {
        let cli = Cli::from_args();
        assert!(cli.command.is_none()); 
    }

    #[test]
    fn test_tasks_command() {
        test_setup::setup();

        let mut cmd = Command::cargo_bin("task-manager").unwrap();

        cmd.arg("tasks")
            .arg("add")
            .arg("Task name")
            .arg("helol desc")
            .arg("2023-03-01T12:00:00Z");

        cmd.assert()
            .success();

        // assert.stdout(predicates::str::contains("Tasks placeholder"));
    } 

    #[test]
    fn test_snippets_command() {
        test_setup::setup();

        let mut cmd = Command::cargo_bin("task-manager").unwrap();
        cmd.arg("snippets");

        let assert = cmd.assert();
        assert.success();

        let assert = cmd.assert();
        assert.stdout(predicates::str::contains("Snippets placeholder"));
    } 

    #[test]
    fn test_timers_command() {
        test_setup::setup();

        let mut cmd = Command::cargo_bin("task-manager").unwrap();
        cmd.arg("timers");

        let assert = cmd.assert();
        assert.success();

        let assert = cmd.assert();
        assert.stdout(predicates::str::contains("Timers placeholder"));
    } 

    #[test]
    fn test_update_command() {
        test_setup::setup();

        let mut cmd = Command::cargo_bin("task-manager").unwrap();

        cmd.arg("tasks")
            .arg("add")
            .arg("Task name")
            .arg("helol desc")
            .arg("2023-03-01T12:00:00Z");

        cmd.assert()
            .success();
            
        let mut cmd = Command::cargo_bin("task-manager").unwrap();

        cmd.arg("tasks")
            .arg("update")
            .arg("1")
            .arg("name: new name");

        // let assert = cmd.assert();
        cmd.assert()
            .success();

        let assert = cmd.assert();
        // assert.stdout(predicates::str::contains(
        //         "Tasks placeholder"
        //     )
        // );
    } 
} 
