## llvm-tools (binutls etc.)
### size
View the section data of the elf file
```cargo size -- -Ax```


## Cross Compiler
### Add the CC for microbit (nRF52833)
```rustup target add thumbv7em-none-eabihf```


## Dependancies
### Cargo
Cortex M runtime
```cargo add cortex-m-rt```
Panic Handler
```cargo add panic-halt```

## Installs
### rustup
Binary Utilties
```rustup component add llvm-tools```

### cargo
#### Wrapper for llvm-tools
```cargo install cargo-binutils```

#### Tools for flashing and debugging to our device
(I had to install dependancies for this [see documentation], but with no luck)
```cargo install cargo-embed```
I then tried to install specifically for a my laptop as the target
```cargo install cargo-embed --target x86_64-apple-darwin```


## Enable Access to Microbit (udev rules)
see https://docs.rust-embedded.org/discovery/microbit/03-setup/linux.html#udev-rules
```sudo touch /etc/udev/rules.d/99-microbit.rules```
Contents
```
# CMSIS-DAP for microbit
SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", MODE:="666"
```
Reload rules
```$ sudo udevadm control --reload-rules```


## Commands
### List Chips
```probe-rs chip list```

### Flash microbit
```cargo embed --chip nRF52833_xxAA```


## Issues
### USB Port
I was using a USB 3 port and the device would not flash. Changing to an older port resolved this issue.