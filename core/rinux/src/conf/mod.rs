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

mod file;

#[unstable(feature = "rinuxcore_custom_config", issue = "none")]
pub enum ConfigType {
    File,
    UserDefined(Config),
}

#[unstable(feature = "rinuxcore_custom_config", issue = "none")]
#[derive(Debug, Default, Clone, Copy)]
pub struct Config {
    pub project_name: &'static str,
    pub project_version: &'static str,
    pub quiet_boot: bool,
}

#[unstable(feature = "rinuxcore_custom_config", issue = "none")]
impl Config {
    // #[rustc_const_stable(feature = "rinuxcore", since = "0.1.23")]
    pub const fn cnst() -> Self {
        Config {
            project_name: "",
            project_version: "",
            quiet_boot: false,
        }
    }

    // #[stable(feature = "rinuxcore", since = "0.1.23")]
    pub const fn new(
        project_name: &'static str,
        project_version: &'static str,
        quiet_boot: bool,
    ) -> Self {
        Config {
            project_name,
            project_version,
            quiet_boot,
        }
    }

    #[unstable(feature = "rinuxcore_custom_config", issue = "none")]
    pub(crate) fn get_config(self, config_type: ConfigType) -> Self {
        match config_type {
            ConfigType::File => { unsafe {
                let res = Config {
                    project_name: file::PROJECT_NAME,
                    project_version: file::PROJECT_VERSION,
                    quiet_boot: file::QUIET_BOOT,
                };
                res
            }},
            ConfigType::UserDefined(data) => data
        }
    }
}
