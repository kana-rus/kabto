use crate::{Result, error::Error};

pub(super) enum Value {
    Str(String),
    // ...
}

impl Value {
    pub(super) fn parse(string: &str) -> Result<Self> {
        if string.starts_with('"') {
            let end_pos = 1 + (&string[1..])
                .find('"')
                .ok_or_else(|| Error::InvalidTomlSyntax(string.to_owned()))?;
            return Ok(Self::Str((&string[1..end_pos]).to_owned()))
        }

        // ...

        Err(Error::InvalidTomlSyntax(string.to_owned()))
    }
}
