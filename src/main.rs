#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Rusty{}", "OS");
    loop {

    }
}





/// Panic function
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {

    }
}