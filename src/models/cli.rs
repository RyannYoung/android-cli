use clap::ArgMatches;

pub struct Cli {
    pub commands: Vec<Command>,
}

pub enum ParseResult {
    Success(Option<ArgMatches>),
    Fail(Box<dyn std::error::Error>),
}

impl Cli {
    pub fn new() -> Self {
        // Create a base implementation
        Self {
            commands: Vec::new(),
        }
    }

    pub fn load_plugin<P: CLIPlugin>(&mut self, plugin: P) {
        // Collect the argument parsing
        let clap_command = plugin.register_argument_parser();

        // Construct the plugin
        let plugin_command = plugin.register_command(self).clap(clap_command);
        self.commands.push(plugin_command)
    }

    // Match and invoke a command
    pub fn run_command<S: Into<String>>(&self, name: S, args: ArgMatches) -> String {
        let name: String = name.into();

        match self.commands.iter().find(|cmd| cmd.name == name) {
            Some(res) => res.run(args),
            None => format!("No plugin found: {}", name),
        }
    }

    // Parses the command from the plugin and return a clap::matches struct
    pub fn parse_command<S: Into<String>>(&self, input: S) -> ParseResult {
        let input: String = input.into();
        let raw_args: Vec<String> = input.split_whitespace().map(|s| s.to_owned()).collect();
        let raw_name = &raw_args[0];

        // Find the associated plugin
        match self.commands.iter().find(|cmd| cmd.name == *raw_name) {
            Some(cmd) => match cmd.clap_command.clone().try_get_matches_from(raw_args) {
                Ok(matches) => ParseResult::Success(Some(matches)),
                Err(err) => ParseResult::Fail(Box::new(err)),
            },
            None => ParseResult::Success(None),
        }
    }
}

pub trait CLIPlugin {
    // Register the command to the CLI interface
    fn register_command(&self, cli: &mut Cli) -> Command;

    // Generate a CLAP user interface
    fn register_argument_parser(&self) -> clap::Command;
}

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
