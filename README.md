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

<p align="center"><img src="./doc/icon.png"alt="rinux-logo"style="width:10%"/></p><h1 align="center"><b style="font-size:5vw;font-family:courier;align:center;background:url(./doc/mech.png) repeat center center;background-size:8vw;-webkit-text-fill-color:transparent;-webkit-background-clip:text;-moz-background-clip:text;background-clip:text;-webkit-text-stroke:1px rgb(75,45,35);">Rinux</b></h1><p align="center"style="">Library for writing OSs with <a href="https://www.rust-lang.org/">Rust</a></p><div align="center"><a><img src="https://img.shields.io/crates/l/std3?label=License"></a> <a href="https://www.github.linkrbot.com/rinux/rinuxcore"><img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/AtomicGamer9523/rinux/Doc?label=Docs"></a><br><a href="https://www.github.com/AtomicGamer9523"><img src="https://img.shields.io/github/followers/atomicgamer9523?label=AtomicGamer9523%20(Me)&style=social"/></a></div><br><h3><b>Obtaining</b>:</h3><samp>Mercurial: </samp><kbd>hg clone linkrbot.com/hg/rinux</kbd><br><samp>Git: </samp><kbd>git clone github.com/AtomicGamer9523/rinux</kbd><h3><b>Documentation</b>: </h3>

[here](./doc/README.md)
<h3><b>Note</b>:</h3><p>This project depends on <a href="https://github.com/AtomicGamer9523/std3">STD3</a>, which is still <strong>WIP</strong>. If you are able to contribute to it please do so! :)</a></p><h3><b>Running</b>:</h3><p>Items expected in PATH:<ul><li><a href="https://www.python.org/">Python</a></li><li><a href="https://www.qemu.org/">Qemu</a></li><li><a href="https://www.rust-lang.org/">Rust</a></li></ul>

```powershell
# First time only, installs dependencies
./epearl init

# Builds the os and kernel
./epearl build

# Runs the os in qemu
./epearl run

# If you need help
./epearl help
```
