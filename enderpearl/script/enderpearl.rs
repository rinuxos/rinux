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

const DEFAULT_FILE_CONFIG: &str = r#"This file is used for configuring Rinux
NOT for building Rinux
Allowed properties are: 'NAME', 'QUIET', and 'VERSION', all other ones will be ignored

Name for the Project, string
#NAME(MyProject)

Project's version, string
#VERSION(v0.1.0)

If no debug info should be printed, will still print errors bool -> 'true' or 'false'
#QUIET(false)"#;

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

#[allow(unused_assignments)]
pub(crate) fn read_config(contents: String) -> Vec<ConfigKV> {
    let mut keys_and_values: Vec<ConfigKV> = Vec::new();
    let mut kv1: ConfigKV = ConfigKV::new();
    let mut key1: String = String::new();
    let mut val1: String = String::new();
    let mut dattype: u8 = 0;
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

    pub(crate) fn run(&self, print: bool, root: &'static str) {
        let output = if cfg!(target_os = "windows") {
            process::Command::new("cmd")
            .args(["/C", format!("cd ./{}",&root).as_str(), self.command.as_str()])
            .output()
            .expect("failed to execute process")
        } else {
            process::Command::new("sh")
            .arg("-c")
            .arg(format!("cd ./{}",&root))
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

    pub fn run(&self, print: bool,root: &'static str) {
        for command in &self.commands {
            command.run(print, root);
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

    pub fn run(&self, name: String) {
        runcmd(name, &self,self.print, "");
    }
}





pub(crate) fn runcmd(cmd: String, tkn: &EnderPearl, print: bool, root: &'static str) {
    for op in &tkn.operations {
        if op.name.to_lowercase() == String::from("pre") {
            op.run(print, &root);
        }
    };
    for op in &tkn.operations {
        if op.name.to_lowercase() == cmd {
            op.run(print, &root);
        }
    };
    for op in &tkn.operations {
        if op.name.to_lowercase() == String::from("post") {
            op.run(print, &root);
        }
    };
}
pub(crate) fn help(color: bool){
    if color {
        println!("\x1b[38;5;24mEnderPearl\x1b[0m:\n    '\x1b[38;5;34m--fix\x1b[0m' \x1b[38;5;208m->\x1b[0m attempts to fix build errors\n    '\x1b[38;5;34m--gen\x1b[0m' \x1b[38;5;208m->\x1b[0m generates/modifies files based on your configuration\n    '\x1b[38;5;34m--nocolor\x1b[0m' \x1b[38;5;208m->\x1b[0m prints without color\n    '\x1b[38;5;34mbuild\x1b[0m' or '\x1b[38;5;34m.\x1b[0m' \x1b[38;5;208m->\x1b[0m runs the '\x1b[38;5;87mbuild\x1b[0m' command\n    '\x1b[38;5;34m*\x1b[0m' \x1b[38;5;208m->\x1b[0m runs the command with that name");
    } else {
        println!("EnderPearl:\n    '--fix' -> attempts to fix build errors\n    '--gen' -> generates/modifies files based on your configuration\n    '--nocolor' -> prints without color\n    'build' or '.' -> runs the 'build' command\n    '*' -> runs the command with that name");
    }
}
pub(crate) fn gen<T: std::fmt::Display>(print: bool,color: bool,i1: T, i2: T, i3: T) {
    match File::create("./core/config/src/lib.rs") {
        Ok(mut f) => {
            match f.write_all(
                format!(
                    "#![no_std]\npub const PROJECT_NAME: &'static str = \"{}\";\npub const PROJECT_VERSION: &'static str = \"{}\";\npub const QUIET_BOOT: bool = {};",
                    i1,
                    i2,
                    i3
                ).as_bytes()
            ) {
                Ok(_) => {
                    if print {
                        if color {
                            println!("\x1b[38;5;24mEnderPearl\x1b[0m: \x1b[38;5;34mGeneration Successful\x1b[0m");
                        } else {
                            println!("EnderPearl: Generation Successful");
                        }
                    }
                },
                Err(_) => {
                    if color {
                        println!("\x1b[38;5;24mEnderPearl \x1b[38;5;196mError\x1b[0m: 'Unable to write file: ./core/config/src/lib.rs'");
                    } else {
                        println!("EnderPearl Error: 'Unable to write file: ./core/config/src/lib.rs'");
                    }
                }
            }
        },
        Err(_) => {
            if color {
                println!("\x1b[38;5;24mEnderPearl \x1b[38;5;196mError\x1b[0m: 'Unable to create file: ./core/config/src/lib.rs'");
            } else {
                println!("EnderPearl Error: 'Unable to create file: ./core/config/src/lib.rs'");
            }
        }
    }
}
fn main() -> io::Result<()> {
    let mut print: bool = true;
    let mut color: bool = true;
    let mut contents = String::new();
    let mut file = match File::open(".enderpearl") {
        Ok(v) => v,
        Err(_) => {
            help(color);
            panic!();
        }
    };
    file.read_to_string(&mut contents)?;
    let contents2 = match File::open("./enderpearl/config.enderpearl"){
        Ok(mut v) => {
            let mut res = String::new();
            v.read_to_string(&mut res)?;
            res
        },
        Err(_) => {
            File::create("./enderpearl/config.enderpearl")?;
            file.write_all(DEFAULT_FILE_CONFIG.as_ref())?;
            String::from(DEFAULT_FILE_CONFIG)
        }
    };
    let (_, tkn) = Token::tokenize(contents, contents2);
    for arg in env::args() {
        if arg.starts_with("--") {
            if arg.contains("--quiet") {
                print = false;
            }
            if arg.contains("--no-color") || arg.contains("--nocolor") || arg.contains("--no-colour") || arg.contains("--nocolour") || arg.contains("-n") {
                color = false;
            }
            if arg.contains("--help") || arg.contains("-h") {
                help(color);
            }
            if arg.contains("--fix") {
                gen(false, color,"MyProject","v0.1.0","false");
            }
            if arg.contains("--stasis"){
                let build = match File::open("./enderpearl/build.enderpearl") {
                    Ok(mut f) => {
                        let mut contents = String::new();
                        f.read_to_string(&mut contents).unwrap();
                        let stasises = Token::tokenize(contents, String::new()).1;
                        for op in &stasises.operations {
                            if op.name.to_lowercase() == String::from("pre") {
                                op.run(print,"");
                            } else if op.name.to_lowercase() == String::from("post") {
                                op.run(print,"");
                            } else if op.name.to_lowercase() == String::from("stasis") {
                                op.run(print,"");
                            }
                        };
                    },
                    Err(_) => {
                        if color {
                            println!("\x1b[38;5;24mEnderPearl \x1b[38;5;196mError\x1b[0m: 'Unable to open file: ./enderpearl/build.enderpearl'");
                        } else {
                            println!("EnderPearl Error: 'Unable to open file: ./enderpearl/build.enderpearl'");
                        }
                        panic!();
                    }
                };
            }
            if arg.contains("--gen") {
                let config = match File::open("./enderpearl/config.enderpearl") {
                    Ok(mut f) => {
                        let mut contents = String::new();
                        f.read_to_string(&mut contents).unwrap();
                        read_config(contents)
                    },
                    Err(_) => {
                        if color {
                            println!("\x1b[38;5;24mEnderPearl \x1b[38;5;196mError\x1b[0m: 'Unable to open file: ./enderpearl/config.enderpearl'");
                        } else {
                            println!("EnderPearl Error: 'Unable to open file: ./enderpearl/config.enderpearl'");
                        }
                        panic!();
                    }
                };
                let mut name: String = String::new();
                let mut version: String = String::new();
                let mut boot: String = String::new();
                for cfg in config {
                    if cfg.key.to_lowercase() == "name" {
                        name = cfg.value;
                    } else if cfg.key.to_lowercase() == "version" {
                        version = cfg.value;
                    } else if cfg.key.to_lowercase() == "quiet" {
                        if cfg.value.to_lowercase() == "true" {
                            boot = String::from("true");
                        } else {
                            boot = String::from("false");
                        }
                    }
                }
                gen(print, color, name, version, boot);
            }
        } else {
            if arg == "build" || arg == "." {
                runcmd(String::from("build"),&tkn,print,"");
            } else if arg == "pre" || arg == "post" {
                println!("Sorry, you may not use this special operation");
            } else if arg == "" || arg == "help" {
                help(color);
            } else {
                runcmd(arg,&tkn,print,"");
            }
        }
    };
    Ok(())
}
