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

import platform, os
from enum import Enum

__DEFAULT_FILE_BUILD = """This file is used for importing your own projects into Rinux
NOT for building Rinux
Allowed commands are: 'STASIS', 'PRE', and 'POST', all other ones will be ignored
DO NOT CHANGE THE ORDER OF COMMANDS

#PRE(
git clone https://github.com/AtomicGamer9523/std3
git clone https://github.com/AtomicGamer9523/rinuxcore
)


#POST(
echo STASIS Complete
)


#STASIS(
echo YOUR PACKAGES HERE
)"""
__DEFAULT_FILE_CONFIG = """this file is used for configuring Rinux
NOT for building Rinux
Allowed properties are: 'NAME', 'QUIET', and 'VERSION', all other ones will be ignored

Name for the Project, string
#NAME(MyProject)

Project's version, string
#VERSION(v0.1.0)

If no debug info should be printed, will still print errors bool -> 'true' or 'false'
#QUIET(false)"""
__DEFAULT_CONFIG_LIB = """#![no_std]
pub const PROJECT_NAME: &'static str = \"HyperNet\";
pub const PROJECT_VERSION: &'static str = \"v1.0.0\";
pub const QUIET_BOOT: bool = true;"""
__DEFAULT_CONFIG_CARGO = """[package]
name = \"config\"
version = \"0.1.0\""""

class __TextKind(Enum):
    ConfigValue = 0,
    CommandName = 1,
    Command = 2,
    NONE = 3



class Command(object):
    def __init__(self, command: str):
        self.command: str = command

    def __repr__(self) -> str:
        return self.__str__()

    def __str__(self) -> str:
        return("Command: " + self.command)

    def run(self, root: str = "") -> None:
        if platform.system() == 'Windows':
            os.system("cd ./"+root+" && "+self.command)
        else:
            os.system("cd ./"+root+" && "+self.command)

class Operation(object):
    def __init__(self):
        self.name: str = ""
        self.commands: list[Command] = []

    def __repr__(self) -> str:
        return self.__str__()

    def __str__(self) -> str:
        return("Operation: name:" + self.name + ", commands:" + str(self.commands))

    def run(self, root: str = "") -> None:
        for command in self.commands:
            command.run(root)

class EnderPearl(object):
    def __init__(self, prnt: bool = True):
        self.prnt: bool = prnt
        self.operations: list[Operation] = []
    
    def __repr__(self) -> str:
        return self.__str__()

    def __str__(self) -> str:
        return("Enderpearl, print:" + str(self.prnt) + ", operations:" + str(self.operations))

    def newop(self, op: Operation) -> None:
        self.operations.append(op)

    def run(self, name: str) -> bool:
        __runcmd(name, self,self.prnt)



def __tokenize(string: str, config: str) -> EnderPearl:
    config = __read_config(config)
    efile = EnderPearl(True)
    op = Operation()
    opstr = ""
    cmds = ""
    config_str = ""
    txttype = __TextKind.NONE
    for part in string:
        if part == "#":
            op = Operation()
            txttype = __TextKind.CommandName
        elif part == "(":
            txttype = __TextKind.Command
        elif part == ")":
            txttype = __TextKind.NONE
            for part in opstr:
                if part == "\n":
                    if cmds != "":
                        op.commands.append(Command(cmds))
                    cmds = ""
                else:
                    cmds += part
            efile.newop(op)
            op = Operation()
            opstr = ""
        elif part == "$" and txttype == __TextKind.ConfigValue:
            txttype = __TextKind.Command
        elif part == "$" and txttype != __TextKind.ConfigValue:
            txttype = __TextKind.ConfigValue
        elif part == "{" and txttype == __TextKind.ConfigValue:
            pass
        elif part == "}" and txttype == __TextKind.ConfigValue:
            for kv in config.iter():
                if kv.key.lower() == config_str.lower():
                    for partchar in kv.value.chars():
                        opstr += partchar
            config_str = ""
        elif txttype == __TextKind.ConfigValue and part != " ":
            config_str += part
        elif txttype == __TextKind.CommandName and part != " ":
            op.name += part
        elif txttype == __TextKind.Command and part != "\r":
            opstr += part
    return efile

class __ConfigKV(object):
    def __init__(self, key: str, value: str):
        self.key = key
        self.value = value
    def keyz(self) -> str:
        return self.key.lower()
    def valz(self) -> str:
        return self.value

def __read_config(contents: str) -> list[__ConfigKV]:
    keys_and_values = []
    kv1 = __ConfigKV("","")
    key1 = ""
    val1 = ""
    dattype = 0
    for part in contents:
        if part == "#":
            kv1 = __ConfigKV("","")
            dattype = 1
        elif part == "(":
            dattype = 2
        elif part == ")":
            dattype = 0
            kv1 = __ConfigKV(key1, val1)
            keys_and_values.append(kv1)
            key1 = ""
            val1 = ""
        elif dattype == 1:
            key1 += part
        elif dattype == 2:
            val1 += part
    return keys_and_values

