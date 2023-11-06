use crate::{operations::OperationError, operations2::Operation};

pub struct ClearOperation {
    name: String,
    description: String,
}

impl ClearOperation {
    pub fn new() -> Self {
        ClearOperation {
            name: "clear".to_string(),
            description: "Clear the list".to_string(),
        }
    }
}

impl Operation for ClearOperation {
    fn execute(&self, _: Vec<String>) -> Result<Vec<String>, OperationError> {
        Ok(vec![])
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::operations2::{clear_operation::ClearOperation, operation::Operation};

    #[test]
    fn new() {
        let operation = ClearOperation::new();
        assert_eq!("clear", operation.name);
        assert_eq!("Clear the list", operation.description);
    }

    #[test]
    fn execute_empty() {
        let lines: Vec<String> = vec![];
        let operation = ClearOperation::new();
        let clear_lines = operation.execute(lines.clone());

        assert_eq!(Vec::<String>::new(), clear_lines.unwrap());
    }

    #[test]
    fn execute_one_element() {
        let lines = vec!["1".to_string()];
        let operation = ClearOperation::new();
        let clear_lines = operation.execute(lines.clone());

        assert_eq!(Vec::<String>::new(), clear_lines.unwrap());
    }

    #[test]
    fn execute_multiple_elements() {
        let lines = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        let operation = ClearOperation::new();
        let clear_lines = operation.execute(lines.clone());

        assert_eq!(Vec::<String>::new(), clear_lines.unwrap());
    }
}
