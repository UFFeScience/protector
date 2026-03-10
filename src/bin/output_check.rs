mod setup;

use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};
use clap::{ArgAction, Parser};
use cover_crime::{Input, Instance, Output, output::validate};
use log::{debug, info};

#[derive(Debug, Parser)]
#[clap(
    name = "Solution Validator",
    author = "Eduardo Canellas <eduardocanellas@id.uff.br>"
)]
struct Opts {
    /// Path to the graph file
    pub graph: PathBuf,
    /// Path to the solution file
    pub solution: PathBuf,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, action = ArgAction::Count)]
    pub verbose: u8,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    setup::log::init(setup::log::log_level(opts.verbose))
        .with_context(|| "Couldn't start logging")?;

    info!("Building graph...");
    let input = Input::from_file(&opts.graph)?;
    let instance = Instance::new(input)?;
    info!("Graph built succesfully.");

    info!("Parsing solution...");
    let output = std::fs::read_to_string(opts.solution)?;
    let output =
        Output::new(&output).map_err(|it| anyhow!("While parsing the solution: {}", it))?;
    info!("Solution parsed successfully.");

    debug!("Score: {}", output.score);
    debug!("Fixed units: {:?}", output.fixed_units);
    debug!("Global Routes: {}", output.global_routes.len());
    debug!("Zone Routes: {}", output.zone_routes.len());

    info!("Validating solution...");
    validate(&instance, &output)?;
    info!("The solution looks valid!");
    Ok(())
}
