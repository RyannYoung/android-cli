use colored::Colorize;

use crate::models::Cli::{CLIPlugin, Command};

pub struct IpPlugin;

impl CLIPlugin for IpPlugin {
    fn register_commands(&self, cli: &mut crate::models::Cli::CLI) {
        let command = Command::new("ip")
            .description("Show network information of target")
            .action(|_args| match cfg!(target_os = "windows") {
                true => get_windows(),
                false => get_unix(),
            });
        cli.add_command(command);
    }
}

fn get_windows() -> String {
    let adapters = ipconfig::get_adapters().unwrap();

    let adapter_output = adapters
        .iter()
        .map(|adapter| {
            let mut adapter_output = String::new();

            // Collect information about the adapter
            let adapter_name = adapter.friendly_name();
            let ip_addrs = adapter.ip_addresses();

            // Construct the output string for the adapter
            let adapter_name = format!("{}", adapter_name.underline().bold());
            let ip_addrs = ip_addrs
                .iter()
                .map(|ip| {
                    format!(
                        "{:>4}: {}",
                        match ip.is_ipv4() {
                            true => "v4",
                            false => match ip.is_ipv6() {
                                true => "v6",
                                false => "v?",
                            },
                        },
                        ip.to_string()
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            adapter_output.push_str(&adapter_name);
            adapter_output.push_str("\n");
            adapter_output.push_str(&ip_addrs);

            adapter_output
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    adapter_output
}

fn get_unix() -> String {
    todo!()
}
