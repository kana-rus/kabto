use std::fmt::Display;
use crate::{Result, Error};

pub(crate) enum Value<'fc> {
    Str(&'fc str),
    // ...
}

const _: (/* Value impls */) = { 
    impl<'fc> Value<'fc> {
        pub(crate) fn parse(string: &'fc str) -> Result<Self> {
            if string.starts_with('"') {
                let end_pos = 1 + (&string[1..])
                    .find('"')
                    .ok_or_else(|| Error::InvalidTomlSyntax(string.to_owned()))?;
                return Ok(Self::Str(&string[1..end_pos]))
            }

            // ...

            Err(Error::InvalidTomlSyntax(string.to_owned()))
        }

        pub(crate) fn as_str(self) -> Result<&'fc str> {
            match self {
                Self::Str(str) => Ok(str),
                // _ => Err(Error::InvalidTomlSyntax(format!(
                //     "Expected string value, but found `{self}`"
                // ))),
            }
        }
    }

    impl<'fc> Display for Value<'fc> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", match self {
                Self::Str(str) => format!("{str}: string"),
                // ...
            })
        }
    }
};
