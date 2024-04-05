## llvm-tools (binutls etc.)
### size
View the section data of the elf file
```cargo size -- -Ax```



## Installs
### rustup
Binary Utilties
```rustup component add llvm-tools```

### cargo
Wrapper for llvm-tools
```cargo install cargo-binutils```
Tools for flashing and debugging to our device
(I had to install dependancies for this [see documentation], but with no luck)
```cargo install cargo-embed```
I then tried to install specifically for a my laptop as the target
```cargo install cargo-embed --target x86_64-apple-darwin```