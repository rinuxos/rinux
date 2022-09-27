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
use std3::__reexports::{x86_64,lazy_static,uart_16550};
use lazy_static::lazy_static;
use std3::sync::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub(crate) static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub(crate) fn _print(args: ::std3::fmt::Arguments) {
    use std3::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}

#[macro_export]
macro_rules! serial_print {
    (
        $(
            $arg: tt
        )*
    ) => {
        $crate::serial::_print(
            format_args!(
                $(
                    $arg
                )*
            )
        );
    };
}

#[macro_export]
macro_rules! serial_println {
    ( ) => (
        $crate::serial_print!("\n")
    );

    (
        $fmt: expr
    ) => (
        $crate::serial_print!(
            concat!(
                $fmt,
                "\n"
            )
        )
    );

    (
        $fmt: expr,
        $(
            $arg: tt
        )*
    ) => (
        $crate::serial_print!(
            concat!(
                $fmt,
                "\n"
            ),
            $(
                $arg
            )*
        )
    );
}
