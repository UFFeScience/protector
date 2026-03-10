mod arc;
mod base_types;
mod general_values;
mod graph_input;
pub mod integrity_check;
mod vertex;
mod vip;

use std::path::Path;
use std::str::FromStr;
use std::{fmt::Debug, fs};

use anyhow::{Context, Result, anyhow};
pub use base_types::{CrimeFactor, Distance, Id, Zone};
pub use general_values::GeneralValues;
pub use graph_input::GraphInput;
pub use vip::Vip;

pub struct Input {
    pub general: GeneralValues,
    pub graph_input: GraphInput,
    pub vips: Vec<Vip>,
}

impl Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Input")
            .field("values", &self.general)
            .finish()
    }
}

impl Input {
    pub fn from_file<P: AsRef<Path>>(file_path: &P) -> Result<Self> {
        let content = fs::read_to_string(file_path)?;
        let mut lines = content.lines();
        let first_line = lines
            .next()
            .ok_or_else(|| anyhow!("First line of input file missing"))?;
        let general = GeneralValues::from_str(first_line)?;
        let vertices = take_from_lines(&mut lines, general.vertex_count as usize)?;
        let arcs = take_from_lines(&mut lines, general.edges as usize)?;

        let vips = lines
            .map(Vip::from_str)
            .collect::<Result<_>>()
            .with_context(|| "when extracting VIP lines")?;

        Ok(Input {
            general,
            graph_input: GraphInput::new(vertices.into_iter(), arcs.into_iter()),
            vips,
        })
    }
}

fn take_from_lines<'a, T: FromStr>(
    lines: &mut impl Iterator<Item = &'a str>,
    line_count: usize,
) -> Result<Vec<T>, T::Err> {
    lines
        .take(line_count)
        .map(|line| line.parse::<T>())
        .collect()
}
