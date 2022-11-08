# 
# MIT License
# 
# Copyright (c) 2022 AtomicGamer9523
# 
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
# 
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
# 
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
# 

import enderpearl
import shutil
import sys
import os

def HELP_MESSAGE():
    return """Available commands:
    - build: Builds in debug mode, pass '--release' to build in release
    - run: Builds and runs, pass '--release' to run in release
    - init: Initializes the project
    - gen: Generates the project config files
    - stasis: Installes dependencies from stasis"""


def build(release: bool):
    enderpearl.run("--gen --nocolor --quiet")
    if release == True:
        enderpearl.run("release")
    else:
        enderpearl.run("build")


def clean(full: bool = False):
    if full:
        os.system("cargo clean")
        try:
            shutil.rmtree("./enderpearl/packages")
        except OSError:
            pass
    try:
        shutil.rmtree("./enderpearl/__pycache__")
    except OSError:
       pass


def main():
    clean()
    __release = False

    for arg in sys.argv:
        if arg.__contains__("--clean"):
            clean(True)
            return
        if arg.__contains__("--release"):
            __release = True

    if len(sys.argv) < 2:
        print(HELP_MESSAGE())
        sys.exit(0)
    cmd = str(sys.argv[1])

    if cmd.startswith("--"):
        if cmd.startswith("--clean"):
            clean(True)
            return
        elif cmd.startswith("--release"):
            __release = True
            return
        elif cmd.startswith("--help"):
            print(HELP_MESSAGE())
            return
        print("Unknown option: " + cmd)
        return
    elif cmd.startswith("build"):
        build(__release)
        return
    elif cmd.startswith("run"):
        if __release:
            os.system("cargo run --release")
        else:
            os.system("cargo run")
        return
    elif cmd.startswith("init"):
        os.system("cargo install bootimage")
        enderpearl.run("--mkdir --init --gen --stasis")
        return
    elif cmd.startswith("gen"):
        enderpearl.run("--gen")
        return
    elif cmd.startswith("stasis"):
        enderpearl.run("--stasis")
        return
    elif cmd.startswith("help"):
        print(HELP_MESSAGE())
        return
    else:
        enderpearl.run(cmd)

if __name__ == "__main__":
    try:
        main()
        clean()
    except KeyboardInterrupt:
        print("Interrupted")
        clean()
        sys.exit(0)
