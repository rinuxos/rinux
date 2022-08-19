<!-- 
MIT License

Copyright (c) 2022 AtomicGamer9523

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
-->

# Rinux

## OS written in rust

[![Build](https://github.com/AtomicGamer9523/rinux/actions/workflows/code.yml/badge.svg)](https://github.com/AtomicGamer9523/rinux/actions/workflows/code.yml)

### Build yourself:

Tools needed to run: [`enderpearl`](./doc/enderpearl.md), [`bootimage`](https://github.com/rust-osdev/bootimage), and [`qemu`](https://qemu.org)

1. Modify project's metadata in [config.enderpearl](./config.enderpearl)

2. Import your project via [build.enderpearl](./build.enderpearl)

3. Build & Run:

```shell
enderpearl run
```

or

```shell
cargo run
```

<br>

### Limitations:

no `std` (working on a replacement)

no terminal

<br>

### Build for release:

```shell
enderpearl release
```
