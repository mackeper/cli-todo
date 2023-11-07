//! Operation module.
//!
//! This module contains the `Operation` enum and its variants.
//! The `Operation` enum represents the different operations that can be performed on the todo list.
//! The `OperationError` enum represents the different errors that can occur when performing an operation.
//! The `execute` function takes an `Operation` and performs it on the todo list.
//!
//! # Examples
//! ```
//! use cli_todo::operations::Operation;
//! use cli_todo::executer::execute;
//!
//! let operation = Operation::List;
//! execute(operation).unwrap();
//! ```
pub mod operation;
pub mod operation_error;
pub mod executer;
mod list_io;

pub use operation::*;
pub use operation_error::*;
pub use executer::execute;
