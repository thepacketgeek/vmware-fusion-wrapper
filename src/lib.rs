use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use walkdir::WalkDir;

lazy_static! {
    static ref NAME_RE: Regex = Regex::new("displayname = \"(.*)\"").unwrap();
}

/// Actions available to run for VMs with `vmrun` CLI
#[derive(Copy, Clone, Debug)]
pub enum Action {
    Start,
    Stop,
    Suspend,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Action::Start => "start",
            Action::Stop => "stop",
            Action::Suspend => "suspend",
        };
        write!(f, "{}", value)
    }
}

/// Representation of a VM with gathered info from `vmrun` and the `.vmx` file
#[derive(Debug)]
pub struct Vm {
    pub name: String,
    pub vmx: PathBuf,
    pub is_running: bool,
}

impl Vm {
    /// Gather VM info from the `.vmx` file
    pub fn from_vmx(vmx: PathBuf) -> Result<Self> {
        let name = Self::extract_vmx_name(&vmx)?;
        Ok(Self {
            name,
            vmx,
            is_running: false,
        })
    }

    /// Extract the VM Display Name from the `.vmx` file
    fn extract_vmx_name(vmx: &PathBuf) -> Result<String> {
        let input = File::open(vmx)?;
        let buffered = BufReader::new(input);

        for line in buffered.lines() {
            if let Ok(line) = line {
                if let Some(caps) = NAME_RE.captures(&line) {
                    return Ok(caps.get(1).expect("displayname match").as_str().to_string());
                }
            }
        }
        Ok("Unknown".to_string())
    }

    /// Run an action using `vmrun`
    pub fn manage(&self, action: Action) -> Result<()> {
        Command::new("vmrun")
            .arg(&action.to_string())
            .arg(&self.vmx.to_str().unwrap())
            .output()
            .context("Running vmrun")?;
        eprintln!("{}ed {}", action, self.name);
        Ok(())
    }
}

/// Walk the given path, searching for `.vmx` files
pub fn get_vms(path: &str) -> Result<Vec<Vm>> {
    let running_vms = get_running_vms()?;

    let mut vms: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|d| {
            if let Some(e) = d.path().extension() {
                e == "vmx"
            } else {
                false
            }
        })
        .filter_map(|entry| Vm::from_vmx(entry.into_path()).ok())
        .collect();

    for mut vm in vms.iter_mut() {
        if running_vms.contains(&vm.vmx.to_str().unwrap().to_owned()) {
            vm.is_running = true;
        }
    }
    Ok(vms)
}

/// Search for a given VM by name
pub fn get_vm(path: &str, name: &str) -> Result<Vm> {
    get_vms(path)?
        .into_iter()
        .find(|vm| String::from(&vm.name).to_lowercase() == vm.name.to_lowercase())
        .ok_or(anyhow!("No VM found matching '{}'", name))
}

/// Gather list of running VMs (by .vmx path) from `vmrun`
pub fn get_running_vms() -> Result<Vec<String>> {
    let output = Command::new("vmrun").arg("list").output()?;
    Ok(String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.to_owned())
        .collect())
}
