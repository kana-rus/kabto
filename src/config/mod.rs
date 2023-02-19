mod toml;
mod table;
mod value;
mod map;

use std::{fs::File, io::Read, iter::Peekable, str::Lines};
use once_cell::sync::Lazy;
use crate::{Result, Error, config::toml::{Toml, FromToml}};

pub(crate) const CONFIG_FILE_NAME: &'static str = "kab.toml";
static CONFIG_FILE_CONTENT: Lazy<Result<String>> = Lazy::new(|| {
    let mut config_file = File::open(CONFIG_FILE_NAME)?;
    let mut file_content = String::new();
    config_file.read_to_string(&mut file_content)?;
    Ok(file_content)
});

pub(crate) struct Config<'fc> {
    pub(crate) build: BuildConfig<'fc>,
}
pub(crate) struct BuildConfig<'fc> {
    pub(crate) out_dir: &'fc str,
}
type FileContent<'fc> = Peekable<Lines<'fc>>;

const _: (/* Config impls */) = {
    impl<'fc> Config<'fc> {
        pub fn get() -> Result<Self> {
            let file_content = CONFIG_FILE_CONTENT.as_ref()?;
            let mut file_content = file_content.lines().peekable();
            let toml = Toml::parse(&mut file_content)?;
            Self::from_toml(toml)
        }
    }

    impl<'fc> Default for Config<'fc> {
        fn default() -> Self {
            Self {
                build: BuildConfig {
                    out_dir: "out",
                },
            }
        }
    }

    impl<'fc> FromToml<'fc> for Config<'fc> {
        fn from_toml(toml: Toml<'fc>) -> Result<Self> {
            let mut config = Config::default();

            for (table_name, table) in toml {
                match table_name {
                    "build" => for (key, value) in table {
                        match key {
                            "out_dir" => config.build.out_dir = value.as_str()?,
                            _ => return Err(Error::UnknownConfigKey {
                                table: table_name.to_owned(),
                                key:   key.to_owned(),
                            })
                        }
                    },
                    _ => return Err(Error::UnknownConfigTable(table_name.to_owned()))
                }
            }

            Ok(config)
        }
    }
};
