use crate::models::Cli::{CLIPlugin, Command};
use std::fmt::Write;

use colored::Colorize;
/// System configuration information module
/// Extracts information about the target system
use sysinfo::{Cpu, CpuExt, Disk, DiskExt, NetworkExt, ProcessExt, System, SystemExt};

pub struct SysInfoPlugin;

impl CLIPlugin for SysInfoPlugin {
    fn register_commands(&self, cli: &mut crate::models::Cli::CLI) {
        let command = Command::new("sysinfo")
            .description("Print various system information")
            .action(|_args| {
                let mut output = String::new();

                // Update all the information to the `System` struct
                let mut sys = System::new_all();
                sys.refresh_all();

                // Disk Information
                write!(&mut output, "{}\n", "Disks".underline().bold()).unwrap();
                sys.disks().iter().for_each(|disk| {
                    let name = &disk.name().to_str().unwrap();
                    let available_space = &disk.available_space();
                    let total_space = &disk.total_space();
                    let mount_point = &disk.mount_point().to_str().unwrap();

                    writeln!(
                        &mut output,
                        "  {} [{:.2}/{:.2}GB]",
                        mount_point,
                        bytes_to_gb(total_space - available_space),
                        bytes_to_gb(total_space.to_owned()),
                    )
                    .unwrap()
                });

                // RAM and Swap
                write!(&mut output, "\n{}\n", "Memory".underline().bold()).unwrap();
                writeln!(&mut output, "  Total Memory:    {}", sys.total_memory()).unwrap();
                writeln!(&mut output, "  Used Memory:     {}", sys.used_memory()).unwrap();
                writeln!(&mut output, "  Total Swap:      {}", sys.total_swap()).unwrap();
                writeln!(&mut output, "  Used Swap:       {}", sys.used_swap()).unwrap();

                // System information
                write!(
                    &mut output,
                    "\n{}\n",
                    "System Information".underline().bold()
                )
                .unwrap();
                writeln!(&mut output, "  System name:     {:?}", sys.name().unwrap()).unwrap();
                writeln!(
                    &mut output,
                    "  Kernel version:  {:?}",
                    sys.kernel_version().unwrap()
                )
                .unwrap();
                writeln!(
                    &mut output,
                    "  OS version:      {:?}",
                    sys.os_version().unwrap()
                )
                .unwrap();
                writeln!(
                    &mut output,
                    "  Host name:       {:?}",
                    sys.host_name().unwrap()
                )
                .unwrap();

                // CPU
                write!(&mut output, "\n{}\n", "CPU".underline().bold()).unwrap();
                sys.cpus().iter().for_each(|cpu| {
                    let brand = &cpu.brand();
                    let name = &cpu.name();
                    let vendor_id = &cpu.vendor_id();
                    let usage = &cpu.cpu_usage();

                    writeln!(
                        &mut output,
                        "  {} [{} / {}] - Usage: {:.2}%",
                        name, brand, vendor_id, usage
                    )
                    .unwrap()
                });

                // Processes
                write!(&mut output, "\n{}\n", "Processes".underline().bold()).unwrap();
                for (pid, process) in sys.processes() {
                    writeln!(
                        &mut output,
                        "  [{}] {} ({:.2})",
                        pid,
                        process.name(),
                        process.run_time()
                    )
                    .unwrap();
                }

                output
            });

        cli.add_command(command)
    }
}

// Converts bytes to gb
fn bytes_to_gb(bytes: u64) -> f64 {
    (bytes as f64) / (1024.0 * 1024.0 * 1024.0)
}
