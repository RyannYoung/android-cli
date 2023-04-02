use crate::models::{cli::CLIPlugin, command::Command};

/// Enables configuration of the CLI settings

pub struct SetPlugin;

impl CLIPlugin for SetPlugin {
    fn register_command(&self, cli: &mut crate::models::cli::Cli) -> Command {
        // Get a reference to the current configuration
        let config = cli.config.clone();

        Command::new("set").action(move |_args| {
            println!("{:?}", &config);
            String::from("This feature has not been implemented yet!")
        })
    }

    fn register_argument_parser(&self) -> clap::Command {
        clap::Command::new("set").arg(clap::arg!([config]).required(true))
    }
}
