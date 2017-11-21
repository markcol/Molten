//! Container provides a container for items in a TOML file.
use std::collections::HashMap;

use items::*;
use errors::*;

/// Container hold a group of items from a `TOMLDocument`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Container<'a> {
    pub(crate) map: HashMap<Key<'a>, usize>,
    pub(crate) body: Vec<(Option<Key<'a>>, Item<'a>)>,
}

impl<'a> Container<'a> {
    /// Create a new Container and return it to the caller.
    pub fn new() -> Container<'a> {
        Container {
            map: HashMap::new(),
            body: Vec::new(),
        }
    }

    /// Append an item to the container.
    pub fn append<K: Into<Option<Key<'a>>>>(&mut self, _key: K, item: Item<'a>) -> Result<()> {
        let key = _key.into();
        if let Some(k) = key.clone() {
            // TODO: Fix AoT
            // if self.map.contains_key(&k) {
            //     bail!(ErrorKind::DuplicateKey(k.key.into()));
            // }
            self.map.insert(k, self.body.len());
        }
        self.body.push((key, item));
        Ok(())
    }

    // @cleanup: minimize duplication with Item::as_string()
    /// Converts a string slice to a wrapper type providing a &String interface.
    pub fn as_string(&self) -> String {
        let mut s = String::new();
        for (k, v) in self.body.clone().into_iter() {
            let cur: String = if k.is_some() {
                match v {
                    Item::Table { is_array, .. } => {
                        let (open, close) = if is_array { ("[[", "]]") } else { ("[", "]") };
                        format!("{}{}{}{}{}{}{}{}",
                        v.meta().indent,
                        open,
                        k.unwrap().as_string(),
                        close,
                        v.meta().comment_ws,
                        v.meta().comment,
                        v.meta().trail,
                        v.as_string(),)
                    }
                    Item::AoT(vec) => {
                        let mut buf = String::new();
                        let key = k.unwrap().as_string();
                        for table in vec {
                            buf.push_str(&format!(
                                "{}[[{}]]{}{}{}",
                                table.meta().indent,
                                key,
                                table.meta().comment_ws,
                                table.meta().comment,
                                table.meta().trail
                            ));
                            buf.push_str(&table.as_string());
                        }
                        buf
                    }
                    _ => {
                        let k = k.unwrap();
                        format!(
                            "{}{}{}{}{}{}{}",
                            v.meta().indent,
                            k.as_string(),
                            k.sep,
                            v.as_string(),
                            v.meta().comment_ws,
                            v.meta().comment,
                            v.meta().trail
                        )
                    }
                }
            } else {
                v.as_string()
            };
            s.push_str(&cur)
        }
        s
    }

    /// Return a Container iterator.
    pub fn iter(&'a self) -> ContainerIterator<'a> {
        ContainerIterator {
            container: self,
            current: 0,
        }
    }

    /// Return an exhaustive Container iterator.
    pub fn iter_exhaustive(&self) -> ContainerIteratorExhaustive {
        ContainerIteratorExhaustive {
            container: self,
            current: 0,
        }
    }
}

/// Iterator that yields the items within a `Container`.
#[derive(Debug)]
pub struct ContainerIterator<'a> {
    container: &'a Container<'a>,
    current: usize,
}

impl<'a> Iterator for ContainerIterator<'a> {
    type Item = &'a Item<'a>;

    // @cleanup: "There must be a better way"
    fn next(&mut self) -> Option<&'a Item<'a>> {
        loop {
            if self.current == self.container.body.len() {
                return None;
            }
            if self.container.body[self.current].1.is_value() {
                let r = Some(&self.container.body[self.current].1);
                self.current += 1;
                return r;
            } else {
                self.current += 1;
            }
        }
    }
}

/// Iterator that yields all items within a `Container`.
#[derive(Debug)]
pub struct ContainerIteratorExhaustive<'a> {
    container: &'a Container<'a>,
    current: usize,
}

impl<'a> Iterator for ContainerIteratorExhaustive<'a> {
    type Item = &'a Item<'a>;

    fn next(&mut self) -> Option<&'a Item<'a>> {
        if self.current != self.container.body.len() {
            let r = &self.container.body[self.current];
            self.current += 1;
            Some(&r.1)
        } else {
            None
        }
    }
}
