mod analysis;
mod data_mining;
mod graph;
mod input;
mod instance;
mod metaheuristic;
pub mod output;
mod run;
mod shared;
mod solution;

pub use analysis::Analysis;
pub use input::Input;
pub use input::integrity_check::check;
pub use instance::Instance;
pub use output::Output;
pub use run::Run;
pub use shared::{Percentage, PercentageError};
pub use solution::{Solution, local_search};

/// Exports all parameter types. Meant to be used as glob import.
pub mod params {
    pub use crate::data_mining::MiningParams;
    pub use crate::local_search::{EnabledNeighborhoods, Neighborhood};
    pub use crate::local_search::Params as LocalSearchParams;
    pub use crate::metaheuristic::Params as HeuristicParams;
    pub use crate::run::Metaheuristic;
    pub use crate::run::RunOpts;
    pub use crate::shared::grasp_selection::Alpha;
    pub use crate::solution::FixedUnitStrategy;
    pub use crate::solution::Params as ConstructionParams;
}
