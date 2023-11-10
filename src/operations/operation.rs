use crate::domain::Item;

/// Operations that can be performed on the todo list.
#[derive(Debug)]
pub enum Operation {
    List,
    Add { item: Item },
    Remove { id: usize },
    Edit { id: usize, item: Item },
    Done { id: usize },
    Clear,
}

impl PartialEq for Operation {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Operation::List, Operation::List) => true,
            (Operation::Add { item: item1 }, Operation::Add { item: item2 }) => item1 == item2,
            (Operation::Remove { id: id1 }, Operation::Remove { id: id2 }) => id1 == id2,
            (
                Operation::Edit {
                    id: id1,
                    item: item1,
                },
                Operation::Edit {
                    id: id2,
                    item: item2,
                },
            ) => id1 == id2 && item1 == item2,
            (Operation::Done { id: id1 }, Operation::Done { id: id2 }) => id1 == id2,
            (Operation::Clear, Operation::Clear) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Item;
    use crate::operations::Operation;

    #[test]
    fn eq_list() {
        assert_eq!(Operation::List, Operation::List);
    }

    #[test]
    fn eq_add() {
        assert_eq!(
            Operation::Add {
                item: Item::new("foo".to_string())
            },
            Operation::Add {
                item: Item::new("foo".to_string())
            }
        );
    }

    #[test]
    fn eq_remove() {
        assert_eq!(Operation::Remove { id: 1 }, Operation::Remove { id: 1 });
    }

    #[test]
    fn eq_edit() {
        assert_eq!(
            Operation::Edit {
                id: 1,
                item: Item::new("foo".to_string())
            },
            Operation::Edit {
                id: 1,
                item: Item::new("foo".to_string())
            }
        );
    }

    #[test]
    fn eq_done() {
        assert_eq!(Operation::Done { id: 1 }, Operation::Done { id: 1 });
    }

    #[test]
    fn eq_clear() {
        assert_eq!(Operation::Clear, Operation::Clear);
    }

    #[test]
    fn ne_list_add() {
        assert_ne!(
            Operation::List,
            Operation::Add {
                item: Item::new("foo".to_string())
            }
        );
    }
}
