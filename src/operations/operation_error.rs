use std::io;

/// Error type for operations.
///
/// This enum represents the different errors that can occur when performing an operation.
///
/// # Variants
/// * `OutOfRange` - The given index is out of range.
/// * `IOError` - An IO error occurred.
///
/// # Examples
/// ```
/// use cli_todo::operations::OperationError;
/// use std::io;
///
/// let err = OperationError::IOError(io::Error::new(io::ErrorKind::Other, "foo"));
/// ```
#[derive(Debug)]
pub enum OperationError {
    OutOfRange,
    IOError(io::Error),
}

impl From<io::Error> for OperationError {
    fn from(err: io::Error) -> Self {
        OperationError::IOError(err)
    }
}

impl PartialEq for OperationError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OperationError::OutOfRange, OperationError::OutOfRange) => true,
            (OperationError::IOError(err1), OperationError::IOError(err2)) => {
                return err1.kind() == err2.kind()
            }
            _ => false,
        }
    }
}

impl std::fmt::Display for OperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationError::OutOfRange => write!(f, "Index out of range"),
            OperationError::IOError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for OperationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_out_of_range() {
        assert_eq!(OperationError::OutOfRange, OperationError::OutOfRange);
    }

    #[test]
    fn eq_io_error() {
        let err1 = OperationError::IOError(io::Error::new(io::ErrorKind::Other, "foo"));
        let err2 = OperationError::IOError(io::Error::new(io::ErrorKind::Other, "bar"));
        assert_eq!(err1, err2);
    }

    #[test]
    fn neq_io_error() {
        let err1 = OperationError::IOError(io::Error::new(io::ErrorKind::Other, "foo"));
        let err2 = OperationError::IOError(io::Error::new(io::ErrorKind::NotFound, "foo"));
        assert_ne!(err1, err2);
    }

    #[test]
    fn neq_out_of_range_io_error() {
        let err1 = OperationError::IOError(io::Error::new(io::ErrorKind::Other, "foo"));
        let err2 = OperationError::OutOfRange;
        assert_ne!(err1, err2);
    }
    
    #[test]
    fn display_out_of_range() {
        assert_eq!(format!("{}", OperationError::OutOfRange), "Index out of range");
    }

    #[test]
    fn display_io_error() {
        let err = OperationError::IOError(io::Error::new(io::ErrorKind::Other, "foo"));
        assert_eq!(format!("{}", err), "IO error: foo");
    }
}
