use std::fmt::Display;

use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub text: String,
    pub done: bool,
    pub created_at: chrono::DateTime<Local>,
}

impl Item {
    pub fn new(text: String) -> Self {
        Item {
            text,
            done: false,
            created_at: Local::now(),
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.done == other.done
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let done = if self.done { "x" } else { " " };
        write!(f, "[{}] {}", done, self.text)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Item;
    use chrono::DateTime;

    #[test]
    fn eq() {
        assert_eq!(Item::new("foo".to_string()), Item::new("foo".to_string()));
    }

    #[test]
    fn neq() {
        assert_ne!(Item::new("foo".to_string()), Item::new("bar".to_string()));
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Item::new("foo".to_string())), "[ ] foo");
    }

    #[test]
    fn display_done() {
        assert_eq!(
            format!(
                "{}",
                Item {
                    text: "foo".to_string(),
                    done: true,
                    created_at: DateTime::parse_from_rfc3339("2021-01-01T00:00:00+00:00")
                        .unwrap()
                        .into()
                }
            ),
            "[x] foo"
        );
    }
}
