#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub use bootloader::{self,BootInfo};
use memory::BootInfoFrameAllocator;
use core::panic::PanicInfo;
extern crate alloc;
pub mod interrupts;
pub mod vga_buffer;
pub mod allocator;
pub mod memory;
pub mod serial;
pub use x86_64;
pub mod task;
pub mod gdt;


const VERSION: &'static str = "v0.1.0BETA";
const AUTHORS: &'static str = "Atomic";
const RINUX_ART: &'static str = r#"
######   ###  #     #  #     #  #     #
#     #   #   ##    #  #     #   #   #
#     #   #   # #   #  #     #    # #
######    #   #  #  #  #     #     #
#   #     #   #   # #  #     #    # #
#    #    #   #    ##  #     #   #   #
#     #  ###  #     #   #####   #     #
"#;



pub fn init(boot_info: &'static BootInfo, script: &'static str){
    print_logo!(
        "{}\nVersion: {}\nAuthors: [{}]\nScript: {}\n\n",
        RINUX_ART,
        VERSION,
        AUTHORS,
        script
    );

    use x86_64::VirtAddr;
    gdt::init();
    interrupts::init_idt();
    unsafe {
        interrupts::PICS.lock().initialize();
        print_ok!("[OK] interupts initialized\n");
    };
    x86_64::instructions::interrupts::enable();
    print_ok!("[OK] instructions initialized\n");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {
        print_ok!("[OK] VRAM initialized\n");
        memory::init(phys_mem_offset)
    };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    match allocator::init_heap(&mut mapper, &mut frame_allocator) {
        Ok(_) => { print_ok!("[OK] Heap Initialization\n") },
        Err(_) => { print_err!("[ERR] Heap Initialization\n") }
    };

    #[cfg(test)]
    test_main();
}
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

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

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
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
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
    print_err!("[FAIL] allocation error: {:?}", layout);
    panic!("allocation error: {:?}", layout)
}
