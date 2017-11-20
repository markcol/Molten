#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#![cfg_attr(feature = "dev", deny(
        missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts, 
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, 
        unused_qualifications
        ))]

#![cfg_attr(feature = "dev", allow(
        unstable_features
        ))]

#![allow(non_snake_case)]
#[allow(unused_imports)]
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

/// NL contains the host OS-specific line terminator. 
/// This is done to allow the code to work properly on both Windows
/// and UNIX-like OSes.
#[cfg(target_os = "windows")]
pub const NL: &'static str = "\r\n";

#[cfg(not(target_os = "windows"))]
pub const NL: &'static str = "\n";