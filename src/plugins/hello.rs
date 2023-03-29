/// A basic representation on how to implement a plugin
use crate::models::cli::{self, Command};

pub struct HelloPlugin;

impl cli::CLIPlugin for HelloPlugin {
    fn register_command(&self, _cli: &mut cli::Cli) -> Command {
        Command::new("hello").action(|args| args.get_one::<String>("echo").unwrap().to_string())
    }

    fn register_argument_parser(&self) -> clap::Command {
        clap::Command::new("hello").arg(clap::arg!([echo]).required(true))
    }
}
