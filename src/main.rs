#![allow(unused_imports, dead_code, unused_variables)]

use structopt::StructOpt;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_yaml;
use predicates;

mod tasks;
mod snippets;
mod timers;
mod config;

#[cfg(test)]
mod tests_common;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    command: Option<AppCommand>
} 

#[derive(StructOpt)]
enum AppCommand {
    Tasks,
    Snippets,
    Timers,
}

fn main() {
    let cli = Cli::from_args();

    match &cli.command {
        Some(AppCommand::Tasks) => {
            tasks::run()
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

    use super::*;

    #[cfg(test)]
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
        cmd.arg("tasks");

        let assert = cmd.assert();
        assert.success();

        let assert = cmd.assert();
        assert.stdout(predicates::str::contains("Tasks placeholder"));
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
} 
