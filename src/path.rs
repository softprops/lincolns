use std::fmt::{self, Display};

#[derive(Copy, Clone)]
pub enum Path<'a> {
    Root,
    Seq { parent: &'a Path<'a>, index: usize },
    Map { parent: &'a Path<'a>, key: &'a str },
}

impl<'a> Display for Path<'a> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter,
    ) -> Result<(), fmt::Error> {
        struct Parent<'a>(&'a Path<'a>);

        impl<'a> Display for Parent<'a> {
            fn fmt(
                &self,
                formatter: &mut fmt::Formatter,
            ) -> Result<(), fmt::Error> {
                match *self.0 {
                    Path::Root => formatter.write_str("/"),
                    ref path => write!(formatter, "{}/", path),
                }
            }
        }

        match *self {
            Path::Root => formatter.write_str("/"),
            Path::Seq { parent, index } => write!(formatter, "{}/{}", parent, index),
            Path::Map { parent, key } => write!(formatter, "{}{}", Parent(parent), key),
        }
    }
}
