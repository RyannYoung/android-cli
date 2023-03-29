/// A basic representation on how to implement a plugin
use crate::models::Cli::{self, Command};

pub struct HelloPlugin;

impl Cli::CLIPlugin for HelloPlugin {
    fn register_commands(&self, cli: &mut Cli::CLI) {
        let command = Command::new("hello")
            .description("A basic example of a hello plugin")
            .long_description("This is a long description")
            .action(|args| {
                let mut clap_args = args.to_owned();
                clap_args.insert(0, "hello".to_string());

                // Generate and match a clap command
                let matches = match clap::Command::new("hello")
                    .arg(clap::arg!([echo]).index(1).required(true))
                    .try_get_matches_from_mut(clap_args)
                {
                    Ok(matches) => matches,
                    Err(error) => return error.to_string(),
                };

                matches.get_one::<String>("echo").unwrap().to_string()
            });

        cli.add_command(command);
    }
}
