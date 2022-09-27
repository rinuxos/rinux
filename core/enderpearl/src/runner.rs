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

// !Non-finished code
//TODO: Implement a runner system

extern crate alloc;

use alloc::{string::String,boxed::Box};

pub type RunableFunc = Box<dyn Fn(&String,bool)->Result<(),RunnerError>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RunnerError {
    error: &'static str,
}
impl RunnerError {
    pub fn new(error: &'static str) -> Self { Self { error } }
    pub fn to_panic(&self) { panic!("{}", self.error) }
}

pub enum Runner<F=RunableFunc> {
    Default,
    Custom(F)
}

pub(crate) static mut RUNNER: Runner<RunableFunc> = Runner::Default;

pub fn set_runner(runnner: Runner){
    unsafe { RUNNER = runnner };
}


pub(crate) fn run(command: &String, print: bool) {
    unsafe {
        let res = match &RUNNER {
            Runner::Default => {
                default_runner(command, print)
            },
            Runner::Custom(v) => {
                v(command,print)
            }
        };
        match res {
            Ok(_) => {},
            Err(e) => {
                //TODO: Make better fail
                e.to_panic();
            }
        }
    };
}

pub(crate) fn default_runner(command: &String, _print: bool) -> Result<(),RunnerError> {
    if command != "" {
        RunnerError::new("Please use custom enderpearl runners for now");
    }
    Ok(())
}
