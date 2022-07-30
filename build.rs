use std::{
    io::prelude::*,
    process,
    fs::File
};


const VERSION: &'static str = "v0.1.0-BETA";
const USE_DEBUG_PRINT: bool = true;








#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum TextKind {
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
    TEXT_COMMAND_NAME,
    TEXT_COMMAND
}
impl Token {
    pub fn tokenize(string: String) -> ( Vec<Token>, EnderPearl, usize) {
        let mut vec = vec![];
    
        let mut efile: EnderPearl = EnderPearl {
            print: true,
            operations: Vec::new()
        };
        let mut op: Operation = Operation::new();
        let mut opstr: String = String::new();
        let mut cmds: String = String::new();
    
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
            } else if txttype == TextKind::CommandName && part != ' ' {
                op.name.push(part);
                vec.push(Token::TEXT_COMMAND_NAME);
            } else if txttype == TextKind::Command && part != '\r'{
                opstr.push(part);
                vec.push(Token::TEXT_COMMAND);
            }
        }

        let len: usize = vec.len();
    
        return (
            vec,
            efile,
            len
        );
    }
}



#[derive(Clone, Debug)]
pub(crate) struct Command {
    #[allow(dead_code)]
    pub command: String
}
impl Command {
    pub fn new(command: String) -> Command {
        Command {
            command
        }
    }

    pub fn run(&self, print: bool) {
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
    pub name: String,
    pub commands: Vec<Command>
}
impl Operation {
    pub fn new() -> Operation {
        Operation {
            name: String::new(),
            commands: Vec::new()
        }
    }

    pub(crate) fn run(&self, print: bool) {
        for command in &self.commands {
            command.run(print);
        }
    }
}
#[derive(Clone, Debug)]
pub struct EnderPearl {
    print: bool,
    pub(crate) operations: Vec<Operation>
}
impl EnderPearl {
    pub fn new(print: bool) -> std::io::Result<EnderPearl> {
        let mut file = File::open(".enderpearl")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let (_, tkn, _) = Token::tokenize(contents);
        Ok(
            EnderPearl {
                print,
                operations: tkn.operations,
            }
        )
    }
    pub(crate) fn newop(&mut self, op: Operation) {
        self.operations.push(op);
    }
    pub fn run(&self, name: String){
        for op in &self.operations {
            if op.name.to_lowercase() == String::from("pre") {
                op.run(self.print);
            }
        };
        for op in &self.operations {
            if op.name.to_lowercase() == name.to_lowercase() {
                op.run(self.print);
            }
        };
        for op in &self.operations {
            if op.name.to_lowercase() == String::from("post") {
                op.run(self.print);
            }
        };
    }
}


fn main() -> std::io::Result<()> {
    let mut print: bool = true;
    let mut file = match File::open(".enderpearl") {
        Ok(file) => {
            println!("Found Build File...\nExecuting 'STASIS' Command");
            file
        },
        Err(_) => {
            panic!("No Build File Found")
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let (_, tkn, _) = Token::tokenize(contents);
    tkn.run("STASIS".to_string());
    Ok(())
}