/*! 
Molten is a lossless TOML parser that preserves all comments, indentations, 
whitespace and internal element ordering, and makes all of these fully 
editable via an easy API. It is written with the intent of replacing the 
current TOML parser used in [cargo-edit](https://github.com/killercup/cargo-edit),
and, eventually, adding that functionality to 
[cargo](https://github.com/rust-lang/cargo) itself.

## Goals

- *Speed*: Molten is a one-pass parser which avoids allocation.
- *Unopinionated*: Molten respects the way you wrote your document, to the letter.
- *Fully addressable*: All TOML elements can be edited, created, or deleted via the API.

## Non-Goals
- *Error recovery*: Molten does not try to correct recoverable errors.
- *Serialization/Deserialization*: See [toml-rs](https://github.com/alexcrichton/toml-rs) for this.


# Setup

Add this to your `Cargo.toml`:

```toml
[dependencies]
molten = "1.0.0"
```

and this to your crate root:

```text
extern crate Molten;
```

# Example

Here is a example of using Molten to parse a trivial TOML document:

```rust
let toml = String::from("bool = true\nstring = \"Hello!\"\nint = 42");
let parsed = {
    let mut parser = Molten::parser::Parser::new(&toml);
    parser.parse().unwrap()
};

assert_eq!(toml, parsed.as_string());
```
*/

// #![warn(missing_docs)]
#![allow(non_snake_case)]
//#[allow(unused_imports)]
//#[macro_use]
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

#[doc(inline)]
pub use tomldoc::TOMLDocument;
#[doc(inline)]
pub use items::*;

// Only public as a hack for testing;
// Should be private and handled via API
#[doc(inline)]
pub mod container;
#[doc(inline)]
pub use container::Container;

/// Line terminator constant.
pub const NL: &str = "\n";