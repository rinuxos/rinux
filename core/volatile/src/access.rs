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

/// Helper trait that is implemented by [`ReadWrite`] and [`ReadOnly`].
pub trait Readable {}

/// Helper trait that is implemented by [`ReadWrite`] and [`WriteOnly`].
pub trait Writable {}

/// Zero-sized marker type for allowing both read and write access.
#[derive(Debug, Copy, Clone)]
pub struct ReadWrite;
impl Readable for ReadWrite {}
impl Writable for ReadWrite {}

/// Zero-sized marker type for allowing only read access.
#[derive(Debug, Copy, Clone)]
pub struct ReadOnly;

impl Readable for ReadOnly {}

/// Zero-sized marker type for allowing only write access.
#[derive(Debug, Copy, Clone)]
pub struct WriteOnly;
impl Writable for WriteOnly {}