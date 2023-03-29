use colored::Colorize;

use crate::models::cli::{self, CLIPlugin, Command};
use std::fmt::Write;

pub struct HelpPlugin;

impl CLIPlugin for HelpPlugin {
    fn register_command(&self, cli: &mut cli::Cli) -> Command {
        // Load all the currently loaded commands (hence this plugin MUST be loaded last)
        let loaded_clap_commands: Vec<(String, String)> = cli
            .commands
            .iter()
            .map(|cmd| {
                // Load the clap command
                let mut clap_cmd = cmd.clap_command().clone();

                // Load the information
                let name = clap_cmd.get_name().to_string();
                let usage = clap_cmd.render_usage().to_string();

                (name, usage)
            })
            .collect();

        // Generate Base Help
        Command::new("help").action(move |_args| {
            let mut output = String::new();

            // Commands output
            writeln!(&mut output, "{}", "Commands".underline().bold()).unwrap();

            for (name, usage) in &loaded_clap_commands {
                writeln!(&mut output, "  {:<15}{}", name, usage).unwrap();
            }

            // Command specific output
            // ...todo

            output
        })
    }

    fn register_argument_parser(&self) -> clap::Command {
        clap::Command::new("help")
    }
}
