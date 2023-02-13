use std::fmt::Display;

use crate::{Result, error::Error};

pub(super) enum Value<'lines> {
    Str(&'lines str),
    // ...
}

const _: (/* Value impls */) = { 
    impl<'lines> Value<'lines> {
        pub(super) fn parse(string: &str) -> Result<Self> {
            if string.starts_with('"') {
                let end_pos = 1 + (&string[1..])
                    .find('"')
                    .ok_or_else(|| Error::InvalidTomlSyntax(string.to_owned()))?;
                return Ok(Self::Str(&string[1..end_pos]))
            }

            // ...

            Err(Error::InvalidTomlSyntax(string.to_owned()))
        }

        pub(crate) fn as_str(self) -> Result<&'lines str> {
            match self {
                Self::Str(str) => Ok(str),
                _ => Err(Error::InvalidTomlSyntax(format!(
                    "Expected string value, but found `{self}`"
                ))),
            }
        }
    }

    impl<'lines> Display for Value<'lines> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", match self {
                Self::Str(str) => format!("{str}: string"),
                // ...
            })
        }
    }
};
