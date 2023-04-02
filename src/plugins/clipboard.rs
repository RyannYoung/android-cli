use crate::models::{self, cli::CLIPlugin, command::Command};
use copypasta::{ClipboardContext, ClipboardProvider};

pub struct ClipboardPlugin;

impl CLIPlugin for ClipboardPlugin {
    fn register_command(&self, _cli: &mut models::cli::Cli) -> Command {
        Command::new("clipboard").action(|_args| {
            let mut ctx = ClipboardContext::new().unwrap();
            let content = ctx.get_contents().unwrap();
            content
        })
    }

    fn register_argument_parser(&self) -> clap::Command {
        clap::Command::new("clipboard")
            .about("Gets information currently set on the clipboard (platform independent)")
    }
}
