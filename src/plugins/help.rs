use colored::Colorize;

use crate::models::Cli::{CLIPlugin, Command};
use std::fmt::Write;

pub struct HelpPlugin;

impl CLIPlugin for HelpPlugin {
    fn register_commands(&self, cli: &mut crate::models::Cli::CLI) {
        let loaded_commands = cli
            .commands
            .iter()
            .map(|cmd| {
                (
                    cmd.name.clone(),
                    cmd.description.clone(),
                    cmd.long_description.clone(),
                )
            })
            .collect::<Vec<_>>();

        let command = Command::new("help")
            .description("Displays help information")
            .action(move |args| {
                println!("{:?}", args);
                match args.len() {
                    0 => get_generic_help(&loaded_commands),
                    1 => get_long_help(args, &loaded_commands),
                    _ => String::from("Too many args to attempt help"),
                }
            });

        cli.add_command(command);
    }
}

fn get_generic_help(commands: &Vec<(String, String, Option<String>)>) -> String {
    let mut output = String::new();

    writeln!(&mut output, "{}", "Available Commands".bold().underline()).unwrap();

    for (name, description, _long_description) in commands {
        writeln!(&mut output, "  {} - {}", name, description).unwrap();
    }

    output
}

/// Returns a string containing the long-evaluated help message format
fn get_long_help(args: &[String], commands: &Vec<(String, String, Option<String>)>) -> String {
    // Check if the command is valid
    match &commands.iter().find(|(name, _, _)| name.clone() == args[0]) {
        Some((name, _, long_desc)) => {
            let desc = match long_desc {
                Some(long_desc) => format!("{}", long_desc),
                None => format!("Appears to be no extended help for {}", name),
            };

            // Print the long description
            let mut output = String::new();

            writeln!(&mut output, "{}", "Extended Help".underline().bold()).unwrap();
            writeln!(&mut output, "{}", desc).unwrap();

            output
        }
        None => format!("Unable to find long help for command: {}", args[0]),
    }
}
