use crate::operations2::Operation;
use crate::operations::OperationError;

pub struct DoneOperation {
    name: String,
    shortname: String,
    description: String,
    index: usize,
}

impl DoneOperation {
    pub fn new(index: usize) -> Self {
        DoneOperation {
            name: "done".to_string(),
            shortname: "d".to_string(),
            description: "Toggle the done status of an item".to_string(),
            index,
        }
    }
}

impl Operation for DoneOperation {
    fn execute(&self, lines: Vec<String>) -> Result<Vec<String>, OperationError> {
        if self.index >= lines.len() {
            return Err(OperationError::OutOfRange);
        }
        let mut lines = lines.clone();

        let line = &mut lines[self.index];
        if line.starts_with("[x]") {
            line.replace_range(1..2, " ");
        } else if line.starts_with("[ ]") {
            line.replace_range(1..2, "x");
        }
        Ok(lines)
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::operations2::{operation::Operation, done_operation::DoneOperation};
    use crate::operations::OperationError;

    #[test]
    fn new() {
        let operation = DoneOperation::new(1);
        assert_eq!("done", operation.name);
        assert_eq!("d", operation.shortname);
        assert_eq!("Toggle the done status of an item", operation.description);
        assert_eq!(1, operation.index);
    }

    #[test]
    fn execute_out_of_range_error() {
        let index = 1;
        let lines: Vec<String> = vec![];
        let operation = DoneOperation::new(index.clone());
        let result = operation.execute(lines.clone());

        assert_eq!(OperationError::OutOfRange, result.unwrap_err());
    }

    #[test]
    fn execute_one_element_toggle_done() {
        let index = 0;
        let lines = vec!["[ ] 1".to_string()];
        let operation = DoneOperation::new(index.clone());
        let done_lines = operation.execute(lines.clone());

        let expected_lines: Vec<String> = vec!["[x] 1".to_string()];
        assert_eq!(expected_lines, done_lines.unwrap());
    }

    #[test]
    fn execute_one_element_toggle_undone() {
        let index = 0;
        let lines = vec!["[x] 1".to_string()];
        let operation = DoneOperation::new(index.clone());
        let done_lines = operation.execute(lines.clone());

        let expected_lines: Vec<String> = vec!["[ ] 1".to_string()];
        assert_eq!(expected_lines, done_lines.unwrap());
    }

    #[test]
    fn execute_multiple_elements_toggle_done() {
        let index = 1;
        let lines = vec!["[ ] 1".to_string(), "[ ] 2".to_string(), "[ ] 3".to_string()];
        let operation = DoneOperation::new(index.clone());
        let done_lines = operation.execute(lines.clone());

        let expected_lines = vec!["[ ] 1".to_string(), "[x] 2".to_string(), "[ ] 3".to_string()];
        assert_eq!(expected_lines, done_lines.unwrap());
    }

    #[test]
    fn execute_multiple_elements_toggle_undone() {
        let index = 1;
        let lines = vec!["[ ] 1".to_string(), "[x] 2".to_string(), "[ ] 3".to_string()];
        let operation = DoneOperation::new(index.clone());
        let done_lines = operation.execute(lines.clone());

        let expected_lines = vec!["[ ] 1".to_string(), "[ ] 2".to_string(), "[ ] 3".to_string()];
        assert_eq!(expected_lines, done_lines.unwrap());
    }
}
