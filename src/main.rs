use std::process::Command;
use structopt::StructOpt;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_yaml;
use predicates;

mod tasks;

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
        },
        Some(AppCommand::Timers) => {
        },
        None => {
            println!("No command passed");
        }, 
    } 
}

#[cfg(test)]
mod tests {
    use std::env;
    use assert_cmd::Command;
    use pred;

    use super::*;

    #[test]
    fn test_cli_no_command() {
        let cli = Cli::from_args();
        assert!(cli.command.is_none()); 
    }

    #[test]
    fn test_tasks_command() {
        let mut cmd = Command::cargo_bin("task-manager").unwrap();
        cmd.arg("tasks");

        let assert = cmd.assert();

        assert.success();

        assert.stdout(predicate::str::contains("Tasks placeholder"));
    } 

} 
