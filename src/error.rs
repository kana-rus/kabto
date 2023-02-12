pub enum Error {
    FailedToWriteFile(String),
    InvalidTomlSyntax(String),
    UnknownConfigTable(String),
    UnknownConfigKey {config: String, key: String},
}

const _: (/* Error impls */) = {
    impl From<std::io::Error> for Error {
        fn from(value: std::io::Error) -> Self {
            Self::FailedToWriteFile(value.to_string())
        }
    }
};
