//! # Simulator
//!
//! TODO: everything

#![warn(missing_docs)]

use std::path::Path;

pub mod error;

use error::Error;

/// Simulate the execution of a binary file located at the given input path
///
/// See the [module level documentation](self) for more info
pub fn simulate(_input_path: &Path) -> Result<(), Error> { Ok(()) }
