use crate::models::Cli::{CLIPlugin, Command};

pub struct Walkdir;

impl CLIPlugin for Walkdir {
    fn register_commands(&self, cli: &mut crate::models::Cli::CLI) {
        let command = Command::new("walkdir")
            .description("A filesystem directory walker")
            .action(|args| {
                let target_path = match args.len() {
                    1.. => args[0].clone(),
                    0 | _ => String::from("."),
                };
                let paths: Vec<String> = walkdir::WalkDir::new(target_path)
                    .into_iter()
                    .filter_map(|v| v.ok())
                    .map(|x| x.path().to_str().unwrap().to_owned())
                    .collect();

                paths.join("\n")
            });

        cli.add_command(command);
    }
}
