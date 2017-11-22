use chrono::{DateTime as ChronoDateTime, FixedOffset};
use container::Container;

/// StringType defines the various types of strings in an `Item`.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StringType {
    SLB,
    MLB,
    SLL,
    MLL,
}

/// Trivia reprsents metadata about an `Item`.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Trivia<'a> {
    /// Whitespace before a value.
    pub indent: &'a str,
    /// Whitespace after a value, but before a comment.
    pub comment_ws: &'a str,
    /// Comment, starting with # character, or empty string if no comment.
    pub comment: &'a str,
    /// Trailing newline.
    pub trail: &'a str,
}

impl<'a> Trivia<'a> {
    /// Creates an empty Trivia
    pub fn empty() -> Trivia<'a> {
        Trivia {
            indent: "",
            comment_ws: "",
            comment: "",
            trail: "\n",
        }
    }
}

/// The type of a key.
/// Keys can be bare or follow the same rules as
/// either string type.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum KeyType {
    Bare,
    Basic,
    Literal,
}

/// Key represents a mapping between types and values.
#[derive(Hash, Clone)]
pub struct Key<'a> {
    pub t: KeyType,
    pub sep: &'a str,
    pub key: &'a str,
}

impl<'a> Key<'a> {
    /// Creates a new bare key with a standard separator
    pub fn new(k: &'a str) -> Key<'a> {
        Key {
            t: KeyType::Bare,
            sep: " = ",
            key: k,
        }
    }
}

impl<'a> Eq for Key<'a> {}

impl<'a> PartialEq for Key<'a> {
    fn eq(&self, other: &Key) -> bool {
        self.key == other.key
    }
}

impl<'a> ::std::fmt::Debug for Key<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl<'a> Key<'a> {
    pub fn as_string(&self) -> String {
        let quote = match self.t {
            KeyType::Bare => "",
            KeyType::Basic => "\"",
            KeyType::Literal => "'",
        };

        format!("{}{}{}", quote, self.key, quote)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item<'a> {
    WS(&'a str),
    Comment(Trivia<'a>),
    Integer {
        val: i64,
        meta: Trivia<'a>,
        raw: &'a str,
    },
    Float {
        val: f64,
        meta: Trivia<'a>,
        raw: &'a str,
    },
    Bool { val: bool, meta: Trivia<'a> },
    DateTime {
        val: ChronoDateTime<FixedOffset>,
        raw: &'a str,
        meta: Trivia<'a>,
    },
    Array {
        val: Vec<Item<'a>>,
        meta: Trivia<'a>,
    },
    Table {
        is_array: bool,
        val: Container<'a>,
        meta: Trivia<'a>,
    },
    InlineTable {
        val: Container<'a>,
        meta: Trivia<'a>,
    },
    Str {
        t: StringType,
        val: &'a str, // TODO, make Cow
        original: &'a str,
        meta: Trivia<'a>,
    },
    AoT(Vec<Item<'a>>),
}

impl<'a> Item<'a> {
    pub fn discriminant(&self) -> i32 {
        use self::Item::*;
        match *self {
            WS(_) => 0,
            Comment(_) => 1,
            Integer { .. } => 2,
            Float { .. } => 3,
            Bool { .. } => 4,
            DateTime { .. } => 5,
            Array { .. } => 6,
            Table { .. } => 7,
            InlineTable { .. } => 8,
            Str { .. } => 9,
            AoT(_) => 10,
        }
    }

    pub(crate) fn is_homogeneous(&self) -> bool {
        use std::collections::HashSet;
        match *self {
            Item::Array { ref val, .. } => {
                let t = val.iter()
                    .filter_map(|it| match it {
                        &Item::WS(_) |
                        &Item::Comment(_) => None,
                        _ => Some(it.discriminant()),
                    })
                    .collect::<HashSet<_>>()
                    .len();
                t == 1

            }
            _ => unreachable!(),
        }
    }

    /// Return a string representation of the Item.
    pub fn as_string(&self) -> String {
        use self::Item::*;
        match *self {
            WS(s) => s.into(),
            Comment(ref meta) => format!("{}{}{}", meta.indent, meta.comment, meta.trail),
            Integer { raw, .. } => format!("{}", raw),
            Float { raw, .. } => format!("{}", raw),
            Bool { val, .. } => format!("{}", val),
            DateTime { raw, .. } => format!("{}", raw),
            Array { ref val, .. } => {
                let mut buf = String::new();
                buf.push_str("[");
                for item in val.iter() {
                    buf.push_str(&item.as_string());
                }
                buf.push_str("]");
                buf
            }
            Table { ref val, .. } => val.as_string(),
            InlineTable { ref val, .. } => {
                let mut buf = String::new();
                buf.push_str("{");
                for (i, &(ref k, ref v)) in val.body.iter().enumerate() {
                    buf.push_str(&format!(
                        "{}{} = {}{}{}",
                        v.meta().indent,
                        k.clone().unwrap().as_string(),
                        v.as_string(),
                        v.meta().comment,
                        v.meta().trail
                    ));
                    if i != val.body.len() - 1 {
                        buf.push_str(", ");
                    }
                }
                buf.push_str("}");
                buf
            }
            Str {
                ref t,
                original,
                ..
            } => {
                match *t {
                    StringType::MLB => format!(r#""""{}""""#, original),
                    StringType::SLB => format!(r#""{}""#, original),
                    StringType::SLL => format!(r#"'{}'"#, original),
                    StringType::MLL => format!(r#"'''{}'''"#, original),
                }
            }
            AoT(ref body) => {
                let mut b = String::new();
                for table in body {
                    b.push_str(&table.as_string());
                }
                b
            }
        }
    }

    /// Return a reference to the Trivia for the Item.
    pub fn meta(&self) -> &Trivia<'a> {
        use self::Item::*;
        match *self {
            WS(_) | Comment(_) | AoT(_) => {
                println!("{:?}", self);
                panic!("Called meta on non-value Item variant");
            }
            Integer { ref meta, .. } |
            Float { ref meta, .. } |
            Bool { ref meta, .. } |
            DateTime { ref meta, .. } |
            Array { ref meta, .. } |
            Table { ref meta, .. } |
            InlineTable { ref meta, .. } |
            Str { ref meta, .. } => meta,
        }
    }

    /// Return a mutable copy of the Item Trivia.
    pub fn meta_mut(&mut self) -> &mut Trivia<'a> {
        use self::Item::*;
        match *self {
            WS(_) | Comment(_) | AoT(_) => {
                println!("{:?}", self);
                panic!("Called meta on non-value Item variant");
            }
            Integer { ref mut meta, .. } |
            Float { ref mut meta, .. } |
            Bool { ref mut meta, .. } |
            DateTime { ref mut meta, .. } |
            Array { ref mut meta, .. } |
            Table { ref mut meta, .. } |
            InlineTable { ref mut meta, .. } |
            Str { ref mut meta, .. } => meta,
        }
    }
}
