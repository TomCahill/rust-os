#!/bin/bash

rustup override set nightly

# List out the current target triple
rustc --version --verbose

cargo build

# cargo +nightly build --target x86_64-unknown-none.json

# To build this binary, we need to compile for a bare metal target such as thumbv7em-none-eabihf:
# cargo build --target thumbv7em-none-eabihf

# Alternatively, we can compile it for the host system by passing additional linker arguments:
# Linux
# cargo rustc -- -C link-arg=-nostartfiles
# Windows
# cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
# cargo rustc -- -C link-args="-e __start -static -nostartfiles"