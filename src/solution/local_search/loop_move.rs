use petgraph::{
    EdgeDirection,
    graph::{EdgeIndex, NodeIndex},
    visit::EdgeRef,
};
use rand::{Rng, prelude::IteratorRandom};

use crate::{
    input::Distance,
    shared::{Coverage, helpers},
    solution::{
        path_builder::{GreedySearch, Restriction},
        route::Route,
        update_coverage,
    },
};

use super::{
    path::{self, Path},
    section_moves::{RemovalParams, expand_route},
};

pub fn add_loop(
    route: &mut Route,
    solution_coverage: &mut Coverage<EdgeIndex>,
    restriction: &impl Restriction,
    params: &super::Params,
    rng: &mut impl Rng,
) -> bool {
    let graph = route.graph;

    let vertices = route
        .edges
        .iter()
        .copied()
        .map(|edge| graph[edge].target())
        .enumerate();

    let incoming = vertices
        .clone()
        .filter(|&(_, vertex)| check_free(route, vertex, EdgeDirection::Incoming))
        .collect::<Vec<_>>();

    let outgoing = vertices
        .filter(|&(_, vertex)| check_free(route, vertex, EdgeDirection::Outgoing))
        .collect::<Vec<_>>();

    if incoming.is_empty() || outgoing.is_empty() {
        return false;
    }

    let first_loop = helpers::iterate_randomly(outgoing.len(), 1, rng)
        .map(|idx| outgoing[idx])
        .flat_map(|(outgoing_position, outgoing)| {
            let edge = graph
                .inner()
                .edges(outgoing)
                .filter(|e| !route.coverage.is_covered(e.id()))
                .filter(|e| !restriction(e.id(), e.target()))
                .choose_stable(rng)?;

            let builder = GreedySearch::new(solution_coverage);

            let sub_route = helpers::iterate_randomly(incoming.len(), 1, rng)
                .map(|idx| incoming[idx])
                // We only want loops which have the incoming vertex before or equal the outgoing one.
                .filter(|&(position, _)| position <= outgoing_position)
                // Limiting the number of edges between the vertices to restrict the search space.
                .filter(|&(position, _)| {
                    let max_size = params.max_edges_between_loop.nearest_int(route.len());
                    outgoing_position - position <= max_size
                })
                .find_map(|incoming| {
                    // We skip the first position because we just want the edges between the two vertices.
                    let start = incoming.0 + 1;
                    let edges_between = &route.edges[start..=outgoing_position];
                    let distance_between = edges_between
                        .iter()
                        .copied()
                        .map(|edge| graph[edge].weight.distance)
                        .sum::<Distance>();

                    let max_distance = route.remaining_distance() - distance_between;
                    let mut sub_route = Route::new(graph, edge.id(), max_distance);

                    sub_route
                        .insert_node(incoming.1, &builder, restriction)
                        .ok()?;

                    // Insert edges between the vertices inside the route and update route's state accordingly.
                    sub_route.edges.extend(edges_between);
                    edges_between
                        .iter()
                        .for_each(|&e| sub_route.add_coverage(e));

                    sub_route.current_distance += distance_between;
                    Some(sub_route)
                })?;

            // NOTE The solution coverage is being cloned because both `expand_route` and `replace`
            // perform updates in the coverage, resulting in duplicated scores.
            // There should be a better solution than this.
            let mut clone = solution_coverage.clone();
            let mut sub_route = update_coverage(sub_route, &mut clone);

            let mut removal_params = RemovalParams {
                solution_coverage: &mut clone,
                route: &mut sub_route,
                path_restriction: restriction,
                special_edge: &|_| false,
            };

            expand_route(&mut removal_params, params.max_section_size, rng);

            let route_distance = sub_route.current_distance();
            // There's an offset of `1` because we want to insert the loop *after* the `vertex`.
            let insertion = outgoing_position + 1;
            let path = Path::new(
                graph,
                insertion,
                sub_route.edges,
                route_distance,
                solution_coverage,
            );

            Some(path)
        })
        .next();

    if let Some(path) = first_loop {
        path::replace(route, 0, path, solution_coverage);
        true
    } else {
        false
    }
}

/// Verify if `vertex` has at least one edge in the given direction which is not covered in the route.
fn check_free(route: &Route, vertex: NodeIndex, dir: EdgeDirection) -> bool {
    route
        .graph
        .inner()
        .edges_directed(vertex, dir)
        .any(|edge| !route.coverage.is_covered(edge.id()))
}
