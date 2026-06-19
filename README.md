# mini-os-rust

## OS File Structure
```text
├── .cargo/
│   └── config.toml    <-- Specifies the 32-bit RISC-V target and linker options
├── disk/              <-- File system contents (same as the tutorial)
├── src/
│   ├── main.rs        <-- Kernel: Process management, syscalls, and drivers (replaces kernel.c)
│   ├── boot.S         <-- Assembly bootstrapping (replaces initial boot code)
│   ├── sbi.rs         <-- Pure OpenSBI abstractions (sbi_call, putchar)
│   ├── common.rs      <-- Kernel/user common library: print macros, formatting, utilities
│   ├── panic.rs       <-- Bare-metal panic handler: descriptive diagnostic crash reports
│   └── user/          <-- Separate userland binary folder
│       ├── main.rs    <-- Command-line shell (replaces shell.c)
│       └── lib.rs     <-- User library: system call wrappers (replaces user.c/h)
├── build.rs           <-- Build script: Assembles boot.S and packages disk assets
├── kernel.ld          <-- Kernel: Linker script
├── user.ld            <-- User: Linker script
└── Cargo.toml         <-- Manifest defining dependencies and workspace build targets
```