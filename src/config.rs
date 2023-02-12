mod toml;
mod table;
mod value;

use std::{fs::File, io::Read, str::Lines};
use crate::{Result, error::Error};

const CONFIG_FILE_NAME: &'static str = "kab.toml";

struct Config {
    build: BuildConfig,
}
struct BuildConfig {
    out_dir: String,
}

const _: (/* Config impls */) = {
    impl Config {
        pub fn get() -> Result<Self> {
            let config_file = File::open(CONFIG_FILE_NAME)?;
            let config = Self::parse(config_file)?;
            Ok(config)
        }
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                build: BuildConfig {
                    out_dir: String::from("out"),
                },
            }
        }
    }

    impl Config {
        fn parse(file: File) -> Result<Self> {
            let mut config = Config::default();

            let mut config_file_lines = {
                let mut config_buffer = String::new();
                file.read_to_string(&mut config_buffer);
                config_buffer
            }.lines();

            while let Some(line) = config_file_lines.next() {
                match line {
                    "[build]" => config.build = BuildConfig::parse(&mut config_file_lines)?,
                    _ => return Err(Error::UnknownConfigTable(line.to_owned())),
                }
            }

            Ok(config)
        }

    }
};

const _: (/* BuildConfig impls */) = {
    impl Default for BuildConfig {
        fn default() -> Self {
            Self {
                out_dir: String::from("out"),
            }
        }
    }

    impl BuildConfig {
        fn parse(lines: &mut Lines) -> Result<Self> {

        }
    }
};
