/// Parse error type for command line arguments.
///
/// ParseError is an enum that represents the different types of errors that can occur when parsing a command.
///
/// # Examples
/// ```
/// use cli_todo::arg_parser::ParseError;
/// use std::env;
///
/// let args: Vec<String> = env::args().collect();
/// let operation = parse_operation(&args).unwrap();
/// ```
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ParseError {
    /// Insufficient arguments.
    InsufficientArguments,
    /// Invalid argument.
    InvalidArgument,
    /// Unknown operation.
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
