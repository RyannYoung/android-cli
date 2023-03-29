use crate::models::{cli::CLIPlugin, command::Command};
use colored::Colorize;
use std::fmt::Write;
use sysinfo::{NetworkExt, System, SystemExt};
pub struct IpPlugin;

impl CLIPlugin for IpPlugin {
    fn register_command(&self, _cli: &mut crate::models::cli::Cli) -> Command {
        Command::new("ip").action(|_args| {
            // Output buffer
            let mut output = String::new();

            // Create a sys object with nothing loaded
            let mut sys = System::new();

            // First we update all information of our `System` struct.
            sys.refresh_networks_list();
            sys.refresh_networks();

            // Write the header
            writeln!(&mut output, "{}", "IP Config".underline().bold()).unwrap();

            for (interface, data) in sys.networks() {
                let mac = &data.mac_address();
                let incoming_mb = bytes_to_mb(&data.total_received());
                let outgoing_mb = bytes_to_mb(&data.total_transmitted());

                writeln!(
                    &mut output,
                    "  {:<15} [{}] {:.2}/{:.2} in/out Mbs",
                    interface, mac, incoming_mb, outgoing_mb
                )
                .unwrap();
            }

            output
        })
    }

    fn register_argument_parser(&self) -> clap::Command {
        clap::Command::new("ip")
    }
}

// Converts bytes to gb
fn bytes_to_mb(bytes: &u64) -> f64 {
    (*bytes as f64) / (1024.0 * 1024.0)
}
