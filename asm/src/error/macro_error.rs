#![allow(missing_docs)]

use std::fmt::{Display, Formatter};

use super::print::{make_info_body, make_info_header};
use super::LocationInfo;

#[derive(Debug)]
pub enum MacroError {
	UnknownMacro { src_file: String, location: LocationInfo },
}
