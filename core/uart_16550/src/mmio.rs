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

use std3::{
    fmt,
    sync::atomic::{AtomicPtr, Ordering},
};

use crate::LineStsFlags;

/// A memory-mapped UART.
pub struct MmioSerialPort {
    data: AtomicPtr<u8>,
    int_en: AtomicPtr<u8>,
    fifo_ctrl: AtomicPtr<u8>,
    line_ctrl: AtomicPtr<u8>,
    modem_ctrl: AtomicPtr<u8>,
    line_sts: AtomicPtr<u8>,
}

impl MmioSerialPort {
    /// Creates a new UART interface on the given memory mapped address.
    ///
    /// This function is unsafe because the caller must ensure that the given base address
    /// really points to a serial port device.
    #[rustversion::attr(since(1.61), const)]
    pub unsafe fn new(base: usize) -> Self {
        let base_pointer = base as *mut u8;
        Self {
            data: AtomicPtr::new(base_pointer),
            int_en: AtomicPtr::new(base_pointer.add(1)),
            fifo_ctrl: AtomicPtr::new(base_pointer.add(2)),
            line_ctrl: AtomicPtr::new(base_pointer.add(3)),
            modem_ctrl: AtomicPtr::new(base_pointer.add(4)),
            line_sts: AtomicPtr::new(base_pointer.add(5)),
        }
    }

    /// Initializes the memory-mapped UART.
    ///
    /// The default configuration of [38400/8-N-1](https://en.wikipedia.org/wiki/8-N-1) is used.
    pub fn init(&mut self) {
        let self_int_en = self.int_en.load(Ordering::Relaxed);
        let self_line_ctrl = self.line_ctrl.load(Ordering::Relaxed);
        let self_data = self.data.load(Ordering::Relaxed);
        let self_fifo_ctrl = self.fifo_ctrl.load(Ordering::Relaxed);
        let self_modem_ctrl = self.modem_ctrl.load(Ordering::Relaxed);
        unsafe {
            // Disable interrupts
            self_int_en.write(0x00);

            // Enable DLAB
            self_line_ctrl.write(0x80);

            // Set maximum speed to 38400 bps by configuring DLL and DLM
            self_data.write(0x03);
            self_int_en.write(0x00);

            // Disable DLAB and set data word length to 8 bits
            self_line_ctrl.write(0x03);

            // Enable FIFO, clear TX/RX queues and
            // set interrupt watermark at 14 bytes
            self_fifo_ctrl.write(0xC7);

            // Mark data terminal ready, signal request to send
            // and enable auxilliary output #2 (used as interrupt line for CPU)
            self_modem_ctrl.write(0x0B);

            // Enable interrupts
            self_int_en.write(0x01);
        }
    }

    fn line_sts(&mut self) -> LineStsFlags {
        unsafe { LineStsFlags::from_bits_truncate(*self.line_sts.load(Ordering::Relaxed)) }
    }

    /// Sends a byte on the serial port.
    pub fn send(&mut self, data: u8) {
        let self_data = self.data.load(Ordering::Relaxed);
        unsafe {
            match data {
                8 | 0x7F => {
                    wait_for!(self.line_sts().contains(LineStsFlags::OUTPUT_EMPTY));
                    self_data.write(8);
                    wait_for!(self.line_sts().contains(LineStsFlags::OUTPUT_EMPTY));
                    self_data.write(b' ');
                    wait_for!(self.line_sts().contains(LineStsFlags::OUTPUT_EMPTY));
                    self_data.write(8)
                }
                _ => {
                    wait_for!(self.line_sts().contains(LineStsFlags::OUTPUT_EMPTY));
                    self_data.write(data);
                }
            }
        }
    }

    /// Receives a byte on the serial port.
    pub fn receive(&mut self) -> u8 {
        let self_data = self.data.load(Ordering::Relaxed);
        unsafe {
            wait_for!(self.line_sts().contains(LineStsFlags::INPUT_FULL));
            self_data.read()
        }
    }
}

impl fmt::Write for MmioSerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}
