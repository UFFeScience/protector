use ordered_float::NotNan;
use petgraph::{graph::EdgeIndex, visit::EdgeRef};

use crate::{
    graph::{
        Graph,
        arc::{self, Cover},
    },
    input::CrimeFactor,
    shared::Coverage,
};

use super::route::Route;

pub(crate) fn compute_score(graph: &Graph, coverage: &Coverage<EdgeIndex>) -> NotNan<f64> {
    let score = *graph
        .inner()
        .edge_references()
        .map(|edge| coverage_contribution(graph, coverage, edge.id()))
        .sum::<CrimeFactor>()
        .as_ref();
    score
}

/// The actual [CrimeFactor] contribution that an arc adds in the solution.
/// This is used to compute the final crime factor.
///
/// - If it's not covered, no contribution.
///
/// - If it's covered and doesn't have a reverse arc, or the reverse arc is already
/// covered, its contribution is its crime factor.
///
/// - If it's covered and has a reverse which is not covered, its contribution
///  is doubled.
pub fn coverage_contribution(
    graph: &Graph,
    coverage: &Coverage<EdgeIndex>,
    index: EdgeIndex,
) -> CrimeFactor {
    let weight = &graph[index].weight;
    let crime_factor = weight.crime_factor;
    let reverse = weight.reverse;

    if !coverage.is_covered(index) {
        CrimeFactor::default()
    } else if reverse
        .filter(|reverse| !coverage.is_covered(*reverse))
        .is_some()
    {
        crime_factor + crime_factor
    } else {
        crime_factor
    }
}

/// Measures the amount of [CrimeFactor] that an `edge` would contribute to the solution if chosen based on coverage's current state.
pub fn addition_contribution(
    graph: &Graph,
    edge: EdgeIndex,
    coverage: &Coverage<EdgeIndex>,
) -> CrimeFactor {
    if edge.is_covered(coverage, graph) {
        CrimeFactor::default()
    } else {
        let arc::Weight {
            reverse,
            crime_factor,
            ..
        } = graph[edge].weight;

        if reverse.is_some() {
            crime_factor + crime_factor
        } else {
            crime_factor
        }
    }
}

/// Used to evaluate contribution on a specific route/section
pub fn actual_contribution(
    route: &Route,
    edge: EdgeIndex,
    coverage: &Coverage<EdgeIndex>,
) -> CrimeFactor {
    let weight = route.graph[edge].weight;
    let reverse = weight.reverse;

    let covered_only_by_me = coverage.count(edge) == route.coverage.count(edge) && {
        match reverse {
            Some(reverse) => coverage.count(reverse) == route.coverage.count(reverse),
            None => true,
        }
    };

    if covered_only_by_me {
        let crime_factor = weight.crime_factor;

        if reverse.is_some() {
            crime_factor + crime_factor
        } else {
            crime_factor
        }
    } else {
        // There's another unit covering it,
        // so we can remove it without losing crime factor.
        CrimeFactor::default()
    }
}
