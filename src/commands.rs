// /// A list of commands to run on the target host

// #[derive(Debug)]
// pub enum CommandType {
//     Help,
//     Shell,
//     SysInfo,
//     IpAddr,
//     Unknown,
// }

// impl CommandType {
//     fn execute_command(&self, args: Vec<&str>) -> String {
//         match self {
//             CommandType::Help => executors::help::run(args),
//             CommandType::Shell => executors::shell::run(args),
//             CommandType::IpAddr => executors::ip::run(args),
//             CommandType::Unknown => String::from("Unknown Command"),
//             CommandType::SysInfo => executors::sysinfo::run(args),
//         }
//     }

//     pub fn match_command<T: AsRef<str>>(input: T) -> Self {
//         match input.as_ref().to_lowercase().as_str() {
//             "help" => CommandType::Help,
//             "shell" | "execute" | "command" => CommandType::Shell,
//             "ip" | "ipaddr" | "ipconfig" => CommandType::IpAddr,
//             "info" | "sysinfo" | "system" | "get_sys" => CommandType::SysInfo,
//             _ => CommandType::Unknown,
//         }
//     }
// }

// pub struct Command<'a> {
//     target: CommandType,
//     args: Vec<&'a str>,
// }

// impl<'a> Command<'a> {
//     fn new(target: CommandType, args: Vec<&'a str>) -> Self {
//         Self { target, args }
//     }

//     pub fn from(input: &str) -> Command {
//         // Split the command args
//         let args = input.trim().split(" ").collect::<Vec<_>>();

//         // Match the command target
//         let target = CommandType::match_command(args[0]);

//         Command::new(target, args)
//     }

//     pub fn execute(self) -> String {
//         self.target.execute_command(self.args)
//     }
// }
