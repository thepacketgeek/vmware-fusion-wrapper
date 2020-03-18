/// VMWare Fusion VM Helper for MacOS
///
/// Provides a quick way to view and manage VMs
/// by discovering `vmx` files in Documents
use anyhow::Result;
use shellexpand::tilde;
use structopt::StructOpt;

use vms::{get_vm, get_vms, Action};

const VM_PATH: &str = "~/Documents/Virtual Machines.localized/";

#[derive(StructOpt, Debug)]
#[structopt(name = "vms", rename_all = "kebab-case")]
/// VMWare Fusion Manager
struct Args {
    #[structopt(subcommand)]
    cmd: Command,
    /// Path the Virtual Machines folder
    #[structopt(default_value = VM_PATH, set = structopt::clap::ArgSettings::Global)]
    vm_path: String,
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
struct VmOptions {
    // VM name
    name: String,
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
enum Command {
    /// Show available VMs (* depicts an active VM)
    List,
    /// Start a VM by name
    Start(VmOptions),
    /// Stop a VM by name
    Stop(VmOptions),
    /// Suspend a VM by name
    Suspend(VmOptions),
}

fn main() -> Result<()> {
    let args = Args::from_args();

    match args.cmd {
        Command::List => {
            for vm in get_vms(&tilde(&args.vm_path))? {
                let symbol = if vm.is_running { "*" } else { "" };
                println!("{}{}", vm.name, symbol);
            }
        }
        Command::Start(opts) => {
            get_vm(&tilde(&args.vm_path), &opts.name)?.manage(Action::Start)?;
        }
        Command::Stop(opts) => {
            get_vm(&tilde(&args.vm_path), &opts.name)?.manage(Action::Stop)?;
        }
        Command::Suspend(opts) => {
            get_vm(&tilde(&args.vm_path), &opts.name)?.manage(Action::Suspend)?;
        }
    };
    Ok(())
}
