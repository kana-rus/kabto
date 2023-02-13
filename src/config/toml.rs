use crate::{Result, config::FileContent};
use super::{table::Table, map::Map};

pub(crate) struct Toml<'fc>(
    Map<'fc, Table<'fc>>
);

pub(crate) trait FromToml<'fc>: Sized {
    fn from_toml(toml: Toml<'fc>) -> Result<Self>;
}

const _: (/* Toml impls */) = {
    impl<'fc> Toml<'fc> {
        pub fn parse(lines: &mut FileContent<'fc>) -> Result<Self> {
            let tables = {
                let mut tables = Map::new();

                while let Some(line) = lines.next() {
                    if line.is_empty() {
                        break
                    }

                    let table = Table::parse(lines)?;
                    tables.insert(table.name, table)?
                }

                tables
            };

            Ok(Self(tables))
        }
    }
    
    impl<'fc> IntoIterator for Toml<'fc> {
        type Item = <Map<'fc, Table<'fc>> as IntoIterator>::Item;
        type IntoIter = <Map<'fc, Table<'fc>> as IntoIterator>::IntoIter;
        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }
};
