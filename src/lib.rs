//! Molten is a lossless TOML parser that preserves all comments, indentations, 
//! whitespace and internal element ordering, and makes all of these fully 
//! editable via an easy API. It is written with the intent of replacing the 
//! current TOML parser used in cargo-edit, and, eventually, adding that
//! functionality to Cargo itself.

#![deny(
        unsafe_code
        )]

#![warn(
        missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts, 
        trivial_numeric_casts,
        unused_import_braces, 
        unused_qualifications
        )]

#![allow(
        unstable_features, 
        non_snake_case,
        unused_imports
        )]

#![doc(test(attr(allow(unused_variables), deny(warnings))))]

#[macro_use]
extern crate pretty_assertions;
extern crate chrono;
#[macro_use]
extern crate error_chain;

mod tomlchar;
mod errors;
pub mod tomldoc;
pub mod parser;
pub mod api;
pub mod index;
pub mod items;

pub use tomldoc::TOMLDocument;
pub use items::*;

// Only public as a hack for testing;
// Should be private and handled via API
pub mod container;
pub use container::Container;
