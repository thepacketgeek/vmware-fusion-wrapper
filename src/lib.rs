use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::PathBuf;
use std::process::Command;

use lazy_static::lazy_static;
use regex::Regex;
use walkdir::WalkDir;

lazy_static! {
    static ref NAME_RE: Regex = Regex::new("displayname = \"(.*)\"").unwrap();
}

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

#[derive(Debug)]
pub struct Vm {
    pub name: String,
    pub vmx: PathBuf,
    pub is_running: bool,
}

impl Vm {
    pub fn from_vmx(vmx: PathBuf) -> Result<Self> {
        let name = Self::extract_vmx_name(&vmx)?;
        return Ok(Self {
            name,
            vmx,
            is_running: false,
        });
    }

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
}

pub fn manage_vm(vm: &Vm, action: Action) -> Result<()> {
    Command::new("vmrun")
        .arg(&action.to_string())
        .arg(&vm.vmx.to_str().unwrap())
        .output()?;
    eprintln!("{}ed {}", action, vm.name);
    Ok(())
}

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

pub fn get_vm(path: &str, name: &str) -> Option<Vm> {
    let mut vms: HashMap<String, Vm> = get_vms(path)
        .unwrap()
        .into_iter()
        .map(|vm| (String::from(&vm.name).to_lowercase(), vm))
        .collect();
    vms.remove(&name.to_lowercase())
}

pub fn get_running_vms() -> Result<Vec<String>> {
    let output = Command::new("vmrun").arg("list").output()?;
    Ok(String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.to_owned())
        .collect())
}
