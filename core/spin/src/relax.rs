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

//! Strategies that determine the behaviour of locks when encountering contention.

/// A trait implemented by spinning relax strategies.
pub trait RelaxStrategy {
    /// Perform the relaxing operation during a period of contention.
    fn relax();
}

/// A strategy that rapidly spins while informing the CPU that it should power down non-essential components via
/// [`core::hint::spin_loop`].
///
/// Note that spinning is a 'dumb' strategy and most schedulers cannot correctly differentiate it from useful work,
/// thereby misallocating even more CPU time to the spinning process. This is known as
/// ['priority inversion'](https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html).
///
/// If you see signs that priority inversion is occurring, consider switching to `Yield` or, even better, not using a
/// spinlock at all and opting for a proper scheduler-aware lock. Remember also that different targets, operating
/// systems, schedulers, and even the same scheduler with different workloads will exhibit different behaviour. Just
/// because priority inversion isn't occurring in your tests does not mean that it will not occur. Use a scheduler-
/// aware lock if at all possible.
pub struct Spin;

impl RelaxStrategy for Spin {
    #[inline(always)]
    fn relax() {
        core::hint::spin_loop();
    }
}

/// A strategy that yields the current time slice to the scheduler in favour of other threads or processes.
///
/// This is generally used as a strategy for minimising power consumption and priority inversion on targets that have a
/// standard library available. Note that such targets have scheduler-integrated concurrency primitives available, and
/// you should generally use these instead, except in rare circumstances.
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct Yield;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl RelaxStrategy for Yield {
    #[inline(always)]
    fn relax() {
        std::thread::yield_now();
    }
}

/// A strategy that rapidly spins, without telling the CPU to do any powering down.
///
/// You almost certainly do not want to use this. Use [`Spin`] instead. It exists for completeness and for targets
/// that, for some reason, miscompile or do not support spin hint intrinsics despite attempting to generate code for
/// them (i.e: this is a workaround for possible compiler bugs).
pub struct Loop;

impl RelaxStrategy for Loop {
    #[inline(always)]
    fn relax() {}
}