def __runcmd(cmd: str, tkn: EnderPearl, root: str = "") -> None:
    for op in tkn.operations:
        if op.name.lower() == "pre":
            op.run(root)

    for op in tkn.operations:
        if op.name.lower() == cmd:
            op.run(root)

    for op in tkn.operations:
        if op.name.lower() == "post":
            op.run(root)

def hlp(color: bool = True) -> None:
    if color == True:
        print("\x1b[38524mEnderPearl\x1b[0m:\n    \"\x1b[38534m--fix\x1b[0m\" \x1b[385208m->\x1b[0m attempts to fix build errors\n    \"\x1b[38534m--gen\x1b[0m\" \x1b[385208m->\x1b[0m generates/modifies files based on your configuration\n    \"\x1b[38534m--nocolor\x1b[0m\" \x1b[385208m->\x1b[0m prints without color\n    \"\x1b[38534mbuild\x1b[0m\" or \"\x1b[38534m.\x1b[0m\" \x1b[385208m->\x1b[0m runs the \"\x1b[38587mbuild\x1b[0m\" command\n    \"\x1b[38534m*\x1b[0m\" \x1b[385208m->\x1b[0m runs the command with that name")
    else:
        print("EnderPearl:\n    \"--fix\" -> attempts to fix build errors\n    \"--gen\" -> generates/modifies files based on your configuration\n    \"--nocolor\" -> prints without color\n    \"build\" or \".\" -> runs the \"build\" command\n    \"*\" -> runs the command with that name")

def __gen(i1: str, i2: str, i3: str, prnt: bool = True, color: bool = True) -> None:
    try:
        f = open("./enderpearl/packages/config/src/lib.rs", "wt")
        data = "#![no_std]\n"
        data += "pub const PROJECT_NAME: &'static str = \"{}\";\npub const PROJECT_VERSION: &'static str = \"{}\";\npub const QUIET_BOOT: bool = {};".format(i1, i2, i3)
        f.write(data)
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
        return

    try:
        f = open("./enderpearl/config.enderpearl", "rt")
        contents2 = f.read()
        f.close()
    except(FileNotFoundError):
        try:
            f = open("./enderpearl/config.enderpearl", "wt")
            f.write(__DEFAULT_FILE_CONFIG)
            f.close()
        except(FileNotFoundError):
            print("Unable to create file: config.enderpearl")
            return
    return __tokenize(contents, contents2)

def __run(cmd: str, tkn: EnderPearl, color: bool = True) -> None:
    if cmd == "build" or cmd == ".":
        __runcmd("build",tkn,"")
    elif cmd == "pre" or cmd == "post":
        print("Sorry, you may not use self special operation")
        return
    elif cmd == "" or cmd == "help":
        hlp(color)
    else:
        __runcmd(cmd,tkn,"") 

def run(argv: list) -> None:
    tkn = __get_tkn()
    color = True

    for arg in argv:
        if arg.startswith("--"):
            if arg.startswith("--no-color") or arg.startswith("--nocolor") or arg.startswith("--no-colour") or arg.startswith("--nocolour") or arg.startswith("--n"):
                color = False
            if arg.startswith("--help") or arg.startswith("-h"):
                hlp(color)
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
                return
            if arg.startswith("--stasis"):
                try:
                    f = open("enderpearl/build.enderpearl", "rt")
                    contents = f.read()
                    f.close()
                    stasisz: EnderPearl = __tokenize(contents,"")
                    # print(stasisz)
                    for op in stasisz.operations:
                        if op.name.lower() == "pre":
                            # op.run("")
                            op.run("enderpearl/packages")
                        elif op.name.lower() == "stasis":
                            # op.run("")
                            op.run("enderpearl/packages")
                        elif op.name.lower() == "post":
                            # op.run("")
                            op.run("enderpearl/packages")

                except(FileNotFoundError):
                    try:
                        f = open("enderpearl/build.enderpearl", "wt")
                        f.write(__DEFAULT_FILE_BUILD)
                        f.close()
                    except(FileNotFoundError):
                        print("Unable to create file: build.enderpearl")
                        return
            if arg.startswith("--gen"):
                stasises: list[__ConfigKV] = []
                try:
                    f = open("./enderpearl/config.enderpearl", "rt")
                    contents = f.read()
                    f.close()
                    stasises = __read_config(contents)
                except(FileNotFoundError):
                    try:
                        f = open("./enderpearl/config.enderpearl", "wt")
                        f.write(__DEFAULT_FILE_CONFIG)
                        f.close()
                    except(FileNotFoundError):
                        print("Unable to create file: config.enderpearl")
                        return
                
                name = ""
                version = ""
                boot = ""
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
                __gen(name, version, boot,True, color)
            if arg.startswith("--init"):
                try:
                    f = open("./enderpearl/config.enderpearl", "wt")
                    f.write(__DEFAULT_FILE_CONFIG)
                    f.close()
                except(FileNotFoundError):
                    print("Unable to create file: config.enderpearl")
                try:
                    f = open("./enderpearl/build.enderpearl", "wt")
                    f.write(__DEFAULT_FILE_BUILD)
                    f.close()
                except(FileNotFoundError):
                    print("Unable to create file: build.enderpearl")
        else:
            __run(arg, tkn, True)