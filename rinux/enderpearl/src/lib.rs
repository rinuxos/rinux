#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![allow(dead_code)]


extern crate alloc;
use alloc::{
    string::String,
    vec::Vec
};
mod runner;








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
        let mut vec = Vec::new();
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

#[allow(non_snake_case)]
#[allow(unused_assignments)]
pub unsafe fn _customTokenize(contents: String) -> Vec<(String, String)> {
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
        pub fn to_set(key: String, value: String) -> (String, String) {
            ( key, value )
        }
    }
    let mut keys_and_values: Vec<ConfigKV> = Vec::new();
    let mut result: Vec<(String, String)> = Vec::new();
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
    for kv in keys_and_values {
        result.push(ConfigKV::to_set(kv.key, kv.value))
    };
    result
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
pub struct Command {
    #[allow(dead_code)]
    pub command: String
}
impl Command {
    pub(crate) fn new(command: String) -> Command {
        Command {
            command
        }
    }

    pub(crate) fn run(&self, print: bool) {
        runner::run(&self.command, print);
    }

    #[allow(non_snake_case)]
    pub unsafe fn _useCustomRunner<F: FnMut(String, bool) -> () >(command: &Command, print: bool, mut f: F) {
        f(String::from(&command.command), print)
    }
}

#[derive(Clone, Debug)]
pub struct Operation {
    pub name: String,
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

    #[allow(non_snake_case)]
    pub unsafe fn _useCustomRunner<F: FnMut(&Vec<Command>, bool) -> () >(operation: &Operation, print: bool, mut f: F) {
        f(&operation.commands, print)
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

    pub fn parse(&mut self, main_file: String, config_file: String ) -> EnderPearl {
        let (_, dat) = Token::tokenize(main_file, config_file);
        dat
    }

    pub fn run(&self, name: String){
        runcmd(name, &self,self.print);
    }

    #[allow(non_snake_case)]
    pub unsafe fn _useCustomRunner<F: FnMut(&Vec<Operation>, bool) -> () >(pearl: &EnderPearl, mut f: F) {
        f(&pearl.operations, pearl.print)
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