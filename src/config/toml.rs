use std::{iter::Peekable, str::Lines};
use crate::Result;
use super::{table::Table, map::Map};

pub(crate) struct Toml<'lines>(
    Map<'lines, Table<'lines>>
);

pub(crate) trait FromToml<'lines>: Sized {
    fn from_toml(toml: Toml<'lines>) -> Result<Self>;
}

const _: (/* Toml impls */) = {
    impl<'lines> Toml<'lines> {
        pub fn parse(lines: &mut Peekable<Lines>) -> Result<Self> {
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
    
    impl<'lines> IntoIterator for Toml<'lines> {
        type Item = <Map<'lines, Table<'lines>> as IntoIterator>::Item;
        type IntoIter = <Map<'lines, Table<'lines>> as IntoIterator>::IntoIter;
        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }
};
