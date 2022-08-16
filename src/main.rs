#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rinuxcore::test_runner)]
#![reexport_test_harness_main = "test_main"]
// extern crate alloc;
use rinuxcore::{
    println,
    task::{executor::Executor, Task},
    BootInfo,
};

bootloader::entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rinuxcore::init(boot_info);
    let mut executor = Executor::new();

    // Stops Rinux from printing error messages when clicking something on the keyboard
    executor.spawn(Task::new(rinuxcore::task::keyboard::init()));
    executor.spawn(Task::new(main()));

    // Prints your key presses, hence the name
    //* Warning don't call rinuxcore::task::keyboard::print_keypresses() if you are using init()
    // executor.spawn(Task::new(rinuxcore::task::keyboard::print_keypresses()));

    executor.run()
}

async fn main() {
    println!("Hello World");
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rinuxcore::print_err!("{}", info);
    rinuxcore::hlt_loop();
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rinuxcore::test_panic_handler(info)
}
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
