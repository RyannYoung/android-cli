mod ascii;
/// A basic android CLI that lets you remotely interact with an Android device
/// Author: Ryan Young
mod models;
mod plugins;

use std::fmt::Write;

use android_cli::{init_tcp, parse_args, split_stream};
use ascii::{get_header, get_summary};
use colored::Colorize;
use models::cli::{Cli, ParseResult};
use plugins::{
    clipboard::ClipboardPlugin, hello::HelloPlugin, help::HelpPlugin, ip::IpPlugin, pwd::PwdPlugin,
    set::SetPlugin, shell::ShellPlugin, sysinfo::SysInfoPlugin, walkdir::Walkdir,
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
    writer.write_all(get_header().as_bytes()).await?;
    writer.write_all(get_summary().as_bytes()).await?;

    // Load the CLI utility and it's plugins
    let mut cli = Cli::new();
    cli.load_plugin(HelloPlugin);
    cli.load_plugin(ShellPlugin);
    cli.load_plugin(IpPlugin);
    cli.load_plugin(SysInfoPlugin);
    cli.load_plugin(Walkdir);
    cli.load_plugin(PwdPlugin);
    cli.load_plugin(SetPlugin);
    cli.load_plugin(ClipboardPlugin);

    // Load help last (generates help information for each element)
    cli.load_plugin(HelpPlugin);

    // Create a loop continuously accepting a TCP connection
    loop {
        // Read an incoming command
        let mut line = String::new();

        writer
            .write_all(format!("{}", "CLI /> ".dimmed().bold()).as_bytes())
            .await?;

        reader.read_line(&mut line).await?;
        let cmd = line.split_whitespace().collect::<Vec<_>>()[0].to_owned();

        // Parse the command
        let args = match cli.parse_command(line) {
            ParseResult::Success(result) => match result {
                Some(args) => args,
                None => {
                    writer
                        .write_all("\nUnable to resolve plugin args\n\n".as_bytes())
                        .await?;
                    continue;
                }
            },
            ParseResult::Fail(error) => {
                writer
                    .write_all(format!("\n{}\n", error).as_bytes())
                    .await?;
                continue;
            }
        };

        let mut output = String::new();

        let res = cli.run_command(cmd, args);
        writeln!(&mut output, "\n{res}\n")?;

        writer.write_all(output.as_bytes()).await?;
    }
}
