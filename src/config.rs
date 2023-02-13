mod toml;
mod table;
mod value;
mod map;

use std::{fs::File, io::Read, str::Lines};
use crate::{Result, error::Error, config::{toml::{Toml, FromToml}, table::Table, value::Value}};

const CONFIG_FILE_NAME: &'static str = "kab.toml";

struct Config<'lines> {
    build: BuildConfig<'lines>,
}
struct BuildConfig<'lines> {
    out_dir: &'lines str,
}

const _: (/* Config impls */) = {
    impl<'lines> Config<'lines> {
        pub fn get() -> Result<Self> {
            let config_file = File::open(CONFIG_FILE_NAME)?;
            let config = Self::parse(config_file)?;
            Ok(config)
        }
    }

    impl<'lines> Default for Config<'lines> {
        fn default() -> Self {
            Self {
                build: BuildConfig {
                    out_dir: "out",
                },
            }
        }
    }

    impl<'lines> FromToml<'lines> for Config<'lines> {
        fn from_toml(toml: Toml<'lines>) -> Result<Self> {
            let mut config = Config::default();

            for (table_name, table) in toml {
                match table_name {
                    "build" => for (key, value) in table {
                        match key {
                            "out_dir" => config.build.out_dir = Value::parse(string)?.
                        }
                    },
                    _ => return Err(Error::UnknownConfigTable(table_name.to_owned()))
                }
            }

            Ok(config)
        }
    }

    impl<'lines> Config<'lines> {
        fn parse(file: File) -> Result<Self> {
            let mut config = Config::default();

            let mut config_file_lines = {
                let mut config_buffer = String::new();
                file.read_to_string(&mut config_buffer);
                config_buffer
            }.lines().peekable();

            let toml = Toml::parse(&mut config_file_lines)?;



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
