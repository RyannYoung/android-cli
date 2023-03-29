/// A basic representation on how to implement a plugin
use crate::models::Cli::{self, Command};

pub struct HelloPlugin;

impl Cli::CLIPlugin for HelloPlugin {
    fn register_commands(&self, cli: &mut Cli::CLI) {
        let command = Command::new("hello")
            .description("A basic example of a hello plugin")
            .long_description("This is a long descriptoin")
            .action(|args| {
                let clap_command = clap::Command::new("Unknown")
                    .version("1.0")
                    .author("Ryan Young")
                    .arg(clap::arg!(--two <VALUE>).required(true))
                    .get_matches_from(args);

                println!("{:?}", clap_command);

                "Hello".into()
            });

        cli.add_command(command);
    }
}
