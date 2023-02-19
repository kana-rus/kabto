use std::{collections::BTreeMap};
use crate::{Result, Error};

pub(crate) struct Map<'lines, V>(
    BTreeMap<&'lines str, V>
);

const _: (/* Map impls */) = {
    impl<'lines, V> Map<'lines, V> {
        pub fn new() -> Self {
            Self(BTreeMap::new())
        }
        pub fn insert(&mut self, key: &'lines str, value: V) -> Result<()> {
            self.0.insert(key, value)
                .ok_or_else(|| Error::InvalidTomlSyntax(format!(
                    "key `{key}` is duplicated"
                )))
                .map(|_| ())
        }
    }

    impl<'lines, V> IntoIterator for Map<'lines, V> {
        type Item = <BTreeMap<&'lines str, V> as IntoIterator>::Item;
        type IntoIter = <BTreeMap<&'lines str, V> as IntoIterator>::IntoIter;
        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }
};