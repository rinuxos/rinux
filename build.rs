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
    fs::File,
    io::{prelude::*, Write},
};

const DEFAULT_FILE_CONFIG: &str = r#"This file is used for configuring Rinux
NOT for building Rinux
Allowed properties are: 'NAME', 'QUIET', and 'VERSION', all other ones will be ignored

Name for the Project, string
#NAME(MyProject)

Project's version, string
#VERSION(v0.1.0)

If no debug info should be printed, will still print errors bool -> 'true' or 'false'
#QUIET(false)"#;
const DEFAULT_FILE_BUILD: &str = r#"This file is used for importing your own projects into Rinux
NOT for building Rinux
Allowed commands are: 'STASIS', 'PRE', and 'POST', all other ones will be ignored

#STASIS(
echo Nothing in 'STASIS'
echo Running PureOS
)"#;

fn main() -> std::io::Result<()> {
    fn set_project_metadata() -> std::io::Result<()> {
        let contents: String = match File::open("config.enderpearl") {
            Ok(mut v) => {
                let mut res = String::new();
                v.read_to_string(&mut res)?;
                res
            }
            Err(_) => {
                println!("couldn't find 'config.enderpearl' in project root");
                let mut newfile = File::create("config.enderpearl")?;
                newfile.write_all(DEFAULT_FILE_CONFIG.as_bytes())?;
                String::from(DEFAULT_FILE_BUILD)
            }
        };
        let keys_and_values = unsafe { epearl::_customTokenize(contents) };
        let mut res_config_name: String = String::new();
        let mut res_config_version: String = String::new();
        let mut res_config_quiet: bool = false;
        for kv in keys_and_values {
            if kv.0.to_lowercase() == "name" {
                res_config_name = kv.1.clone();
            };
            if kv.0.to_lowercase() == "version" {
                res_config_version = kv.1.clone();
            };
            if kv.0.to_lowercase() == "quiet" {
                if kv.1 == "true" {
                    res_config_quiet = true;
                } else if kv.1 == "false" {
                    res_config_quiet = false;
                } else {
                    panic!("Invalid 'QUIET' value")
                }
            };
        }
        fn preset(name: String, version: String, quiet: bool) -> String {
            format!(
                r#"#![allow(dead_code)]
pub(crate) const PROJECT_NAME: &'static str = "{}";
pub(crate) const PROJECT_VERSION: &'static str = "{}";
pub(crate) const QUIET_BOOT: bool = {};"#,
                name, version, quiet
            )
        }
        let _res = match File::open("./core/rinux/src/conf/file.rs") {
            Ok(mut file) => file
                .write_all(preset(res_config_name, res_config_version, res_config_quiet).as_bytes())
                .is_ok(),
            Err(_) => match File::create("./core/rinux/src/conf/file.rs") {
                Ok(mut file) => file
                    .write_all(
                        preset(res_config_name, res_config_version, res_config_quiet).as_bytes(),
                    )
                    .is_ok(),
                Err(_) => false,
            },
        };
        Ok(())
        // let mut file = File::create("./src/conf/file.rs")?;
    }
    fn enderpearl_build() -> std::io::Result<()> {
        let config_file: String = match File::open("config.enderpearl") {
            Ok(mut v) => {
                let mut res = String::new();
                v.read_to_string(&mut res)?;
                res
            }
            Err(_) => {
                println!("couldn't find 'config.enderpearl' in project root");
                let mut newfile = File::create("config.enderpearl")?;
                newfile.write_all(DEFAULT_FILE_CONFIG.as_bytes())?;
                String::from(DEFAULT_FILE_CONFIG)
            }
        };
        let main_file: String = match File::open("build.enderpearl") {
            Ok(mut v) => {
                let mut res = String::new();
                v.read_to_string(&mut res)?;
                res
            }
            Err(_) => {
                println!("couldn't find 'build.enderpearl' in project root");
                let mut newfile = File::create("build.enderpearl")?;
                newfile.write_all(DEFAULT_FILE_BUILD.as_bytes())?;
                String::from(DEFAULT_FILE_BUILD)
            }
        };
        let pearl = epearl::EnderPearl::new(true).parse(main_file, config_file);
        unsafe {
            epearl::EnderPearl::_useCustomRunner(&pearl, |operations, print| {
                for operation in operations {
                    if operation.name.to_lowercase() == "pre" {
                        epearl::Operation::_useCustomRunner(operation, print, |commands, print| {
                            for command in commands {
                                epearl::Command::_useCustomRunner(command, print, |cmd, print| {
                                    run(cmd, print)
                                });
                            }
                        });
                    }
                    if operation.name.to_lowercase() == "stasis" {
                        epearl::Operation::_useCustomRunner(operation, print, |commands, print| {
                            for command in commands {
                                epearl::Command::_useCustomRunner(command, print, |cmd, print| {
                                    run(cmd, print)
                                });
                            }
                        });
                    }
                    if operation.name.to_lowercase() == "post" {
                        epearl::Operation::_useCustomRunner(operation, print, |commands, print| {
                            for command in commands {
                                epearl::Command::_useCustomRunner(command, print, |cmd, print| {
                                    run(cmd, print)
                                });
                            }
                        });
                    }
                }
            });
        };
        fn run(cmd: String, print: bool) {
            let output = if cfg!(target_os = "windows") {
                std::process::Command::new("cmd")
                    .args(["/C", cmd.as_str()])
                    .output()
                    .expect("failed to execute process")
            } else {
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .output()
                    .expect("failed to execute process")
            };
            if print {
                std::io::stdout().write_all(&output.stdout).unwrap();
            }
        }
        Ok(())
    }
    set_project_metadata()?;
    enderpearl_build()?;
    Ok(())
}
