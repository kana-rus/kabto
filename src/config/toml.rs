use std::{iter::Peekable, str::Lines, collections::HashSet};
use crate::{Result, error::Error};
use super::table::Table;

pub(crate) struct Toml(
    Vec<Table>
);

impl Toml {
    pub fn parse(lines: &mut Peekable<Lines>) -> Result<Self> {
        let tables = {
            let mut tables = Vec::new();

            while let Some(line) = lines.next() {
                if line.is_empty() {
                    break
                }

                tables.push(Table::parse(lines)?)
            }

            tables
        };

        Self::validated(tables)
    }

    fn validated(tables: Vec<Table>) -> Result<Self> {
        // check table_name duplication
        let mut names = HashSet::new();
        for table in &tables {
            if names.insert(&table.name) == false /* "it's already set" */ {
                return Err(Error::InvalidTomlSyntax(format!(
                    "tables `{}` is defined more than once",
                    &table.name
                )))
            }
        }

        Ok(Self(tables))
    }
}

