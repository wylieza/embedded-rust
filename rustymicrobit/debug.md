## GDB

### Requires
On Linux
```sudo apt install gdb-multiarch```
On Mac
```brew install arm-none-eabi-gdb```

### Config
Enable GDB server in Embed.toml. Good idea to halt on reset too.

### Debug
Load with 'cargo embed' as usual.

In a second terminal, open GDB. Specify the binary you are debugging.
```arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/rustymicrobit```

Very useful to dumb warnings to std error.
```arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/rustymicrobit 2>/dev/null```

### GDB Debugging
#### Connect to target
```target remote :1337```

#### View register values
```info registers```

#### Disassemble
View disassembly for current function
```disassemble```

#### Demangle disassembly
Demangle symbol names
```set print asm-demangle on```

#### Step Into
```stepi```

#### Reset Processor
```monitor reset```

#### Set a break point
For a line of code
```break main.rs:<line number>```
Info on current break points
```info break```
Delete breakpoint
```delete <break point number>```

#### Continue from current break/halt
```continue```

#### Local variables
Info
```info locals```
Specific variable
```print <variable name>```
Sepcific variable address
```print &<variable name>```
Modify values
```set var <variable name>=<new value>```




