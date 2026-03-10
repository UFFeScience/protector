use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use cover_crime::{Input, Instance, check};

fn main() -> Result<()> {
    let opts = Opts::parse();
    println!("Parsing {}...", opts.file.display());
    let input = Input::from_file(&opts.file)?;
    println!("Checking the input...");
    check(&input)?;

    // Instance creation can catch some errors too.
    let _instance = Instance::new(input)?;

    println!("Input passed on all checks.");
    Ok(())
}

#[derive(Debug, Parser)]
#[clap(
    name = "Input Checks for the Cover Crime Problem",
    author = "Eduardo Canellas <eduardocanellas@id.uff.br>"
)]
struct Opts {
    /// Path to the input file to be checked
    pub file: PathBuf,
}
