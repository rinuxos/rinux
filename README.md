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

<p align="center"><img src="./doc/icon.png"alt="rinux-logo"style="width:10%"/></p>
<h1 align="center">
    <b style="font-size:5vw;font-family:courier;align:center;background:url(./doc/mech.png) repeat center center;background-size:50%;-webkit-text-fill-color:transparent;-webkit-background-clip:text;-moz-background-clip:text;background-clip:text;-webkit-text-stroke:1px rgb(75,45,35);">Rinux</b>
</h1>
<p align="center"style="">Library for writing OSs with <a href="https://www.rust-lang.org/">Rust</a></p><div align="center">

<a><img src="https://img.shields.io/crates/l/std3?label=License"></a><a href="https://www.github.linkrbot.com/std3"><img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/AtomicGamer9523/std3/Page?label=Docs"></a><br><a href="https://rust-osdev.com/"><img src="https://img.shields.io/github/followers/atomicgamer9523?label=AtomicGamer9523%20(Me)&style=social"/></a></div><br><b>Obtaining</b>:<br><samp>Mercurial: </samp><kbd>hg clone linkrbot.com/hg/std3</kbd><br><samp>Git: </samp><kbd>git clone github.com/AtomicGamer9523/std3</kbd>

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
