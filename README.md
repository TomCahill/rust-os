# OS in Rust
Following [Writing an OS in Rust Philipp Oppermann's blog](https://os.phil-opp.com/minimal-rust-kernel/)

## Prerequisites
- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [QEMU](https://www.qemu.org/download/)
- Install the rust source code `rustup component add rust-src`
- Install `bootimage` crate `cargo install bootimage`
- Install `llvm-tools-preview` `rustup component add llvm-tools-preview`

## Build and Run
Use `cargo run` to build and run the kernel in QEMU.

Alternatively, you can use the provided scripts to build and run the kernel:
- `./build.sh` to build the kernel.   
- `./run.sh` to run the kernel in QEMU.


## References
- [Writing an OS in Rust](https://os.phil-opp.com/)