use crate::operations::OperationError;
use crate::operations::Operation;

use super::list_io::read_file_lines;
use super::list_io::write_file_lines;


fn list_items(lines: &mut Vec<String>) {
    for (i, line) in lines.iter().enumerate() {
        println!("{}: {}", i + 1, line);
    }
}

fn add_item(lines: &mut Vec<String>, item: String) {
    lines.push(format!("[ ] {}", item));
}

fn remove_item(lines: &mut Vec<String>, id: usize) -> Result<(), OperationError> {
    if id == 0 || id > lines.len() {
        return Err(OperationError::OutOfRange);
    }
    lines.remove(id - 1);
    Ok(())
}

fn toggle_done(lines: &mut Vec<String>, id: usize) -> Result<(), OperationError> {
    if id == 0 || id > lines.len() {
        return Err(OperationError::OutOfRange);
    }

    let line = &mut lines[id - 1];
    if line.starts_with("[x]") {
        line.replace_range(1..2, " ");
    } else if line.starts_with("[ ]") {
        line.replace_range(1..2, "x");
    }
    Ok(())
}

pub fn execute(operation: Operation) -> Result<(), OperationError> {
    let mut lines = read_file_lines().map_err(|x| OperationError::IOError(x))?;
    match operation {
        Operation::List => {}
        Operation::Add { item } => add_item(&mut lines, item),
        Operation::Remove { id } => remove_item(&mut lines, id)?,
        Operation::Done { id } => toggle_done(&mut lines, id)?,
        Operation::Clear => lines.clear(),
    }
    list_items(&mut lines);
    write_file_lines(&lines).map_err(|x| OperationError::IOError(x))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn list_items() {
        let mut lines = vec!["foo".to_string(), "bar".to_string()];
        super::list_items(&mut lines);
    }

    #[test]
    fn add_item() {
        let mut lines = vec!["foo".to_string(), "bar".to_string()];
        super::add_item(&mut lines, "baz".to_string());
        assert_eq!(lines, vec!["foo".to_string(), "bar".to_string(), "[ ] baz".to_string()]);
    }

    #[test]
    fn remove_item() {
        let mut lines = vec!["foo".to_string(), "bar".to_string()];
        super::remove_item(&mut lines, 1).unwrap();
        assert_eq!(lines, vec!["bar".to_string()]);
    }

    #[test]
    fn remove_item_out_of_range() {
        let mut lines = vec!["foo".to_string(), "bar".to_string()];
        let result = super::remove_item(&mut lines, 3);
        assert_eq!(result, Err(super::OperationError::OutOfRange));
    }

    #[test]
    fn toggle_done() {
        let mut lines = vec!["[ ] foo".to_string(), "[x] bar".to_string()];
        super::toggle_done(&mut lines, 1).unwrap();
        assert_eq!(lines, vec!["[x] foo".to_string(), "[x] bar".to_string()]);
    }

    #[test]
    fn toggle_done_already_done() {
        let mut lines = vec!["[ ] foo".to_string(), "[x] bar".to_string()];
        super::toggle_done(&mut lines, 2).unwrap();
        assert_eq!(lines, vec!["[ ] foo".to_string(), "[ ] bar".to_string()]);
    }

    #[test]
    fn toggle_done_out_of_range() {
        let mut lines = vec!["[ ] foo".to_string(), "[x] bar".to_string()];
        let result = super::toggle_done(&mut lines, 3);
        assert_eq!(result, Err(super::OperationError::OutOfRange));
    }
}
