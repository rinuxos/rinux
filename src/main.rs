#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rinuxcore::test_runner)]
#![reexport_test_harness_main = "test_main"]


#![feature(rinuxcore_task)]
#![feature(rinuxcore_keyboard)]

use rinuxcore::{
    println,
    task::{executor::Executor, Task},
    BootInfo,
    std3
};

#[rinuxcore::main]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rinuxcore::init(&boot_info);// Initializes Rinux
    let mut executor = Executor::new();// Creates new Task Executor

    // Built-in keyboard driver, requires "rinuxcore_keyboard" feature
    executor.spawn(Task::new(rinuxcore::task::keyboard::init()));
    executor.spawn(Task::new(main()));

    executor.run()
}

async fn main() {
    println!("Hello World");
}



#[panic_handler]
fn panic(info: &std3::panic::PanicInfo) -> ! {
    rinuxcore::print_err!("{}", info);
    rinuxcore::hlt_loop();
}