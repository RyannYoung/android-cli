use crate::models::Cli::{self, Command};

pub struct ShellPlugin;

impl Cli::CLIPlugin for ShellPlugin {
    fn register_commands(&self, cli: &mut Cli::CLI) {
        let command = Command::new("shell")
            .description("Run shell commands")
            .action(|args| {
                // Guard if no args are supplied
                if args.is_empty() {
                    return "You need to supply command arguments".to_string();
                }

                let args = get_command_args(args);
                let output = std::process::Command::new(get_shell_environment())
                    .args(args)
                    .output()
                    .unwrap();

                // Generate the stdout and stderr
                let stdout = std::str::from_utf8(&output.stdout).unwrap();
                let stderr = std::str::from_utf8(&output.stderr).unwrap();

                // This is bad but I'm doing it anyway (adds a new line in between)
                let host_output: Vec<&str> = vec![stdout, stderr];
                let host_output = host_output.join("\n");

                host_output
            });

        cli.add_command(command);
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
