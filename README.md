# RustyOS

Basic Kernel implementation on Rust

## Description

A minimal subset of OS with compiled bootloader and simple kernel

### Dependencies

To use RustyOS you must have the following components installed:
* QEMU - Virtualization technology, check https://www.qemu.org/ for installation guide

To install Bootimage and supporting tools, run following commands:
* cargo install cargo-binutils
* cargo install bootimage
* rustup toolchain install nightly
* rustup default nightly
* rustup component add rust-src
* rustup component add llvm-tools-preview

### More about OSDev

To learn more about OSDev check out the following websites:

https://docs.rust-embedded.org/  

https://wiki.osdev.org/Expanded_Main_Page

https://os.phil-opp.com/ 