use crate::operations2::Operation;
use crate::operations::OperationError;

pub struct ListOperation {
    name: String,
    shortname: String,
    description: String,
}

impl ListOperation {
    pub fn new() -> Self {
        ListOperation {
            name: "list".to_string(),
            shortname: "l".to_string(),
            description: "List all items".to_string(),
        }
    }
}

impl Operation for ListOperation {
    fn execute(&self, lines: Vec<String>) -> Result<Vec<String>, OperationError> {
        for (i, line) in lines.iter().enumerate() {
            println!("{}: {}", i + 1, line);
        }
        Ok(lines.clone())
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::operations2::{list_operation::ListOperation, operation::Operation};

    #[test]
    fn new() {
        let operation = ListOperation::new();
        assert_eq!("list", operation.name);
        assert_eq!("l", operation.shortname);
        assert_eq!("List all items", operation.description);
    }

    #[test]
    fn execute_empty() {
        let lines: Vec<String> = vec![];
        let operation = ListOperation::new();
        let new_lines = operation.execute(lines.clone());
        assert_eq!(lines, new_lines.unwrap());
    }

    #[test]
    fn execute_one_element() {
        let lines = vec!["1".to_string()];
        let operation = ListOperation::new();
        let new_lines = operation.execute(lines.clone());
        assert_eq!(lines, new_lines.unwrap());
    }

    #[test]
    fn execute_multiple_elements() {
        let lines = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        let operation = ListOperation::new();
        let new_lines = operation.execute(lines.clone());
        assert_eq!(lines, new_lines.unwrap());
    }
}
