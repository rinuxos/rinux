import platform, os, enderpearl
from enum import Enum

DEFAULT_FILE_BUILD = """This file is used for importing your own projects into Rinux
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
DEFAULT_FILE_CONFIG = """this file is used for configuring Rinux
NOT for building Rinux
Allowed properties are: 'NAME', 'QUIET', and 'VERSION', all other ones will be ignored

Name for the Project, string
#NAME(MyProject)

Project's version, string
#VERSION(v0.1.0)

If no debug info should be printed, will still print errors bool -> 'true' or 'false'
#QUIET(false)

Automatically update Stasises
#AUTOSTASIS(true)"""

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

    def run(self, name: str) -> None:
        runcmd(name, self,"")



def tokenize(string: str, confg: str, prefix: str = "#") -> EnderPearl:
    config = read_config(confg)
    efile = EnderPearl(True)
    op = Operation()
    opstr = ""
    cmds = ""
    config_str = ""
    txttype = __TextKind.NONE
    for part in string:
        if part == prefix:
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
            for kv in config:
                if kv.key.lower() == config_str.lower():
                    opstr += kv.value
            config_str = ""
        elif txttype == __TextKind.ConfigValue and part != " ":
            config_str += part
        elif txttype == __TextKind.CommandName and part != " ":
            op.name += part
        elif txttype == __TextKind.Command and part != "\r":
            opstr += part
    return efile

class ConfigKV(object):
    def __init__(self, key: str, value: str):
        self.key = key
        self.value = value
    def keyz(self) -> str:
        return self.key.lower()
    def valz(self) -> str:
        return self.value

def read_config(contents: str) -> list[ConfigKV]:
    keys_and_values = []
    kv1 = ConfigKV("","")
    key1 = ""
    val1 = ""
    dattype = 0
    for part in contents:
        if part == "#":
            kv1 = ConfigKV("","")
            dattype = 1
        elif part == "(":
            dattype = 2
        elif part == ")":
            dattype = 0
            kv1 = ConfigKV(key1, val1)
            keys_and_values.append(kv1)
            key1 = ""
            val1 = ""
        elif dattype == 1:
            key1 += part
        elif dattype == 2:
            val1 += part
    return keys_and_values

def runcmd(cmd: str, tkn: EnderPearl, root: str = "") -> None:
    for op in tkn.operations:
        if op.name.lower() == "pre":
            op.run(root)

    for op in tkn.operations:
        if op.name.lower() == cmd:
            op.run(root)

    for op in tkn.operations:
        if op.name.lower() == "post":
            op.run(root)

def default_run(cmd: str) -> None:
    contents = ""
    try:
        f = open(".enderpearl", "rt")
        contents = f.read()
        f.close()
    except(FileNotFoundError):
        print("EnderPearl Error: .enderpearl file not found")
        exit(1)
    tkn = tokenize(contents,"")
    if cmd == "build" or cmd == ".":
        runcmd("build",tkn,"")
    elif cmd == "pre" or cmd == "post":
        print("Sorry, you may not use self special operation")
        return
    elif cmd == "" or cmd == "help":
        print(enderpearl.extension.helper())
    else:
        runcmd(cmd,tkn,"")

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