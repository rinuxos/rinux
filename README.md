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

[![Build](https://github.com/AtomicGamer9523/rinux/actions/workflows/code.yml/badge.svg)](https://github.com/AtomicGamer9523/rinux/actions/workflows/code.yml) [![Page](https://github.com/AtomicGamer9523/rinux/actions/workflows/pages.yml/badge.svg)](https://atomicgamer9523.github.io/rinux)

### [Docs](https://atomicgamer9523.github.io/rinux)

### If you would like to help with STD3, please do so [here](https://github.com/AtomicGamer9523/std3)

### Build yourself:

Tools needed to run: [`bootimage`](https://github.com/rust-osdev/bootimage), and [`Qemu`](https://qemu.org). [Enderpearl](https://github.com/AtomicGamer9523/Enderpearl) is now included

Building:

```bash
python x.py build
```

Running:

```bash
python x.py run
```

If you are encountering build errors envolving enderpearl, please run `./epearl --fix` and try again

<br>

### Limitations:

no `std` instead: [STD3](https://www.github.linkrbot.com/std3)

no terminal

<br>

### Build for release:

```shell
python x.py release
```
