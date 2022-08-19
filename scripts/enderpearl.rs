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

use std::{
    process,
    env,
    io::{
        self,
        prelude::*,
    },
    fs::File
};
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum TextKind {
    ConfigValue,
    CommandName,
    Command,
    None
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Token {
    SIGN_HASHTAG,
    SIGN_OPENING_BRACKET,
    SIGN_CLOSING_BRACKET,
    SIGN_DOLLAR,
    SIGN_OPENING_CURLY_BRACKET,
    SIGN_CLOSING_CURLY_BRACKET,
    TEXT_COMMAND_NAME,
    TEXT_COMMAND
}
impl Token {
    pub(crate) fn tokenize(string: String, config: String) -> ( Vec<Token>, EnderPearl ) {
        let config = read_config(config);
        let mut vec = vec![];
        let mut efile: EnderPearl = EnderPearl {
            print: true,
            operations: Vec::new()
        };
        let mut op: Operation = Operation::new();
        let mut opstr: String = String::new();
        let mut cmds: String = String::new();
        let mut config_str: String = String::new();
        let mut txttype: TextKind = TextKind::None;
        for part in string.chars() {
            if part == '#' {
                op = Operation::new();
                vec.push(Token::SIGN_HASHTAG);
                txttype = TextKind::CommandName;
            } else if part == '(' {
                vec.push(Token::SIGN_OPENING_BRACKET);
                txttype = TextKind::Command;
            } else if part == ')' {
                vec.push(Token::SIGN_CLOSING_BRACKET);
                txttype = TextKind::None;
                for part in opstr.clone().chars() {
                    if part == '\n' {
                        if cmds != "" {
                            op.commands.push(Command::new(cmds));
                        }
                        cmds = String::new();
                    } else {
                        cmds.push(part);
                    }
                }
                efile.newop(op);
                op = Operation::new();
                opstr = String::new();
            } else if part == '$' && txttype == TextKind::ConfigValue {
                vec.push(Token::SIGN_DOLLAR);
                txttype = TextKind::Command;
            } else if part == '$' && txttype != TextKind::ConfigValue {
                vec.push(Token::SIGN_DOLLAR);
                txttype = TextKind::ConfigValue;
            } else if part == '{' && txttype == TextKind::ConfigValue {
                vec.push(Token::SIGN_OPENING_CURLY_BRACKET);
            } else if part == '}' && txttype == TextKind::ConfigValue {
                vec.push(Token::SIGN_CLOSING_CURLY_BRACKET);
                for kv in config.iter() {
                    if kv.key.to_lowercase() == config_str.to_lowercase() {
                        for partchar in kv.value.chars() {
                            opstr.push(partchar);
                        };
                    }
                }
                config_str = String::new();
            } else if txttype == TextKind::ConfigValue && part != ' ' {
                config_str.push(part);
                vec.push(Token::TEXT_COMMAND);
            } else if txttype == TextKind::CommandName && part != ' ' {
                op.name.push(part);
                vec.push(Token::TEXT_COMMAND_NAME);
            } else if txttype == TextKind::Command && part != '\r'{
                opstr.push(part);
                vec.push(Token::TEXT_COMMAND);
            }
        }
        return (
            vec,
            efile
        );
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ConfigKV {
    pub key: String,
    pub value: String,
}
impl ConfigKV {
    pub fn new() -> ConfigKV {
        ConfigKV {
            key: String::new(),
            value: String::new(),
        }
    }
    pub fn from(key: String, value: String) -> ConfigKV {
        ConfigKV {
            key,
            value
        }
    }
}

pub(crate) fn read_config(contents: String) -> Vec<ConfigKV> {
    let mut keys_and_values: Vec<ConfigKV> = Vec::new();
    #[allow(unused_assignments)]
    let mut kv1: ConfigKV = ConfigKV::new();
    let mut key1: String = String::new();
    let mut val1: String = String::new();
    let mut dattype: u8 = 0;
    #[allow(unused_assignments)]
    for part in contents.chars() {
        if part == '#' {
            kv1 = ConfigKV::new();
            dattype = 1;
        } else if part == '(' {
            dattype = 2;
        } else if part == ')' {
            dattype = 0;
            kv1 = ConfigKV::from(key1, val1);
            keys_and_values.push(kv1);
            key1 = String::new();
            val1 = String::new();
        } else if dattype == 1 {
            key1.push(part);
        } else if dattype == 2 {
            val1.push(part);
        }
    }
    keys_and_values
}



#[derive(Clone, Debug)]
pub(crate) struct Command {
    #[allow(dead_code)]
    pub(crate) command: String
}
impl Command {
    pub(crate) fn new(command: String) -> Command {
        Command {
            command
        }
    }

    pub(crate) fn run(&self, print: bool) {
        let output = if cfg!(target_os = "windows") {
            process::Command::new("cmd")
            .args(["/C", self.command.as_str()])
            .output()
            .expect("failed to execute process")
        } else {
            process::Command::new("sh")
            .arg("-c")
            .arg(self.command.clone())
            .output()
            .expect("failed to execute process")
        };
        if print {
            print!(
                "{}",
                String::from_utf8(
                    output.stdout
                )
                .unwrap()
            )
        }
    }
}
#[derive(Clone, Debug)]
pub(crate) struct Operation {
    pub(crate) name: String,
    pub(crate) commands: Vec<Command>
}
impl Operation {
    pub(crate) fn new() -> Operation {
        Operation {
            name: String::new(),
            commands: Vec::new()
        }
    }

    pub fn run(&self, print: bool) {
        for command in &self.commands {
            command.run(print);
        }
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct EnderPearl {
    print: bool,
    pub(crate) operations: Vec<Operation>
}
#[allow(dead_code)]
impl EnderPearl {
    pub fn new(print: bool) -> EnderPearl {
        EnderPearl {
            print,
            operations: Vec::new(),
        }
    }

    pub(crate) fn newop(&mut self, op: Operation) {
        self.operations.push(op);
    }

    pub fn run(&self, name: String){
        runcmd(name, &self,self.print);
    }
}





pub(crate) fn runcmd(cmd: String, tkn: &EnderPearl, print: bool){
    for op in &tkn.operations {
        if op.name.to_lowercase() == String::from("pre") {
            op.run(print);
        }
    };
    for op in &tkn.operations {
        if op.name.to_lowercase() == cmd {
            op.run(print);
        }
    };
    for op in &tkn.operations {
        if op.name.to_lowercase() == String::from("post") {
            op.run(print);
        }
    };
}
pub(crate) fn help(color: bool){
    if color {
        println!("\x1b[38;5;24mEnderPearl\x1b[0m:\n    '\x1b[38;5;34mbuild\x1b[0m' or '\x1b[38;5;34m.\x1b[0m' \x1b[38;5;208m->\x1b[0m runs the '\x1b[38;5;87mbuild\x1b[0m' command\n    '\x1b[38;5;34m*\x1b[0m' \x1b[38;5;208m->\x1b[0m runs the command with that name");
    } else {
        println!("EnderPearl:\n    'build' or '.' -> runs the 'build' command\n    '*' -> runs the command with that name");
    }
}
fn main() -> io::Result<()> {
    let mut print: bool = true;
    let mut color: bool = true;
    let mut contents = String::new();
    let mut contents2 = String::new();
    let mut file = match File::open(".enderpearl") {
        Ok(v) => v,
        Err(_) => {
            help(color);
            panic!();
        }
    };
    file.read_to_string(&mut contents)?;
    file = File::open("config.enderpearl")?;
    file.read_to_string(&mut contents2)?;
    let (_, tkn) = Token::tokenize(contents, contents2);
    let mut arg: String = match env::args().nth(1) {
        Some(data) => data,
        None => String::from("")
    }.to_lowercase();
    for args in env::args() {
        if args.contains("--quiet") {
            print = false;
        }
        if args.contains("--no-color") || args.contains("--nocolor") {
            color = false;
        }
    }
    if arg.starts_with("--") {
        arg = arg.replace("--","");
        if arg == "help" {
            help(color);
            return Ok(());
        } else {
            help(color);
            return Ok(());
        }
    } else {
        if arg == "build" || arg == "." {
            runcmd(String::from("build"),&tkn,print);
            return Ok(());
        } else if arg == "pre" || arg == "post" {
            println!("Sorry, you may not use this special operation");
            return Ok(());
        } else if arg == "" || arg == "help" {
            help(color);
            return Ok(());
        } else {
            runcmd(arg,&tkn,print);
            return Ok(());
        }
    }
}