use crate::domain::Item;
use crate::domain::List;
use crate::operations::Operation;
use crate::operations::OperationError;

use super::list_io::read_file_lines;
use super::list_io::write_file_lines;

/// Execute the given operation.
///
/// # Errors
/// Returns an error if the operation is invalid or if there is an IO error.
pub fn execute(operation: Operation) -> Result<(), OperationError> {
    let mut list = read_file_lines().map_err(|x| OperationError::IOError(x))?;
    let mut show_details = false;
    match operation {
        Operation::List { details } => {
            show_details = details;
        }
        Operation::Add { item } => add_item(&mut list, item),
        Operation::Remove { id } => remove_item(&mut list, id)?,
        Operation::Edit { id, item } => edit_item(&mut list, id, item)?,
        Operation::Done { id } => toggle_done(&mut list, id)?,
        Operation::Clear { force } => clear_items(&mut list, force),
    }
    list_items(&mut list, show_details);
    write_file_lines(&list).map_err(|x| OperationError::IOError(x))?;
    Ok(())
}

fn list_items(list: &mut List, show_details: bool) {
    for (i, item) in list.items.iter().enumerate() {
        if show_details {
            println!(
                "{}: {} [{}]",
                i + 1,
                item,
                item.created_at.format("%Y-%m-%d %H:%M:%S")
            );
        } else {
            println!("{}: {}", i + 1, item);
        }
    }
}

fn add_item(list: &mut List, item: Item) {
    list.items.push(item);
}

fn remove_item(list: &mut List, id: usize) -> Result<(), OperationError> {
    if id == 0 || id > list.items.len() {
        return Err(OperationError::OutOfRange);
    }
    list.items.remove(id - 1);
    Ok(())
}

fn edit_item(list: &mut List, id: usize, item: Item) -> Result<(), OperationError> {
    if id == 0 || id > list.items.len() {
        return Err(OperationError::OutOfRange);
    }
    list.items[id - 1] = item;
    Ok(())
}

fn toggle_done(list: &mut List, id: usize) -> Result<(), OperationError> {
    if id == 0 || id > list.items.len() {
        return Err(OperationError::OutOfRange);
    }

    let item = &mut list.items[id - 1];
    item.done = !item.done;
    Ok(())
}

fn clear_items(list: &mut List, force: bool) {
    if force {
        list.items.clear();
    } else {
        list.items.retain(|item| !item.done);
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{Item, List};

    #[test]
    fn list_items() {
        let mut list = List::new("foo".to_string());
        super::list_items(&mut list, false);
    }

    #[test]
    fn add_item() {
        let mut list = List::new("foo".to_string());
        let item = Item::new("bar".to_string());
        super::add_item(&mut list, item);
        assert_eq!(list.items.len(), 1);
        assert_eq!(list.items[0].text, "bar".to_string());
    }

    #[test]
    fn remove_item() {
        let mut list = List::new("foo".to_string());
        list.items.push(Item::new("bar".to_string()));
        list.items.push(Item::new("baz".to_string()));
        assert_eq!(list.items.len(), 2);
        assert_eq!(list.items[0].text, "bar".to_string());
        assert_eq!(list.items[1].text, "baz".to_string());

        super::remove_item(&mut list, 1).unwrap();

        assert_eq!(list.items.len(), 1);
        assert_eq!(list.items[0].text, "baz".to_string());
    }

    #[test]
    fn edit_item() {
        let mut list = List::new("foo".to_string());
        list.items.push(Item::new("bar".to_string()));
        list.items.push(Item::new("baz".to_string()));
        assert_eq!(list.items.len(), 2);
        assert_eq!(list.items[0].text, "bar".to_string());
        assert_eq!(list.items[1].text, "baz".to_string());

        super::edit_item(&mut list, 1, Item::new("qux".to_string())).unwrap();

        assert_eq!(list.items.len(), 2);
        assert_eq!(list.items[0].text, "qux".to_string());
        assert_eq!(list.items[1].text, "baz".to_string());
    }

    #[test]
    fn remove_item_out_of_range() {
        let mut list = List::new("foo".to_string());
        list.items.push(Item::new("bar".to_string()));
        list.items.push(Item::new("baz".to_string()));
        assert_eq!(list.items.len(), 2);
        assert_eq!(list.items[0].text, "bar".to_string());
        assert_eq!(list.items[1].text, "baz".to_string());

        let result = super::remove_item(&mut list, 3);

        assert_eq!(result, Err(super::OperationError::OutOfRange));
    }

    #[test]
    fn toggle_done() {
        let mut list = List::new("foo".to_string());
        list.items.push(Item::new("bar".to_string()));
        list.items.push(Item::new("baz".to_string()));

        super::toggle_done(&mut list, 1).unwrap();

        assert_eq!(list.items[0].done, true);
        assert_eq!(list.items[1].done, false);
    }

    #[test]
    fn toggle_done_already_done() {
        let mut list = List::new("foo".to_string());
        list.items.push(Item::new("bar".to_string()));
        list.items.push(Item::new("baz".to_string()));

        super::toggle_done(&mut list, 1).unwrap();
        super::toggle_done(&mut list, 1).unwrap();

        assert_eq!(list.items[0].done, false);
        assert_eq!(list.items[1].done, false);
    }

    #[test]
    fn clear_items_without_force() {
        let mut list = List::new("foo".to_string());
        list.items.push(Item::new("bar".to_string()));
        list.items.push(Item::new("baz".to_string()));
        list.items[0].done = true;

        super::clear_items(&mut list, false);

        assert_eq!(list.items.len(), 1);
        assert_eq!(list.items[0].text, "baz".to_string());
    }

    #[test]
    fn clear_items_with_force() {
        let mut list = List::new("foo".to_string());
        list.items.push(Item::new("bar".to_string()));
        list.items.push(Item::new("baz".to_string()));
        list.items[0].done = true;

        super::clear_items(&mut list, true);

        assert_eq!(list.items.len(), 0);
    }

    #[test]
    fn toggle_done_out_of_range() {
        let mut list = List::new("foo".to_string());
        list.items.push(Item::new("bar".to_string()));
        list.items.push(Item::new("baz".to_string()));

        let result = super::toggle_done(&mut list, 3);

        assert_eq!(result, Err(super::OperationError::OutOfRange));
    }
}
