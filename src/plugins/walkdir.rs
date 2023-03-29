use crate::models::{
    cli::{self, CLIPlugin},
    command::Command,
};

pub struct Walkdir;

impl CLIPlugin for Walkdir {
    fn register_command(&self, _cli: &mut cli::Cli) -> Command {
        Command::new("walkdir").action(|args| {
            let target_path = args.get_one::<String>("path").unwrap();
            let paths: Vec<String> = walkdir::WalkDir::new(target_path)
                .into_iter()
                .filter_map(|v| v.ok())
                .map(|x| x.path().to_str().unwrap().to_owned())
                .collect();

            paths.join("\n")
        })
    }

    fn register_argument_parser(&self) -> clap::Command {
        clap::Command::new("walkdir").arg(clap::arg!([path]).required(true))
    }
}
