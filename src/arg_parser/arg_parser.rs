use crate::domain::Item;
use crate::operations::Operation;
use crate::arg_parser::ParseError;

/// Parse the command line arguments and return an `Operation` enum.
pub fn parse_operation(op_str: &str, args: &[String]) -> Result<Operation, ParseError> {
    match op_str {
        "list" | "l" => Ok(Operation::List),

        "add" | "a" => args
            .get(0)
            .ok_or(ParseError::InsufficientArguments)
            .map(|item| Operation::Add { item: Item::new(item.clone())}),

        "remove" | "r" => args
            .get(0)
            .ok_or(ParseError::InsufficientArguments)?
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidArgument)
            .map(|id| Operation::Remove { id }),

        "done" | "d" => args
            .get(0)
            .ok_or(ParseError::InsufficientArguments)?
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidArgument)
            .map(|id| Operation::Done { id }),

        "clear" => Ok(Operation::Clear),

        _ => Err(ParseError::UnknownOperation),
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Item;

    #[test]
    fn parse_operation_list() {
        let args = vec![];
        let result = super::parse_operation("list", &args);
        assert_eq!(result.unwrap(), super::Operation::List);
    }

    #[test]
    fn parse_operation_list_short() {
        let args = vec![];
        let result = super::parse_operation("l", &args);
        assert_eq!(result.unwrap(), super::Operation::List);
    }

    #[test]
    fn parse_operation_add() {
        let args = vec!["item".to_string()];
        let result = super::parse_operation("add", &args);
        assert_eq!(
            result.unwrap(),
            super::Operation::Add {
                item: Item::new("item".to_string())
            }
        );
    }

    #[test]
    fn parse_operation_add_short() {
        let args = vec!["item".to_string()];
        let result = super::parse_operation("a", &args);
        assert_eq!(
            result.unwrap(),
            super::Operation::Add {
                item: Item::new("item".to_string())
            }
        );
    }

    #[test]
    fn parse_operation_remove() {
        let args = vec!["1".to_string()];
        let result = super::parse_operation("remove", &args);
        assert_eq!(result.unwrap(), super::Operation::Remove { id: 1 });
    }

    #[test]
    fn parse_operation_remove_short() {
        let args = vec!["1".to_string()];
        let result = super::parse_operation("r", &args);
        assert_eq!(result.unwrap(), super::Operation::Remove { id: 1 });
    }

    #[test]
    fn parse_operation_done() {
        let args = vec!["1".to_string()];
        let result = super::parse_operation("done", &args);
        assert_eq!(result.unwrap(), super::Operation::Done { id: 1 });
    }

    #[test]
    fn parse_operation_done_short() {
        let args = vec!["1".to_string()];
        let result = super::parse_operation("d", &args);
        assert_eq!(result.unwrap(), super::Operation::Done { id: 1 });
    }

    #[test]
    fn parse_operation_clear() {
        let args = vec![];
        let result = super::parse_operation("clear", &args);
        assert_eq!(result.unwrap(), super::Operation::Clear);
    }

    #[test]
    fn parse_operation_unknown() {
        let args = vec![];
        let result = super::parse_operation("unknown", &args);
        assert_eq!(result.unwrap_err(), super::ParseError::UnknownOperation);
    }
}
