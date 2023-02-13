pub enum Error {
    FailedToIO(String),
    InvalidTomlSyntax(String),
    UnknownConfigTable(String),
    UnknownConfigKey {table: String, key: String},
}

const _: (/* Error impls */) = {
    impl From<&Error> for Error {
        fn from(value: &Error) -> Self {
            match value {
                Self::FailedToIO(msg) => Self::FailedToIO(msg.clone()),
                Self::InvalidTomlSyntax(msg) => Self::InvalidTomlSyntax(msg.clone()),
                Self::UnknownConfigTable(msg) => Self::UnknownConfigTable(msg.clone()),
                Self::UnknownConfigKey { table, key } => Self::UnknownConfigKey { table: table.clone(), key: key.clone() }
            }
        }
    }

    impl From<std::io::Error> for Error {
        fn from(value: std::io::Error) -> Self {
            Self::FailedToIO(value.to_string())
        }
    }
};
