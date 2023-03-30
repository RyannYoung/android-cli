use std::env::current_dir;

use crate::models::{cli::CLIPlugin, command::Command};

pub struct PwdPlugin;

impl CLIPlugin for PwdPlugin {
    fn register_command(&self, _cli: &mut crate::models::cli::Cli) -> Command {
        Command::new("pwd").action(|_args| {
            let current_dir = current_dir().unwrap();

            current_dir.to_str().unwrap().to_string()
        })
    }

    fn register_argument_parser(&self) -> clap::Command {
        clap::Command::new("pwd")
    }
}
