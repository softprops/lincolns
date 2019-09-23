<div align="center">
  👉 #️⃣
</div>
<h1 align="center">
  lincolns
</h1>

<p align="center">
   A JSON Pointer index for line/column information within JSON and YAML content
</p>

<div align="center">
  <a href="https://github.com/softprops/lincolns/actions">
    <img src="https://github.com/softprops/lincolns/workflows/Main/badge.svg"/>
  </a>
  <a href="https://softprops.github.io/lincolns">
    <img src="https://img.shields.io/badge/docs-master-green.svg"/>
  </a>
</div>

<br />

## 📦 install

Add the following to your `Cargo.toml` file's `[dependencies]` heading

```toml
[dependencies]
lincolns = "0.1"
```

## 🤸 usage

Lincolns exposes two sets of operations: one to load YAML/JSON content into an index for lookup and one to perform the lookup

```rust
use lincolns::{from_str, Position}
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  Ok(
    println!(
      "{:#?}",
      from_str("path/to/file.yml")?.get("/path/to/field")
    )
  );
}
```

That's it.

## 🤔 lincolns?

lin(e and )col(umn)n(umber)s

Doug Tangren (softprops) 2019