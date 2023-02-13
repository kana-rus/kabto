use std::{str::Lines, iter::Peekable};
use crate::{Result, error::Error};
use super::{value::Value, map::Map};

pub(super) struct Table<'lines> {
    pub(super) name: &'lines str,
    pub(super) key_values: Map<'lines, Value<'lines>>,
}

const _: (/* Table impls */) = {
    impl<'lines> Table<'lines> {
        pub(crate) fn parse(lines: &mut Peekable<Lines>) -> Result<Self> {
            let name = {
                let name_line = lines.next()
                    .ok_or_else(|| Error::InvalidTomlSyntax(String::from("table name is not found")))?;

                name_line.trim()
                    .strip_prefix('[')
                    .ok_or_else(|| Error::InvalidTomlSyntax(format!("invalid table name: `{name_line}`")))?
                    .strip_suffix(']')
                    .ok_or_else(|| Error::InvalidTomlSyntax(format!("invalid table name: `{name_line}`")))?
                    .trim()
            };

            let key_values = {
                let mut key_values = Map::new();

                while let Some(next_line) = lines.peek() {
                    if next_line.is_empty()
                    || next_line.starts_with('[') {
                        break
                    }

                    let line = lines.next().unwrap();
                    let (key, value) = line.split_once('=')
                        .map(|(k, v)| (k.trim(), v.trim()))
                        .ok_or_else(|| Error::InvalidTomlSyntax(format!("`=` is not found in key-value line")))?;

                    key_values.insert(key, Value::parse(value)?)?
                }

                key_values
            };

            Ok(Self {name, key_values})
        }
    }

    impl<'lines> IntoIterator for Table<'lines> {
        type Item = <Map<'lines, Value<'lines>> as IntoIterator>::Item;
        type IntoIter = <Map<'lines, Value<'lines>> as IntoIterator>::IntoIter;
        fn into_iter(self) -> Self::IntoIter {
            self.key_values.into_iter()
        }
    }
};
