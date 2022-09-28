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
#![feature(custom_test_frameworks)]
#![recursion_limit = "512"]
use std3::__reexports::x86_64::instructions::interrupts;
mod writers;
pub use writers::*;


#[macro_export]
macro_rules! println {
    ()=>($crate::print!("\n"));
    ($($arg:tt)*)=>($crate::print!("{}\n",format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*)=>($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print_ok {
    ($($arg:tt)*)=>($crate::_print_ok(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print_err {
    ($($arg:tt)*)=>($crate::_print_err(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*)=>($crate::_print_info(format_args!($($arg)*)));
}




#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _print_logo(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        LOGOWRITER.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _print_ok(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        OKWRITER.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _print_err(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        ERRWRITER.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _print_warn(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        WARNWRITER.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _print_info(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        INFOWRITER.lock().write_fmt(args).unwrap();
    });
}

#[test_case]
fn test_print_simple() {
    print!("test_print_simple output\n");
}
#[test_case]
fn test_print_many() {
    for _ in 0..200 {
        print!("test_print_many output\n");
    }
}
#[test_case]
fn test_print_output() {
    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}
