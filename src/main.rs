use structopt::StructOpt;

use vms::{get_vm, get_vms, manage_vm, Action};

const VM_PATH: &str = "/Users/matwood/Documents/Virtual Machines.localized/";

#[derive(StructOpt, Debug)]
#[structopt(name = "vms", rename_all = "kebab-case")]
/// VM Manager
struct Args {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    List,
    Start {
        // VM name
        name: String,
    },
    Stop {
        // VM name
        name: String,
    },
    Suspend {
        // VM name
        name: String,
    },
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::from_args();

    match args.cmd {
        Command::List => {
            for vm in get_vms(VM_PATH)? {
                let symbol = if vm.is_running { "*" } else { "" };
                println!("{}{}", vm.name, symbol);
            }
        }
        Command::Start { name } => {
            if let Some(vm) = get_vm(VM_PATH, &name) {
                manage_vm(&vm, Action::Start);
            } else {
                eprintln!("No VM found for '{}'", name);
            }
        }
        Command::Stop { name } => {
            if let Some(vm) = get_vm(VM_PATH, &name) {
                manage_vm(&vm, Action::Stop);
            } else {
                eprintln!("No VM found for '{}'", name);
            }
        }
        Command::Suspend { name } => {
            if let Some(vm) = get_vm(VM_PATH, &name) {
                manage_vm(&vm, Action::Suspend);
            } else {
                eprintln!("No VM found for '{}'", name);
            }
        }
    };

    Ok(())
}
