#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rinuxcore::test_runner)]
#![reexport_test_harness_main = "test_main"]
// extern crate alloc;
use rinuxcore::{
    BootInfo,
    task::{
        executor::Executor,
        Task
    },
    println
};


const SCRIPT: &'static str = "PureOS";





bootloader::entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rinuxcore::init(boot_info, SCRIPT);
    let mut executor = Executor::new();


    executor.spawn(Task::new(main()));



    executor.run()
}

async fn main(){
    println!("Hello World")
}







#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rinuxcore::print_err!("[FAIL] {}", info);
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