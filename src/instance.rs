use std::convert::TryInto;

use itertools::Itertools;
use petgraph::visit::IntoNodeReferences;

use crate::{
    graph::{
        Graph,
        vertex::Vertex,
        vip::{InsertedVip, map_vips},
    },
    input::{Distance, GeneralValues, Input, Vip, Zone},
};

#[derive(Debug)]
pub struct Instance {
    graph: Graph,
    inserted: Vec<InsertedVip>,
    zones: Vec<Zone>,
    general: GeneralValues,
    input: Vec<Vip>,
}

impl Instance {
    pub fn new(input: Input) -> anyhow::Result<Self> {
        let graph = Graph::new(input.graph_input)?;
        let vips = map_vips(&input.vips, &graph)?;

        // When a vertex has zone "0", we consider that it doesn't belong to any zone.
        let ignored_zone = Zone::new(0);
        let zones = graph
            .inner()
            .node_references()
            .map(|(_idx, weight)| weight.zone())
            .sorted()
            .dedup()
            .filter(|&zone| zone != ignored_zone)
            .collect();

        Ok(Self {
            graph,
            inserted: vips,
            zones,
            general: input.general,
            input: input.vips,
        })
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    pub fn vertices(&self) -> usize {
        self.general.vertex_count.try_into().unwrap()
    }

    pub fn arcs(&self) -> usize {
        self.general.edges.try_into().unwrap()
    }

    pub fn global_distance(&self) -> Distance {
        self.general.global_routes.max_distance
    }

    pub fn zones(&self) -> impl Iterator<Item = Zone> + '_ {
        self.zones.iter().copied()
    }

    pub fn zone_distance(&self) -> Distance {
        self.general.zone_routes.max_distance
    }

    pub fn routes_per_zone(&self) -> usize {
        self.general.zone_routes.quantity.try_into().unwrap()
    }

    /// The number of global routes which aren't VIP
    pub fn global_routes(&self) -> u32 {
        self.general.global_routes.quantity - self.inserted.len() as u32
    }

    pub fn fixed_units(&self) -> u32 {
        self.general.fixed_units
    }

    pub fn inserted_vips(&self) -> &[InsertedVip] {
        &self.inserted
    }

    pub fn input_vips(&self) -> &[Vip] {
        &self.input
    }
}
