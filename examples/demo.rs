use lincol::Position;
use std::{error::Error, fs, path::PathBuf, process::exit};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    file: PathBuf,
    field_path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Opts { file, field_path } = Opts::from_args();
    let content = fs::read_to_string(&file)?;
    match lincol::from_str(&content)?.get(field_path) {
        Some(Position { line, col }) => println!("{}:{}", line, col),
        _ => {
            eprintln!("could not find path in {}", file.display());
            exit(1);
        }
    }
    Ok(())
}
