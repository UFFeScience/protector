use anyhow::{Context, Result};
use petgraph::graph::EdgeIndex;

use crate::{graph::Graph, input::Vip};

#[derive(Debug)]
pub struct InsertedVip {
    pub id: u32,
    pub edges: Vec<EdgeIndex>,
}

impl InsertedVip {
    pub fn new(vip: &Vip, graph: &Graph) -> Result<Self> {
        let edges = vip
            .vip_arcs
            .iter()
            .map(|(origin, destiny)| graph.find_edge(origin, destiny))
            .collect::<Result<_>>()
            .context("While searching for vips arcs on the graph")?;

        Ok(Self { id: vip.id, edges })
    }
}

/// Creates [InsertedVip] based on [Vip], adding edges information.
pub fn map_vips(vips: &[Vip], graph: &Graph) -> Result<Vec<InsertedVip>> {
    Ok(vips
        .iter()
        .map(|vip| InsertedVip::new(vip, graph))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(|vip| !vip.edges.is_empty())
        .collect())
}
