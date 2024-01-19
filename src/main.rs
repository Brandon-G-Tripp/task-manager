use structopt::StructOpt;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_yaml;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    command: Option<Command>
} 

#[derive(StructOpt)]
enum Command {
    Tasks,
    Snippets,
    Timers,
}

fn main() {
    let cli = Cli::from_args();

    match &cli.command {
        Some(Command::Tasks) => {
        },
        Some(Command::Snippets) => {
        },
        Some(Command::Timers) => {
        },
        None => {
            println!("No command passed");
        }, 
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_no_command() {
        let cli = Cli::from_args();
        assert!(cli.command.is_none()); 
    }

} 
