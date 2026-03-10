mod route_builder;
use std::{fmt::Display, str::FromStr};

use log::{info, warn};
pub use route_builder::Params as RouteParams;

mod vip;

use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    visit::EdgeRef,
};
use rand::{Rng, prelude::IteratorRandom};
pub use vip::{build_vip, build_vip_with_pattern};

use crate::{data_mining::RoutePattern, graph::vertex::Vertex, input::Zone};
use crate::{
    graph::{Graph, arc::Cover},
    input::CrimeFactor,
    shared::Coverage,
    solution::Route,
};

pub fn build_zone<'graph, 'a>(
    params: &RouteParams<'graph, 'a>,
    zone: Zone,
    rng: &mut impl Rng,
) -> Result<Route<'graph>, ()> {
    route_builder::build(
        params,
        &|_, node: NodeIndex| params.graph[node].weight.zone() != zone,
        rng,
    )
}

pub fn build_zone_with_pattern<'graph, 'a>(
    params: &RouteParams<'graph, 'a>,
    zone: Zone,
    pattern: Option<&RoutePattern>,
    rng: &mut impl Rng,
) -> Result<Route<'graph>, ()> {
    if let Some(pattern) = pattern.filter(|p| !p.is_empty()) {
        info!("using data mining");
        route_builder::build_with_pattern(
            pattern,
            params,
            &(|_, node: NodeIndex| params.graph[node].weight.zone() != zone),
            rng,
        )
    } else {
        warn!("fallback to normal builder");
        build_zone(params, zone, rng)
    }
}

pub fn build_global<'graph, 'a>(
    params: &RouteParams<'graph, 'a>,
    rng: &mut impl Rng,
) -> Result<Route<'graph>, ()> {
    route_builder::build(params, &|_, _| false, rng)
}

pub fn build_global_with_pattern<'graph, 'a>(
    params: &RouteParams<'graph, 'a>,
    pattern: Option<&RoutePattern>,
    rng: &mut impl Rng,
) -> Result<Route<'graph>, ()> {
    if let Some(pattern) = pattern.filter(|p| !p.is_empty()) {
        info!("using data mining");
        route_builder::build_with_pattern(pattern, params, &|_, _| false, rng)
    } else {
        warn!("fallback to normal builder");
        build_global(params, rng)
    }
}

/// Amount of crime factor to be earned on [node]
fn crime_factor(node: NodeIndex, graph: &Graph, coverage: &Coverage<EdgeIndex>) -> CrimeFactor {
    graph
        .all_edges(node)
        .filter(|edge| !edge.id().is_covered(coverage, graph))
        .map(|edge| edge.weight().crime_factor)
        .sum::<CrimeFactor>()
}

pub fn greedy_fixed_unit_insertion(graph: &Graph, coverage: &mut Coverage<EdgeIndex>) -> NodeIndex {
    let chosen = graph
        .inner()
        .node_indices()
        .max_by_key(|&node| crime_factor(node, graph, coverage))
        .expect("at least one node on graph");

    cover_fixed_unit_edges(graph, chosen, coverage);

    chosen
}

fn cover_fixed_unit_edges(graph: &Graph, chosen: NodeIndex, coverage: &mut Coverage<EdgeIndex>) {
    // I'm using `coverage.cover...` instead of `edge.cover...´ because I'm iterating over
    // all edges coming in AND out of the `chosen` node. Because of that, the reverses
    // of the edges will already be covered. Using `edge.cover...` would duplicate the
    // coverage for edges with reverses.
    graph
        .all_edges(chosen)
        .for_each(|edge| coverage.cover(edge.id()));
}

#[derive(Debug, Copy, Clone)]
pub enum FixedUnitStrategy {
    Random,
    Greedy,
}

impl FromStr for FixedUnitStrategy {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "random" => FixedUnitStrategy::Random,
            "greedy" => FixedUnitStrategy::Greedy,
            _ => return Err("failed to parse fixed unit strategy"),
        })
    }
}

impl Display for FixedUnitStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FixedUnitStrategy::Random => f.write_str("random"),
            FixedUnitStrategy::Greedy => f.write_str("greedy"),
        }
    }
}

pub fn random_fixed_unit_insertion(
    graph: &Graph,
    coverage: &mut Coverage<EdgeIndex>,
    rng: &mut impl Rng,
) -> NodeIndex {
    let chosen = graph
        .inner()
        .node_indices()
        .choose_stable(rng)
        .expect("at least one node on graph");

    cover_fixed_unit_edges(graph, chosen, coverage);
    chosen
}
