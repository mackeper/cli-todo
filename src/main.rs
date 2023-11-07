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
//! $ cli-todo add "Buy milk"
//! $ cli-todo add "Buy eggs"
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
mod arg_parser;
mod operations;

use arg_parser::parse_operation;

/// Print the usage information for the application.
///
/// # Examples
/// ```
/// use cli_todo::print_usage;
/// print_usage("cli-todo");
/// ```
fn print_usage(program_name: &str) {
    let operations = [
        ("list (l)", "Display all current items."),
        ("add (a) [item]", "Add a new item."),
        ("remove (r) [id]", "Remove an item by its ID."),
        ("done (d) [id]", "Toggle its done state by ID."),
        ("clear", "Remove all items."),
    ];

    let max_command_length = operations
        .iter()
        .map(|(cmd, _)| cmd.len())
        .max()
        .unwrap_or(0);

    println!("Usage: {} [operation] [arguments]", program_name);
    println!("Operations:");
    for (command, description) in operations {
        println!(
            "    {:width$} : {}",
            command,
            description,
            width = max_command_length
        );
   }
}

/// main parses the command line arguments and executes the requested operation.
/// If the operation is not recognized, the usage information is printed.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program_name = &args[0];

    if args.len() < 2 {
        print_usage(program_name);
        return;
    }

    match parse_operation(&args[1], &args[2..]) {
        Ok(op) => {
            if let Err(err) = operations::execute(op) {
                println!("{}", err);
            }
        }
        Err(err) => {
            println!("{}", err);
            print_usage(program_name);
        }
    }
}
