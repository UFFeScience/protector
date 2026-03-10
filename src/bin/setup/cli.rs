use std::path::PathBuf;

use clap::{ArgAction, Parser};
use cover_crime::{Percentage, params::*};

/// Heuristics solutions for the Cover Crime Problem.
#[derive(Parser, Debug)]
#[clap()]
pub struct Opts {
    /// Problem instance file to be solved
    pub instance: PathBuf,

    /// Makes the program not print any logs
    #[clap(short, long)]
    pub quiet: bool,

    /// Used to make the output friendly for the irace calibration
    #[clap(long)]
    pub irace: bool,

    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, action = ArgAction::Count)]
    pub verbose: u8,

    /// Saves the solution result. You can customize the file's name.
    #[clap(long)]
    pub save: Option<Option<PathBuf>>,

    /// The number of executions to be done
    #[clap(short, long, default_value = "10")]
    pub executions: u32,

    /// Number of iterations of the heuristic
    #[clap(long)]
    pub iterations: Option<usize>,

    /// How many seconds the heuristic is allowed to run
    #[clap(long)]
    pub time: Option<u64>,

    #[clap(long, default_value = "0.2")]
    pub alpha: Alpha,

    #[clap(long, default_value = "0.1")]
    pub max_edges_between_loop: Percentage,

    #[clap(long, default_value = "0.1")]
    pub max_section_size: Percentage,

    /// The value used as base to the execution seeds
    #[clap(long, default_value = "1")]
    pub seed: u64,

    /// Makes the generation of solutions run in parallel
    #[clap(short, long)]
    pub parallel: bool,

    /// Controls whether the solution is written to stdout
    #[clap(long)]
    pub print: bool,

    /// Determines which folder will be used to store intermediate solutions
    #[clap(long, default_value = "intermediate_solutions")]
    pub intermediates_folder: PathBuf,

    /// The percentage used as support for the mining procedure
    #[clap(long, default_value = "10")]
    pub mining_support: usize,

    /// Metaheuristic used
    #[clap(long)]
    pub metaheuristic: Metaheuristic,

    /// Fixed unit strategy used in construction
    #[clap(long, default_value = "random")]
    pub fixed_unit_strategy: FixedUnitStrategy,

    /// Neighborhoods to use in local search
    #[clap(long, value_delimiter = ',', default_values_t = [Neighborhood::ExpandRoute, Neighborhood::AddLoop, Neighborhood::RepositionUnit])]
    pub neighborhoods: Vec<Neighborhood>,
}

impl Opts {
    #[allow(dead_code)]
    pub fn heuristic_params(&self) -> HeuristicParams {
        HeuristicParams {
            construction: ConstructionParams {
                alpha: self.alpha,
                fixed_unit_strategy: self.fixed_unit_strategy,
            },
            local_search: LocalSearchParams {
                max_edges_between_loop: self.max_edges_between_loop,
                max_section_size: self.max_section_size,
                neighborhoods: EnabledNeighborhoods::from_list(&self.neighborhoods),
            },
        }
    }
}
