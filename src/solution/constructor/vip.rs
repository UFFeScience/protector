use log::{debug, error, trace, warn};
use petgraph::graph::EdgeIndex;
use rand::Rng;

use crate::{
    data_mining::RoutePattern,
    graph::{Graph, arc::Cover, vip::InsertedVip},
    input::Distance,
    shared::SelectionControl,
    solution::{Route, path_builder::Dijkstra},
};

pub fn build_vip<'a>(
    graph: &'a Graph,
    vip: &InsertedVip,
    max_distance: Distance,
    rng: &mut impl Rng,
) -> Result<Route<'a>, ()> {
    let (with_reverse, without_reverse) = vip
        .edges
        .iter()
        .copied()
        .partition::<Vec<_>, _>(|&arc| graph[arc].weight.reverse.is_some());

    let result = without_reverse
        .into_iter()
        .map(|arc| vip_solution(graph, arc, vip, max_distance))
        .find_map(Result::ok);

    if let Some(route) = result {
        return Ok(route);
    }

    let mut selection = SelectionControl::new(with_reverse.len());

    while let Some(element) = selection.next(rng) {
        let index = with_reverse[element];

        let solution = vip_solution(graph, index, vip, max_distance);
        if solution.is_ok() {
            return solution;
        }

        let reverse = graph[index].weight.reverse.unwrap();

        let solution = vip_solution(graph, reverse, vip, max_distance);
        if solution.is_ok() {
            return solution;
        }
    }
    error!("Couldn't find a route for VIP {}", vip.id);
    Err(())
}

fn vip_solution<'a>(
    graph: &'a Graph,
    arc: EdgeIndex,
    vip: &InsertedVip,
    max_distance: Distance,
) -> Result<Route<'a>, ()> {
    let mut route = Route::new(graph, arc, max_distance);

    if rec_vip_solution(graph, &mut route, &vip.edges).is_ok() {
        debug!("Route for VIP {} found", vip.id);
        trace!("{:?}", route);
        Ok(route)
    } else {
        Err(())
    }
}

fn rec_vip_solution(graph: &Graph, route: &mut Route, vips: &[EdgeIndex]) -> Result<(), ()> {
    for vip in vips.iter() {
        if vip.is_covered(&route.coverage, route.graph) {
            continue;
        }

        if build_remaining(route, *vip, graph, vips).is_ok() {
            return Ok(());
        }

        if let Some(reverse) = graph[*vip].weight.reverse {
            if build_remaining(route, reverse, graph, vips).is_ok() {
                return Ok(());
            }
        }
    }

    if vips
        .iter()
        .all(|vip| vip.is_covered(&route.coverage, graph))
    {
        let beginning = graph[route.edges[0]].source();
        if route
            .insert_node(beginning, &Dijkstra, |_, _| false)
            .is_ok()
        {
            return Ok(());
        }
    }

    Err(())
}

/// Inserts the `edge` in the route.
/// If successful, it calls [rec_vip_solution] to try to build the rest of the route.
/// If it fails, the route is reset to the position it was before.
fn build_remaining(
    route: &mut Route,
    edge: EdgeIndex,
    graph: &Graph,
    vips: &[EdgeIndex],
) -> Result<(), ()> {
    let initial_len = route.edges.len();

    if route.insert_edge(edge, &Dijkstra, |_, _| false).is_ok() {
        let solution = rec_vip_solution(graph, route, vips);
        if solution.is_ok() {
            return solution;
        } else {
            route.reset_to(initial_len);
        }
    }
    Err(())
}

pub fn build_vip_with_pattern<'a>(
    graph: &'a Graph,
    vip: &InsertedVip,
    max_distance: Distance,
    pattern: Option<&RoutePattern>,
    rng: &mut impl Rng,
) -> Result<Route<'a>, ()> {
    let Some(pattern) = pattern.filter(|p| !p.is_empty()) else {
        warn!("using data mining fallback");
        return build_vip(graph, vip, max_distance, rng);
    };

    let mut route = Route::new(graph, pattern[0], max_distance);

    for arc in pattern[1..].iter().copied() {
        route.insert_edge(arc, &Dijkstra, |_, _| false)?;
    }

    rec_vip_solution(graph, &mut route, &vip.edges)?;
    Ok(route)
}
