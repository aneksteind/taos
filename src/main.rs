// do not include the standard library
// https://doc.rust-lang.org/1.30.0/book/first-edition/using-rust-without-the-standard-library.html
#![no_std]
// we don't want to use the normal main entrypoint
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(taos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use taos::allocator;
use taos::memory::BootInfoFrameAllocator;
use taos::println;
use taos::task::executor::Executor;
use taos::task::keyboard;
use taos::task::Task;

// With no_std, a custom panic handler needs to be defined
// because the default implementation is in the std library
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    taos::test_panic_handler(info)
}

/*
    With no_main, the entry point to the program must be
    defined and specified.

    The type of the function is () -> !, meaning it diverges,
    because it's not expected to return (it runs the whole time)

    The linker looks for a function called _start by default.

    We use no_mangle to ensure the function name is maintained
    for the linker to find.

    In normal applications, the _start() function is run before
    the main funciton by the runtime to set up whatever is needed
    to run the program, like setting up stack overflow guards or
    setting up backtrace printing on panic.

    In normal Rust applications, the C runtime library crt0 calls
    the Rust runtime, which calls its own start and eventually the
    main function. Here, because we don't have access to crt0, we
    create our own entry point.
*/
entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    /*
        The BootInfo struct passed to the _start function
        contains the memory map as well as an offset which
        maps virtual memory to physical memory (by subtracting
        the offset from the virtual address we get the
        physical address)
    */

    use taos::memory;
    use x86_64::VirtAddr;

    taos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    // taos::hlt_loop();
}
