import enderpearl
import shutil
import sys


def main() -> None:
    enderpearl.extension.RELEASE = False
    clean()
    for arg in sys.argv:
        if arg.__contains__("--release"):
            enderpearl.extension.RELEASE = True
    if len(sys.argv) < 2:
        print(enderpearl.extension.helper())
        sys.exit(0)
    cmd = str(sys.argv[1])
    if cmd.startswith("--"):
        enderpearl.parser.default_run(cmd)
        if cmd.startswith("--release"):
            enderpearl.extension.RELEASE = True
            return
        elif cmd.startswith("--help"):
            print(enderpearl.extension.helper())
            return
        print("Unknown option: " + cmd)
        return
    else:
        enderpearl.extension.command_parser(cmd)


def clean() -> None:
    try:
        shutil.rmtree("./enderpearl/__pycache__")
    except OSError:
       pass

if __name__ == "__main__":
    try:
        main()
        clean()
    except KeyboardInterrupt:
        print("Interrupted")
        clean()
        sys.exit(0)

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