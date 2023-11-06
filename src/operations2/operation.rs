use crate::operations::OperationError;

pub trait Operation {
    fn execute(&self, lines: Vec<String>) -> Result<Vec<String>, OperationError>;
    fn description(&self) -> String;
}

