from enderpearl.parser import DEFAULT_FILE_CONFIG, DEFAULT_FILE_BUILD
from enderpearl.parser import tokenize, runcmd, read_config
from enderpearl.parser import EnderPearl, ConfigKV
import shutil, os

__DEFAULT_CONFIG_LIB = """#![no_std]
pub const PROJECT_NAME: &'static str = \"HyperNet\";
pub const PROJECT_VERSION: &'static str = \"v1.0.0\";
pub const QUIET_BOOT: bool = true;"""
__DEFAULT_CONFIG_CARGO = """[package]
name = \"config\"
version = \"0.1.0\""""




def hlp() -> str:
    return """Available commands:
        - build: Builds in debug mode, pass '--release' to build in release
        - run: Builds and runs, pass '--release' to run in release
        - init: Initializes the project
        - gen: Generates the project config files
        - stasis: Installes dependencies from stasis"""

def __gen(i1: str, i2: str, i3: str, prnt: bool = True, color: bool = True) -> None:
    def dgen() -> str:
        data = "#![no_std]\n"
        data += "pub const PROJECT_NAME: &'static str = \"{}\";\npub const PROJECT_VERSION: &'static str = \"{}\";\npub const QUIET_BOOT: bool = {};".format(i1, i2, i3)
        return data
    try:
        f = open("./enderpearl/packages/config/src/lib.rs", "wt")
        f.write(dgen())
        f.close()
    except(FileNotFoundError):
        try:
            f = open("./enderpearl/packages/config/src/lib.rs", "xt")
            f.write(dgen())
            f.close()
        except(FileNotFoundError):
            if prnt:
                if color:
                    print("\x1b[38524mEnderPearl \x1b[385196mError\x1b[0m: \"Unable to create file: ./enderpearl/packages/config/src/lib.rs\"")
                else:
                    print("EnderPearl Error: \"Unable to create file: ./enderpearl/packages/config/src/lib.rs\"")

def __get_tkn() -> EnderPearl:
    contents = ""
    contents2 = ""
    try:
        f = open(".enderpearl", "rt")
        contents = f.read()
        f.close()
    except(FileNotFoundError):
        print("EnderPearl Error: .enderpearl file not found")
        exit(1)

    try:
        f = open("./enderpearl/config.enderpearl", "rt")
        contents2 = f.read()
        f.close()
    except(FileNotFoundError):
        try:
            f = open("./enderpearl/config.enderpearl", "wt")
            f.write(DEFAULT_FILE_CONFIG)
            f.close()
        except(FileNotFoundError):
            print("Unable to create file: config.enderpearl")
            exit(1)
    return tokenize(contents, contents2)

def __run(cmd: str, tkn: EnderPearl, color: bool = True) -> None:
    if cmd == "build" or cmd == ".":
        runcmd("build",tkn,"")
    elif cmd == "pre" or cmd == "post":
        print("Sorry, you may not use self special operation")
        return
    elif cmd == "" or cmd == "help":
        hlp()
    else:
        runcmd(cmd,tkn,"") 

def run(argv: str) -> None:
    tkn = __get_tkn()
    color = True

    for arg in argv.split(" "):
        if arg.startswith("--"):
            if arg.startswith("--no-color") or arg.startswith("--nocolor") or arg.startswith("--no-colour") or arg.startswith("--nocolour") or arg.startswith("--n"):
                color = False
            if arg.startswith("--help") or arg.startswith("-h"):
                hlp()
                return
            if arg.startswith("--mkdir"):
                try:
                    os.mkdir("enderpearl/packages")
                except(FileExistsError):
                    pass
                try:
                    os.mkdir("enderpearl/packages/config")
                except(FileExistsError):
                    pass
                try:
                    os.mkdir("enderpearl/packages/config/src")
                except(FileExistsError):
                    pass
                try:
                    f = open("./enderpearl/packages/config/src/lib.rs", "w")
                    f.write(__DEFAULT_CONFIG_LIB)
                    f.close()
                except(FileExistsError):
                    pass
                try:
                    f = open("./enderpearl/packages/config/Cargo.toml", "w")
                    f.write(__DEFAULT_CONFIG_CARGO)
                    f.close()
                except(FileExistsError):
                    pass 
            if arg.startswith("--fix"):
                __gen("MyProject","v0.1.0","false",False, color)
            if arg.startswith("--stasis"):
                try:
                    os.mkdir("enderpearl/packages")
                except(FileExistsError):
                    pass
                try:
                    f = open("enderpearl/build.enderpearl", "rt")
                    contents = f.read()
                    f.close()
                    stasisz: EnderPearl = tokenize(contents,"")
                    times_ran = 0
                    for op in stasisz.operations:
                        if op.name.lower() == "pre" and times_ran == 0:
                            op.run("enderpearl/packages")
                            times_ran  = 1
                        elif op.name.lower() == "stasis" and times_ran == 1:
                            op.run("enderpearl/packages")
                            times_ran = 2
                        elif op.name.lower() == "post" and times_ran == 2:
                            op.run("enderpearl/packages")
                            times_ran = 3

                except(FileNotFoundError):
                    try:
                        f = open("enderpearl/build.enderpearl", "wt")
                        f.write(DEFAULT_FILE_BUILD)
                        f.close()
                    except(FileNotFoundError):
                        print("Unable to create file: build.enderpearl")
                        return
            if arg.startswith("--gen"):
                stasises: list[ConfigKV] = []
                try:
                    f = open("./enderpearl/config.enderpearl", "rt")
                    contents = f.read()
                    f.close()
                    stasises = read_config(contents)
                except(FileNotFoundError):
                    try:
                        f = open("./enderpearl/config.enderpearl", "wt")
                        f.write(DEFAULT_FILE_CONFIG)
                        f.close()
                    except(FileNotFoundError):
                        print("Unable to create file: config.enderpearl")
                        return
                
                name = ""
                version = ""
                boot = ""
                auto_stasis = True
                for cfg in stasises:
                    if cfg.key.lower() == "name":
                        name = cfg.value
                    elif cfg.key.lower() == "version":
                        version = cfg.value
                    elif cfg.key.lower() == "quiet":
                        if cfg.value.lower() == "true":
                            boot = "true"
                        else:
                            boot = "false"
                    elif cfg.key.lower() == "autostasis":
                        if cfg.value.lower() == "true":
                            auto_stasis = True
                        else:
                            auto_stasis = False
                if auto_stasis:
                    run("--stasis")
                __gen(name, version, boot, True, color)
            if arg.startswith("--init"):
                try:
                    f = open("./enderpearl/config.enderpearl", "wt")
                    f.write(DEFAULT_FILE_CONFIG)
                    f.close()
                except(FileNotFoundError):
                    print("Unable to create file: config.enderpearl")
                try:
                    f = open("./enderpearl/build.enderpearl", "wt")
                    f.write(DEFAULT_FILE_BUILD)
                    f.close()
                except(FileNotFoundError):
                    print("Unable to create file: build.enderpearl")
            if arg.startswith("--clean"): 
                import stat
                def on_rm_error(_f,path,_i):
                    os.chmod( path, stat.S_IWRITE )
                    os.unlink( path )
                try:
                    shutil.rmtree("enderpearl/packages",onerror=on_rm_error)
                except(FileNotFoundError):
                    pass
                os.system("cargo clean")
                
        else:
            __run(arg, tkn, True)

if __name__ == "__main__":
    raise Exception("This file is not meant to be run directly")

# 
# MIT License
# 
# Copyright (c) 2022 AtomicGamer9523
# 
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of self software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
# 
# The above copyright notice and self permission notice shall be included in all
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