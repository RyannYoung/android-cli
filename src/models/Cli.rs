pub struct CLI {
    pub commands: Vec<Command>,
}

impl CLI {
    pub fn new() -> Self {
        // Create a base implementation
        Self {
            commands: Vec::new(),
        }
    }

    pub fn load_plugin<P: CLIPlugin>(&mut self, plugin: P) {
        plugin.register_commands(self);
    }

    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command)
    }

    // Match and invoke a command
    pub fn run_command<S: Into<String>>(&self, name: S, args: &[String]) -> String {
        let name: String = name.into();

        match self.commands.iter().find(|cmd| cmd.name == name) {
            Some(res) => res.run(args),
            None => format!("No plugin found: {}", name),
        }
    }

    // Parses the command from the
    pub fn parse_command<S: Into<String>>(input: S) -> Vec<String> {
        let input: String = input.into();

        input.split_whitespace().map(|s| s.to_string()).collect()
    }
}

pub trait CLIPlugin {
    // Register the command to the CLI interface
    fn register_commands(&self, cli: &mut CLI);
}

/// Represents a structured command/plugin module
pub struct Command {
    // The invoking name of the plugin
    pub name: String,

    // A brief description about the plugin
    pub description: String,

    // The extended description about the plugin
    pub long_description: Option<String>,

    // The command action
    action: Box<dyn Fn(&[String]) -> String>,
}

impl Command {
    // Create a new blanket instance of a command
    pub fn new<S: Into<String>>(name: S) -> Self {
        Command {
            name: name.into(),
            description: String::new(),
            long_description: None,
            action: Box::new(|_args| String::new()),
        }
    }

    pub fn description<S: Into<String>>(mut self, desc: S) -> Self {
        self.description = desc.into();
        self
    }

    pub fn long_description<S: Into<String>>(mut self, desc: S) -> Self {
        self.long_description = Some(desc.into());
        self
    }

    pub fn action<F: 'static + Fn(&[String]) -> String>(mut self, action: F) -> Self {
        self.action = Box::new(action);
        self
    }

    pub fn run(&self, args: &[String]) -> String {
        (self.action)(args)
    }
}
