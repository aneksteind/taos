# taos

This project is the result of following Philipp Oppermann's [series of tutorials](https://os.phil-opp.com/) in building a small operating system kernel targeting x86-64, along with some minor modifications and additions.

The OS so far features:
- VGA output
- unit and integration tests
- CPU exception handling
- Keyboard hardware interrupt handling
- Virtual memory using paging
- Heap allocation with a choice of three different allocation techniques. Default is fixed-size block allocation
- Cooperative multitasking using `async`/`await`, using custom task Executor and Wakers

## Setup

Install `qemu-system-x86_64`.

Use `rustup` to install Rust nightly. The `rust-toolchain` file specifies that Rust nightly will be used.

## Running

`cargo run` will launch the OS inside of QEMU. It's pretty bare-bones at the moment, the only visible result will be the outputting keyboard input to the VGA display. Under the hood keyboard strokes are added to a queue by a hardware interrupt handler and asynchronously written to VGA memory.

`cargo test` will run all tests.

## TODO
- add a small file system that persists across restarts of the OS
- add a basic shell
- add some VGA convenience functions to be used in a shell (e.g. clear/reset)