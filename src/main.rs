//
// MIT License
//
// Copyright (c) 2022 AtomicGamer9523
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rinuxcore::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rinuxcore::{
    kernel, println,
    task::{executor::Executor, Task},
    BootInfo,

    //? Useful for setting custom project metadata, instead of using enderpearl
    set_config_type,ConfigType,conf::Config
};


kernel!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    //? Useful for setting custom project metadata, instead of using enderpearl
    set_config_type(
        ConfigType::Custom(
            Config::new(
                "MyProject",
                "v0.1.0",
                false
            )
        )
    );

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

#[doc(hidden)]
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &std3::panic::PanicInfo) -> ! {
    rinuxcore::print_err!("{}", info);
    rinuxcore::hlt_loop();
}
#[doc(hidden)]
#[cfg(test)]
#[panic_handler]
fn panic(info: &std3::panic::PanicInfo) -> ! {
    rinuxcore::test_panic_handler(info)
}
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
