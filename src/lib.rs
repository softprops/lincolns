//! Provides a [JSON Pointer](https://tools.ietf.org/html/rfc6901) lookup of line/column information within JSON and YAML content
//!
//! # Example
//!
//! The following loads YAML content into a structure
//! that can be queried by JSON Pointer paths
//!
//! ```rust,edition2018
//! use lincolns::{from_str, Position};
//!
//! # fn main() -> lincolns::Result<()>  {
//! let positions = from_str(
//! r#"foo:
//!     - bar: baz
//!       boom: true
//! "#
//! )?;
//!
//! assert_eq!(
//!   positions.get("/foo/0/boom"),
//!   Some(
//!     &Position { line: 3,  col: 6 }
//!   )
//!);
//!
//! assert_eq!(
//!   positions.get("/foo/0/zoom"),
//!   None
//! );
//! # Ok(())
//! # }
//! ```
mod error;
mod path;
mod position;

pub use error::{Error, Result};
use path::Path;
pub use position::{Position, Positions};
use std::io::Read;
use yaml_rust::parser::Parser;

/// Load a lookup table of `Position` information
/// from utf8 text
pub fn from_str<S>(s: S) -> Result<Positions>
where
    S: AsRef<str>,
{
    let mut parser = Parser::new(s.as_ref().chars());
    let mut positions = Positions::default();
    parser.load(&mut positions, true)?;
    positions.collect(&Path::Root);
    Ok(positions)
}

/// Load a lookup table of `Position` information from a type which implements
/// `Read`
pub fn from_reader<R>(mut rdr: R) -> Result<Positions>
where
    R: Read,
{
    let mut bytes = Vec::new();
    rdr.read_to_end(&mut bytes)?;
    let s = std::str::from_utf8(&bytes)?;
    from_str(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_with_json() -> Result<()> {
        let positions = from_str(include_str!("../tests/data/example.json"))?;
        assert_eq!(
            positions.get("/test/2/nested/foo"),
            Some(&Position { line: 13, col: 10 })
        );
        Ok(())
    }

    #[test]
    fn from_str_with_yaml() -> Result<()> {
        let positions = from_str(include_str!("../tests/data/example.yml"))?;
        assert_eq!(
            positions.get("/test/2/nested/foo"),
            Some(&Position { line: 7, col: 6 })
        );
        Ok(())
    }

    #[test]
    fn impl_into_iter() -> Result<()> {
        let positions = from_str(include_str!("../tests/data/example.yml"))?;
        assert!(positions.iter().into_iter().next().is_some());
        Ok(())
    }
}
