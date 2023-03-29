mod ascii;
/// A basic android CLI that lets you remotely interact with an Android device
/// Author: Ryan Young
mod commands;
mod models;
mod plugins;

use std::fmt::Write;

use android_cli::{init_tcp, parse_args, split_stream};
use ascii::{get_header, get_summary};
use colored::Colorize;
use models::Cli::CLI;
use plugins::{
    hello::HelloPlugin, help::HelpPlugin, ip::IpPlugin, shell::ShellPlugin, walkdir::Walkdir, sysinfo::SysInfoPlugin,
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the arguments
    let (ip, port) = parse_args();

    // Connect to the stream
    let mut stream = init_tcp(ip, port).await?;

    // Split the stream and generate a buffer
    let (mut reader, mut writer) = split_stream(&mut stream);

    // Print the header
    writer.write(get_header().as_bytes()).await?;
    writer.write(get_summary().as_bytes()).await?;

    // Load the CLI utility and it's plugins
    let mut cli = CLI::new();
    cli.load_plugin(HelloPlugin);
    cli.load_plugin(ShellPlugin);
    cli.load_plugin(IpPlugin);
    cli.load_plugin(SysInfoPlugin);
    cli.load_plugin(Walkdir);

    // Load help last (generates help information for each element)
    cli.load_plugin(HelpPlugin);

    // Create a loop continuously accepting a TCP connection
    loop {
        // Read an incoming command
        let mut line = String::new();

        writer.write(format!("{}", "CLI /> ".dimmed().bold()).as_bytes()).await?;

        reader.read_line(&mut line).await?;

        // Parse the command
        let command = CLI::parse_command(line);

        if command.is_empty() {
            continue;
        }

        // Filter out the command and args
        let cmd = &command[0];
        let args = &command[1..];

        let mut output = String::new();

        let res = cli.run_command(cmd, args);
        writeln!(&mut output, "\n{res}\n")?;

        writer.write(output.as_bytes()).await?;
    }
}
