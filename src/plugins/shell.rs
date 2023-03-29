use crate::models::{
    cli::{self},
    command::Command,
};
pub struct ShellPlugin;

impl cli::CLIPlugin for ShellPlugin {
    fn register_command(&self, _cli: &mut cli::Cli) -> Command {
        Command::new("shell").action(|args| {
            let args: Vec<String> = args
                .get_raw("commands")
                .unwrap()
                .map(|os| os.to_str().unwrap().to_string())
                .collect();

            let args = get_command_args(args.as_slice());

            let output = std::process::Command::new(get_shell_environment())
                .args(args)
                .output()
                .unwrap();

            // Generate the stdout and stderr
            let stdout = std::str::from_utf8(&output.stdout).unwrap();
            let stderr = std::str::from_utf8(&output.stderr).unwrap();

            // This is bad but I'm doing it anyway (adds a new line in between)
            let host_output: Vec<&str> = vec![stdout, stderr];

            host_output.join("\n")
        })
    }

    fn register_argument_parser(&self) -> clap::Command {
        clap::Command::new("shell").arg(clap::arg!([commands] ...))
    }
}

/// Get the terminal environment depending on the target platform
///
/// Note: Assumes that windows OS's are running via cmd.exe and linux/Unix
/// systems have access to the sh command.
fn get_shell_environment() -> &'static str {
    match cfg!(target_os = "windows") {
        true => "cmd",
        false => "sh",
    }
}

/// Get the command arguments depending on the target platform
///
/// Note: Assumes that windows OS's are running via cmd.exe and linux/Unix
/// systems have access to the sh command.
fn get_command_args(args: &[String]) -> Vec<String> {
    let custom_command = args.join(" ");

    match cfg!(target_os = "windows") {
        true => vec!["/C".to_string(), custom_command],
        false => vec!["-c".to_string(), custom_command],
    }
}
