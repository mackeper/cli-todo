//! Command line todo list application.
//!
//! This application is a simple todo list that can be used from the command line.
//! It stores the todo list in a file in the user's home directory.
//! The file is called `default-todo.txt` on Linux and `cli-todo\default-todo.txt` on Windows.
//! The file is created if it does not exist.
//! The file is overwritten when the application exits.
//! The file is a plain text file with one todo item per line.
//! Each line starts with either `[ ]` or `[x]` to indicate whether the item is done or not.
//! The rest of the line is the todo item text.
//! The application supports the following operations:
//! - `list (l)`: Display all current items.
//! - `add (a) [item]`: Add a new item.
//! - `remove (r) [id]`: Remove an item by its ID.
//! - `done (d) [id]`: Toggle its done state by ID.
//! - `clear`: Remove all items.
//!
//! # Examples
//! ```
//! $ cli-todo add "Buy oatmilk"
//! $ cli-todo add "Buy kale"
//! $ cli-todo list
//! 1: [ ] Buy oatmilk
//! 2: [ ] Buy kale
//! $ cli-todo done 1
//! $ cli-todo list
//! 1: [x] Buy oatmilk
//! 2: [ ] Buy kale
//! $ cli-todo remove 2
//! $ cli-todo list
//! 1: [x] Buy oatmilk
//! ```
mod domain;
mod operations;

use clap::{Parser, Subcommand};
use domain::Item;
use operations::{executer, Operation};

#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[command(subcommand)]
    operation: CliOperations,
}

#[derive(Subcommand)]
enum CliOperations {
    /// List all items
    #[clap(visible_alias = "l")]
    List,

    /// Add a new item
    #[clap(visible_alias = "a")]
    Add { item: String },

    /// Remove an item by its ID
    #[clap(visible_alias = "r")]
    Remove { id: usize },

    /// Edit an item by its ID
    #[clap(visible_alias = "e")]
    Edit { id: usize, item: String },

    /// Toggle the done state of an item by its ID
    #[clap(visible_alias = "d")]
    Done { id: usize },

    /// Remove all items that are done
    Clear {
        /// Force clear all items
        #[clap(short, long)]
        force: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let operation = match cli.operation {
        CliOperations::List => Operation::List,
        CliOperations::Add { item } => Operation::Add {
            item: Item::new(item),
        },
        CliOperations::Remove { id } => Operation::Remove { id },
        CliOperations::Edit { id, item } => Operation::Edit {
            id,
            item: Item::new(item),
        },
        CliOperations::Done { id } => Operation::Done { id },
        CliOperations::Clear { force } => Operation::Clear { force },
    };

    match executer::execute(operation) {
        Ok(_) => {}
        Err(_) => {}
    };
}
