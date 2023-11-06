use crate::{operations::OperationError, operations2::Operation};

pub struct RemoveOperation {
    name: String,
    shortname: String,
    description: String,
    index: usize,
}

impl RemoveOperation {
    pub fn new(index: usize) -> Self {
        RemoveOperation {
            name: "remove".to_string(), 
            shortname: "r".to_string(),
            description: "Remove an item from the list".to_string(),
            index,
        }
    }
}

impl Operation for RemoveOperation {
    fn execute(&self, lines: Vec<String>) -> Result<Vec<String>, OperationError> {
        if self.index >= lines.len(){
            return Err(OperationError::OutOfRange);
        }

        Ok(lines
            .iter()
            .enumerate()
            .filter_map(|(i, line)| {
                if i == self.index {
                    None
                } else {
                    Some(line.clone())
                }
            })
            .collect())
    }
    
    fn description(&self) -> String {
        self.description.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::operations2::{operation::Operation, remove_operation::RemoveOperation};
    use crate::operations::OperationError;

    #[test]
    fn new() {
        let operation = RemoveOperation::new(1);
        assert_eq!("remove", operation.name);
        assert_eq!("r", operation.shortname);
        assert_eq!("Remove an item from the list", operation.description);
        assert_eq!(1, operation.index);
    }

    #[test]
    fn execute_empty() {
        let index = 1;
        let lines: Vec<String> = vec![];
        let operation = RemoveOperation::new(index.clone());
        let result = operation.execute(lines.clone());

        assert_eq!(Err(OperationError::OutOfRange), result);
    }

    #[test]
    fn execute_one_element() {
        let index = 0;
        let lines = vec!["1".to_string()];
        let operation = RemoveOperation::new(index.clone());
        let remove_lines = operation.execute(lines.clone());

        assert_eq!(Vec::<String>::new(), remove_lines.unwrap());
    }

    #[test]
    fn execute_multiple_elements() {
        let index = 1;
        let lines = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        let operation = RemoveOperation::new(index.clone());
        let remove_lines = operation.execute(lines.clone());

        let expected_lines = vec!["1".to_string(), "3".to_string()];
        assert_eq!(expected_lines, remove_lines.unwrap());
    }
}
