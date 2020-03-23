## VMWare Fusion Wrapper for MacOS

[![Actions Status](https://github.com/thepacketgeek/vmware-fusion-wrapper/workflows/Cargo/badge.svg)](https://github.com/thepacketgeek/vmware-fusion-wrapper/actions)

```
vms 0.1.0
VMWare Fusion Manager

USAGE:
    vms [FLAGS] [vm-path] <SUBCOMMAND>

FLAGS:
    -d, --debug      Show debug logs
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <vm-path>    Path the Virtual Machines folder [default: ~/Documents/Virtual Machines.localized/]

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    list       Show available VMs (* depicts an active VM)
    start      Start a VM by name
    stop       Stop a VM by name
    suspend    Suspend a VM by name
```

## Listing VMs
Browse available VMs by display name, and 
```
$ vms list
vSRX
GNS3 VM
Ubuntu19.10
XRv1*
```

# Managing VMs
You can quickly stop, start, or suspend VMs by display name, no need to know the `.vmx` path

```
$ vms start vSRX
started vSRX
```
