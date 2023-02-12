use std::{str::Lines, iter::Peekable, collections::HashSet};
use crate::{Result, error::Error};
use super::value::Value;

pub(super) struct Table {
    pub(super) name: String,
    pub(super) key_values: Vec<(String, Value)>,
}

impl Table {
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
                .to_owned()
        };

        let key_values = {
            let mut key_values = Vec::new();

            while let Some(next_line) = lines.peek() {
                if next_line.is_empty()
                || next_line.starts_with('[') {
                    break
                }

                let line = lines.next().unwrap();
                let (key, value) = line.split_once('=')
                    .map(|(k, v)| (k.trim(), v.trim()))
                    .ok_or_else(|| Error::InvalidTomlSyntax(format!("`=` is not found in key-value line")))?;

                key_values.push((
                    key.to_owned(),
                    Value::parse(value)?
                ))
            }

            key_values
        };

        Self::validated(name, key_values)
    } 

    fn validated(name: String, key_values: Vec<(String, Value)>) -> Result<Self> {
        // check key duplication
        let mut keys = HashSet::new();
        for key_value in &key_values {
            if keys.insert(&key_value.0) == false /* "it's already set" */ {
                return Err(Error::InvalidTomlSyntax(format!(
                    "table `{}` has duplicated key: `{}`",
                    &name,
                    key_value.0
                )))
            }
        }

        Ok(Self {name, key_values})
    }
}
