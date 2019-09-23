use crate::Path;
use log::debug;
use std::collections::BTreeMap;
use yaml_rust::{
    parser::{Event as YamlEvent, MarkedEventReceiver},
    scanner::{Marker, TScalarStyle, TokenType},
};

/// Line and column position of content in a file
#[derive(Debug, PartialEq)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl Into<Position> for Marker {
    fn into(self) -> Position {
        let (line, col) = (self.line(), self.col());
        Position { line, col }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Event {
    Scalar(String, TScalarStyle, Option<TokenType>),
    SequenceStart,
    SequenceEnd,
    MappingStart,
    MappingEnd,
}

#[doc(hidden)]
impl Default for Positions {
    fn default() -> Self {
        Self {
            pos: 0,
            events: Vec::new(),
            index: BTreeMap::new(),
        }
    }
}

/// A table of [Position](struct.Position.html) information
pub struct Positions {
    pos: usize,
    events: Vec<(Event, Marker)>,
    index: BTreeMap<String, Position>,
}

impl Positions {
    /// Gets a yaml fields position within a document given its JSON Pointer path
    ///
    /// JSON Pointer defines a string syntax for identifying a specific value
    /// within a JavaScript Object Notation (JSON) document.
    ///
    /// A Pointer is a Unicode string with the reference tokens separated by `/`.
    /// Inside tokens `/` is replaced by `~1` and `~` is replaced by `~0`. The
    /// addressed value is returned and if there is no such value `None` is
    /// returned.
    ///
    /// For more information read [RFC6901](https://tools.ietf.org/html/rfc6901).
    pub fn get<P>(
        &self,
        ptr: P,
    ) -> Option<&Position>
    where
        P: AsRef<str>,
    {
        self.index.get(ptr.as_ref())
    }

    fn next(&mut self) -> Option<(Event, Position)> {
        self.events.clone().get(self.pos).map(|event| {
            self.pos += 1;
            (event.clone().0, event.1.into())
        })
    }

    /// Returns an iterator over positions
    pub fn iter(&self) -> impl IntoIterator<Item = (&String, &Position)> {
        self.index.iter()
    }

    pub(crate) fn collect(
        &mut self,
        path: &Path,
    ) {
        if let Some((ev, _)) = self.next() {
            match ev {
                Event::SequenceStart => {
                    self.collect_seq(0, path);
                    self.collect(path);
                }
                Event::MappingStart => {
                    self.collect_map(path);
                    self.collect(path);
                }
                other => debug!("unhandled {:?} in collect", other),
            }
        }
    }

    fn collect_seq(
        &mut self,
        index: usize,
        path: &Path,
    ) {
        if let Some((ev, pos)) = self.next() {
            match ev {
                Event::SequenceEnd => (),
                Event::Scalar(_, _, _) => {
                    self.index.insert(
                        format!(
                            "{}",
                            Path::Seq {
                                parent: &path,
                                index: index
                            }
                        ),
                        pos,
                    );
                    self.collect_seq(index + 1, &path);
                }
                Event::MappingStart => {
                    self.collect_map(&Path::Seq {
                        parent: &path,
                        index,
                    });
                    self.collect_seq(index + 1, &path);
                }
                other => debug!("unhandled {:?} in collect_seq", other),
            }
        }
    }

    fn collect_map(
        &mut self,
        path: &Path,
    ) {
        if let Some((ev, pos)) = self.next() {
            match ev {
                Event::MappingEnd => (),
                Event::Scalar(key, _, _) => {
                    let this_path = Path::Map {
                        parent: &path,
                        key: &key,
                    };
                    self.index.insert(format!("{}", this_path), pos);
                    match self.next() {
                        Some((Event::MappingStart, _)) => {
                            self.collect_map(&this_path);
                        }
                        Some((Event::SequenceStart, _)) => {
                            self.collect_seq(0, &this_path);
                        }
                        _ => (),
                    }
                    self.collect_map(&path);
                }
                other => debug!("unhandled {:?} in collect_map", other),
            }
        }
    }
}

#[doc(hidden)]
impl MarkedEventReceiver for Positions {
    fn on_event(
        &mut self,
        event: YamlEvent,
        marker: Marker,
    ) {
        let event = match event {
            YamlEvent::Nothing
            | YamlEvent::StreamStart
            | YamlEvent::StreamEnd
            | YamlEvent::DocumentStart
            | YamlEvent::DocumentEnd
            | YamlEvent::Alias(_) /*come back to Alias later*/=> return,
            YamlEvent::Scalar(value, style, _, tag) => {
                Event::Scalar(value, style, tag)
            }
            YamlEvent::SequenceStart(_) => {
                Event::SequenceStart
            }
            YamlEvent::SequenceEnd => Event::SequenceEnd,
            YamlEvent::MappingStart(_) => {
                Event::MappingStart
            }
            YamlEvent::MappingEnd => Event::MappingEnd,
        };
        self.events.push((event, marker));
    }
}
