#[derive(Debug)]
#[derive(PartialEq)]
pub enum ParseError {
    InsufficientArguments,
    InvalidArgument,
    UnknownOperation,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InsufficientArguments => writeln!(f, "Error: Insufficient arguments."),
            ParseError::InvalidArgument => writeln!(f, "Error: Invalid argument."),
            ParseError::UnknownOperation => writeln!(f, "Error: Unsupported operation."),
        }
    }
}
