use crate::operations2::Operation;
use crate::operations::OperationError;

pub struct AddOperation {
    name: String,
    shortname: String,
    description: String,
    value: String,
}

impl AddOperation {
    pub fn new(value: String) -> Self {
        AddOperation {
            name: "add".to_string(),
            shortname: "a".to_string(),
            description: "Add a value to the end of the list".to_string(),
            value,
        }
    }
}

impl Operation for AddOperation {
    fn execute(&self, lines: Vec<String>) -> Result<Vec<String>, OperationError> {
        Ok([lines, vec![self.value.clone()]].concat())
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::operations2::{add_operation::AddOperation, operation::Operation};

    #[test]
    fn new() {
        let operation = AddOperation::new("value".to_string());
        assert_eq!("add", operation.name);
        assert_eq!("a", operation.shortname);
        assert_eq!("Add a value to the end of the list", operation.description);
        assert_eq!("value", operation.value);
    }

    #[test]
    fn execute_empty() {
        let value = "value".to_string();
        let lines: Vec<String> = vec![];
        let operation = AddOperation::new(value.clone());
        let new_lines = operation.execute(lines.clone());

        let expected_lines = vec![value];
        assert_eq!(expected_lines, new_lines.unwrap());
    }

    #[test]
    fn execute_one_element() {
        let value = "value".to_string();
        let lines = vec!["1".to_string()];
        let operation = AddOperation::new(value.clone());
        let new_lines = operation.execute(lines.clone());

        let expected_lines = vec!["1".to_string(), value];
        assert_eq!(expected_lines, new_lines.unwrap());
    }

    #[test]
    fn execute_multiple_elements() {
        let value = "value".to_string();
        let lines = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        let operation = AddOperation::new(value.clone());
        let new_lines = operation.execute(lines.clone());

        let expected_lines = vec!["1".to_string(), "2".to_string(), "3".to_string(), value];
        assert_eq!(expected_lines, new_lines.unwrap());
    }
}
