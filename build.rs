// Specifies if Rinux should be built with enderpearl
//? Defaults to 'true'
//* Modify Allowed
const USE_ENDERPEARL: bool = true;


// Main build function
//* Modify Allowed
fn main() -> std::io::Result<()> {


    // Checks to see if build should use Enderpearl
    if USE_ENDERPEARL {


        // Function for all enderpearl
        enderpearl()?;
    };

    
    // Returns ok at the end
    Ok(())
}


// actaul function for dealing with enderpearl
//* MODIFY HIGHLY NOT RECOMMENDED
fn enderpearl() -> std::io::Result<()> {
    fn set_project_metadata() -> std::io::Result<()> {
        use std::{io::{Write,prelude::*},fs::File};
        #[derive(Clone, Debug)]
        pub(crate) struct ConfigKV {
            pub key: String,
            pub value: String
        }
        impl ConfigKV {
            pub fn new() -> ConfigKV {
                ConfigKV {
                    key: String::new(),
                    value: String::new()
                }
            }
            pub fn from(key: String, value: String) -> ConfigKV {
                ConfigKV { key, value }
            }
        }
        let mut file = File::open(".config.enderpearl")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
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
                // kv1 = ConfigKV::new();
                key1 = String::new();
                val1 = String::new();
            } else if dattype == 1 {
                key1.push(part);
            } else if dattype == 2 {
                val1.push(part);
            }
        }
        let mut res_config_name: String = String::new();
        let mut res_config_version: String = String::new();
        let mut res_config_quiet: bool = false;
        for kv in &keys_and_values {
            if kv.key.to_lowercase() == "name" {
                res_config_name = kv.value.clone();
            };
            if kv.key.to_lowercase() == "version" {
                res_config_version = kv.value.clone();
            };
            if kv.key.to_lowercase() == "quiet" {
                if kv.value == "true" {
                    res_config_quiet = true;
                } else if kv.value == "false" {
                    res_config_quiet = false;
                } else {
                    panic!("Invalid 'QUIET' value")
                }
            };
        };
        fn preset(name: String, version: String, quiet: bool) -> String {
            format!("#[allow(dead_code)] pub(crate) const PROJECT_NAME: &'static str = \"{}\";\n#[allow(dead_code)] pub(crate) const PROJECT_VERSION: &'static str = \"{}\";\n#[allow(dead_code)] pub(crate) const QUIET_BOOT: bool = {};",
                name,
                version,
                quiet
            )
        }
        let mut file = File::create("./rinux/src/conf/mod.rs")?;
        file.write_all(
            preset(
                res_config_name,
                res_config_version,
                res_config_quiet
            )
            .as_bytes()
        )?;
        Ok(())
    }
    fn enderpearl_build() -> std::io::Result<()> {
        #[derive(Clone, Debug)]
        pub(crate) struct Command {
            pub command: String
        }
        impl Command {
            pub fn new(command: String) -> Command {
                Command {
                    command
                }
            }
            pub fn run(&self) {
                let output = if cfg!(target_os = "windows") {
                    std::process::Command::new("cmd")
                    .args(["/C", self.command.as_str()])
                    .output()
                    .expect("failed to execute process")
                } else {
                    std::process::Command::new("sh")
                    .arg("-c")
                    .arg(self.command.clone())
                    .output()
                    .expect("failed to execute process")
                };
                print!(
                    "{}",
                    String::from_utf8(
                        output.stdout
                    )
                    .unwrap()
                )
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
            pub(crate) fn run(&self) {
                for command in &self.commands {
                    command.run();
                }
            }
        }
        use std::io::prelude::*;
        let mut file = std::fs::File::open(".build.enderpearl")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let mut operations: Vec<Operation> = Vec::new();
        let mut op: Operation = Operation::new();
        let mut opstr: String = String::new();
        let mut cmds: String = String::new();
        let mut txttype: u8 = 0;
        for part in contents.chars() {
            if part == '#' {
                op = Operation::new();
                txttype = 1;
            } else if part == '(' {
                txttype = 2;
            } else if part == ')' {
                txttype = 0;
                for part in opstr.clone().chars() {
                    if part == '\n' {
                        if !cmds.is_empty() {
                            op.commands.push(Command::new(cmds));
                        }
                        cmds = String::new();
                    } else {
                        cmds.push(part);
                    }
                }
                operations.push(op);
                op = Operation::new();
                opstr = String::new();
            } else if txttype == 1 && part != ' ' {
                op.name.push(part);
            } else if txttype == 2 && part != '\r'{
                opstr.push(part);
            }
        }
        for op in &operations {
            if op.name.to_lowercase() == "pre" {
                op.run();
            }
        };
        for op in &operations {
            if op.name.to_lowercase() == "stasis" {
                op.run();
            }
        };
        for op in &operations {
            if op.name.to_lowercase() == "post" {
                op.run();
            }
        };
        Ok(())
    }
    set_project_metadata()?;
    enderpearl_build()?;
    Ok(())
}