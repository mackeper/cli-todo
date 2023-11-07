//! # Module for parsing command line arguments
//!
//! This module contains the `parse_operation` function which parses the command
//! line arguments and returns an `Operation` enum.
//!
//! # Examples
//! ```
//! use cli_todo::arg_parser::parse_operation;
//! use cli_todo::operations::Operation;
//! use std::env;
//!
//! let args: Vec<String> = env::args().collect();
//! let operation = parse_operation(&args).unwrap();
//! match operation {
//!    Operation::List => println!("List operation"),
//!    Operation::Add { item } => println!("Add operation with item: {}", item),
//!    Operation::Remove { id } => println!("Remove operation with id: {}", id),
//!    Operation::Done { id } => println!("Done operation with id: {}", id),
//!    Operation::Clear => println!("Clear operation"),
//!    Operation::Help => println!("Help operation"),
//! }
//! ```
pub mod arg_parser;
mod parse_error;

pub use arg_parser::*;
use parse_error::ParseError;
