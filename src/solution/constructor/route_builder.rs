use std::cmp::Reverse;

use log::warn;
use petgraph::{graph::EdgeIndex, visit::EdgeRef};
use rand::Rng;

use crate::{
    data_mining::RoutePattern,
    graph::Graph,
    input::Distance,
    params::Alpha,
    shared::{Coverage, SelectionControl},
    solution::{
        contribution::addition_contribution,
        path_builder::{Dijkstra, Restriction},
        route::Route,
    },
};

use super::crime_factor;

pub struct Params<'graph, 'a> {
    pub max_distance: Distance,
    pub graph: &'graph Graph,
    pub coverage: &'a Coverage<EdgeIndex>,
    pub alpha: Alpha,
}

/// To be used by global routes and zone routes.
pub fn build<'graph, 'a, F>(
    params: &Params<'graph, 'a>,
    restriction: &F,
    rng: &mut impl Rng,
) -> Result<Route<'graph>, ()>
where
    F: Restriction,
{
    let mut vertices: Vec<_> = params.graph.inner().node_indices().collect();

    vertices.sort_by_cached_key(|&node| Reverse(crime_factor(node, params.graph, params.coverage)));

    vertices.truncate(params.alpha.rcl_size(vertices.len()));

    let mut selection = SelectionControl::new(vertices.len());

    while let Some(initial_vertex) = selection.next(rng).map(|s| vertices[s]) {
        let maybe_id = params
            .graph
            .inner()
            .edges(initial_vertex)
            .filter(|edge| !restriction(edge.id(), edge.target()))
            .max_by_key(|edge| addition_contribution(params.graph, edge.id(), params.coverage))
            .map(|edge| edge.id());

        let edge = match maybe_id {
            Some(id) => id,
            None => continue,
        };

        let mut route = Route::new(params.graph, edge, params.max_distance);

        // Connect the `greedy_edge`'s endpoints
        if route
            .insert_node(initial_vertex, &Dijkstra, restriction)
            .is_ok()
        {
            return Ok(route);
        }
    }

    Err(())
}

/// To be used by global routes and zone routes.
pub fn build_with_pattern<'graph, 'a, F>(
    pattern: &RoutePattern,
    params: &Params<'graph, 'a>,
    restriction: &F,
    rng: &mut impl Rng,
) -> Result<Route<'graph>, ()>
where
    F: Restriction,
{
    let builder = Dijkstra;
    let initial_edge = pattern[0];
    let mut route = Route::new(params.graph, initial_edge, params.max_distance);

    // NOTE: instead of reusing the existing `insert_edge` function, it would be more efficient
    // to just insert the whole pattern at once. It requires changing the `Route` type.
    for edge in pattern[1..].iter().copied() {
        route.insert_edge(edge, &builder, restriction)?;
    }

    let initial_vertex = params.graph[initial_edge].source();

    // Close the route
    if route
        .insert_node(initial_vertex, &builder, restriction)
        .is_ok()
    {
        Ok(route)
    } else {
        warn!("building with pattern failed. Using fallback.");
        build(params, restriction, rng)
    }
}
