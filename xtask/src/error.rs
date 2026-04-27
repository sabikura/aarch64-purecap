#[derive(Debug)]
pub enum XtaskError {
    MissingSubcommand,
    InvalidSubcommand,
    IoError,
    CommandFailed,
    MissingExampleName,
    MissingPackageName,
}

impl From<std::io::Error> for XtaskError {
    fn from(_value: std::io::Error) -> Self {
        XtaskError::IoError
    }
}
