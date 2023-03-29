use clap::ArgMatches;

/// Represents a structured command/plugin module
pub struct Command {
    // The invoking name of the plugin
    name: String,

    // The command action
    action: Box<dyn Fn(ArgMatches) -> String>,

    // The CLAP argument interface
    clap_command: clap::Command,
}

impl Command {
    // Create a new blanket instance of a command
    pub fn new<S: Into<String>>(name: S) -> Self {
        Command {
            name: name.into(),
            action: Box::new(|_args| String::new()),
            clap_command: clap::Command::new(""),
        }
    }

    pub fn clap(mut self, command: clap::Command) -> Self {
        self.clap_command = command;
        self
    }

    pub fn action<F: 'static + Fn(ArgMatches) -> String>(mut self, action: F) -> Self {
        self.action = Box::new(action);
        self
    }

    pub fn run(&self, args: ArgMatches) -> String {
        (self.action)(args)
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn clap_command(&self) -> &clap::Command {
        &self.clap_command
    }
}
