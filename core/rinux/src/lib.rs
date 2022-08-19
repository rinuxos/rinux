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
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub use bootloader::{self, BootInfo,entry_point};
use core::panic::PanicInfo;
use memory::BootInfoFrameAllocator;
pub mod conf;
extern crate alloc;
pub mod allocator;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga_buffer;
pub use x86_64;
pub mod gdt;
pub mod task;

#[cfg(feature = "epearl")]
pub use epearl;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BuildType {
    Test = 0,
    Debug = 1,
    Release = 2,
}

static mut CONFIGURED: bool = false;
static mut CONFIGTYPE: ConfigType = ConfigType::File;
pub(crate) static mut CONFIG: conf::Config = conf::Config::cnst();
static mut TEST_MODE: BuildType = BuildType::Test;
const VERSION: &'static str = "v0.1.2-RELEASE";
const AUTHORS: &'static str = "Atomic";
const RINUX_ART: &'static str = r#"######   ###  #     #  #     #  #     #
#     #   #   ##    #  #     #   #   #
#     #   #   # #   #  #     #    # #
######    #   #  #  #  #     #     #
#   #     #   #   # #  #     #    # #
#    #    #   #    ##  #     #   #   #
#     #  ###  #     #   #####   #     #
"#;

/// Enum for Configuration Data Specification
#[derive(Debug, Clone, Copy)]
pub enum ConfigType {
    File,
    Custom(conf::Config),
}

/// Used for setting source for rinux to get it's config from
pub fn set_config_type(config_type: ConfigType) {
    match config_type {
        ConfigType::File => {
            unsafe {
                CONFIG = conf::Config::cnst().get_config(conf::ConfigType::File);
            };
        }
        ConfigType::Custom(data) => unsafe {
            CONFIG = conf::Config::cnst().get_config(conf::ConfigType::UserDefined(data));
        },
    };
    unsafe {
        CONFIGURED = true;
    };
}

/// Initializes the core of Rinux
pub fn init(boot_info: &'static BootInfo) {
    unsafe {
        if CONFIGURED != true {
            set_config_type(ConfigType::File);
        }

        match CONFIGTYPE {
            ConfigType::Custom(data) => {
                if data.project_name == "" || data.project_version == "" {
                    panic!("Please use the enderpearl build system");
                };
            }
            ConfigType::File => {
                if CONFIG.project_name == "" || CONFIG.project_version == "" {
                    panic!("Please use the enderpearl build system");
                };
            }
        }

        match env!("BUILD_TYPE") {
            v => {
                if v == "test" {
                    TEST_MODE = BuildType::Test
                } else if v == "debug" {
                    TEST_MODE = BuildType::Debug;
                } else if v == "release" {
                    TEST_MODE = BuildType::Release;
                } else {
                    TEST_MODE = BuildType::Debug;
                };
            }
        }

        vga_buffer::_print_logo(format_args!("{}\n", RINUX_ART));
        if VERSION.ends_with("-RELEASE") {
            if TEST_MODE == BuildType::Test {
                vga_buffer::_print_warn(format_args!("Rinux Version: {}-TEST\n", VERSION));
            } else if TEST_MODE == BuildType::Debug {
                vga_buffer::_print_logo(format_args!("Rinux Version: {}\n", VERSION));
            } else if TEST_MODE == BuildType::Release {
                vga_buffer::_print_logo(format_args!("Rinux Version: {}\n", VERSION));
            } else {
                panic!("Invalid BuildType");
            }
        } else {
            if TEST_MODE == BuildType::Test {
                vga_buffer::_print_warn(format_args!("Rinux Version: {}-TEST\n", VERSION));
            } else if TEST_MODE == BuildType::Debug {
                vga_buffer::_print_warn(format_args!("Rinux Version: {}\n", VERSION));
            } else if TEST_MODE == BuildType::Release {
                panic!("Please match VERSION and ENV.BUILD_TYPE");
            } else {
                panic!("Invalid BuildType");
            }
        }

        vga_buffer::_print_logo(format_args!(
            "Rinux Authors: [{}]\nScript: {}\nScript Version: {}\n\n",
            AUTHORS, CONFIG.project_name, CONFIG.project_version
        ));

        use x86_64::VirtAddr;
        gdt::init();
        interrupts::init_idt();
        interrupts::PICS.lock().initialize();
        if CONFIG.quiet_boot != true {
            print_ok!("[OK] Interupts initialized\n");
        };
        x86_64::instructions::interrupts::enable();
        if CONFIG.quiet_boot != true {
            print_ok!("[OK] Instructions initialized\n");
        };

        let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
        let mut mapper = {
            if CONFIG.quiet_boot != true {
                print_ok!("[OK] VRAM initialized\n");
            }
            memory::init(phys_mem_offset)
        };
        let mut frame_allocator = BootInfoFrameAllocator::init(&boot_info.memory_map);

        match allocator::init_heap(&mut mapper, &mut frame_allocator) {
            Ok(_) => {
                if CONFIG.quiet_boot != true {
                    print_ok!("[OK] Heap Initialization\n");
                };
            }
            Err(_) => {
                print_err!("[ERR] Heap Initialization\n");
            }
        };
    }

    #[cfg(test)]
    test_main();
}

/// Useful for testing
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

/// Runs Tests
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

/// It's in its name
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

/// Enum For Qemu Exit codes, sometimes useful
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Quits Qemu (stops)
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// Loop used to just do nothing
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
use bootloader::entry_point;
#[cfg(test)]
entry_point!(test_kernel_main);
#[cfg(test)]
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);
    test_main();
    hlt_loop();
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
