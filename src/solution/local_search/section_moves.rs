use petgraph::graph::EdgeIndex;
use rand::Rng;

use crate::{
    Percentage,
    graph::Graph,
    input::{CrimeFactor, Distance},
    shared::{Coverage, helpers},
    solution::{
        contribution::actual_contribution,
        local_search,
        path_builder::{GreedySearch, PathBuilder, Restriction},
        route::Route,
    },
};

use super::path::Path;

/// A piece of route
#[derive(Debug)]
pub struct Section<'a> {
    edges: &'a [EdgeIndex],
    position: usize,
}

impl<'a> Section<'a> {
    /// Creates a new Section. Returns `Err` if there's at least one edge in the section
    /// which is special and cannot be removed. The `Err` value returns this edge's position.
    fn new<G>(edges: &'a [EdgeIndex], position: usize, special_edge: G) -> Result<Self, usize>
    where
        G: Fn(EdgeIndex) -> bool,
    {
        if let Some(idx) = edges.iter().copied().position(special_edge) {
            Err(idx)
        } else {
            Ok(Self { edges, position })
        }
    }

    pub fn first(&self) -> EdgeIndex {
        self.edges[0]
    }

    pub fn last(&self) -> EdgeIndex {
        self.edges[self.edges.len() - 1]
    }

    fn crime_factor(&self, route: &Route, coverage: &Coverage<EdgeIndex>) -> CrimeFactor {
        self.edges
            .iter()
            .map(|&edge| actual_contribution(route, edge, coverage))
            .sum::<CrimeFactor>()
    }

    fn distance(&self, graph: &Graph) -> Distance {
        self.edges
            .iter()
            .map(|edge| graph[*edge].weight.distance)
            .sum::<Distance>()
    }
}

pub fn expand_route(
    params: &mut RemovalParams,
    section_percentage: Percentage,
    rng: &mut impl Rng,
) {
    let mut section_size = 1;
    loop {
        if section_removal(params, section_size, rng).is_some() {
            // reset removal size
            section_size = 1
        } else {
            section_size += 1;

            let max_size = section_percentage.nearest_int(params.route.len());

            if section_size > max_size {
                break;
            }
        }
    }
}

fn find_first_improving_path(
    params: &mut RemovalParams,
    removal_size: usize,
    rng: &mut impl Rng,
) -> Option<Path> {
    let route = &params.route;
    let solution_coverage = &params.solution_coverage;
    let path_builder = GreedySearch::new(solution_coverage);
    let mut coverage = Coverage::<EdgeIndex>::new(route.graph.inner().edge_count());

    helpers::iterate_randomly(route.edges.len(), removal_size, rng)
        .flat_map(|start| {
            let edges = &route.edges[start..start + removal_size];

            let section = Section::new(edges, start, params.special_edge).ok()?;

            let origin = route.graph[section.first()].source();
            let target = route.graph[section.last()].target();

            let max_distance = route.remaining_distance() + section.distance(route.graph);

            for edge in section.edges {
                coverage.cover(*edge);
            }

            let restriction = |e, n| (params.path_restriction)(e, n) || coverage.is_covered(e);

            let path = path_builder
                .build(route.graph, origin, target, max_distance, restriction)
                .map(|(edges, distance)| {
                    Path::new(route.graph, start, edges, distance, solution_coverage)
                })
                .filter(|path| {
                    path.crime_factor() > section.crime_factor(route, solution_coverage)
                });

            for edge in section.edges {
                coverage.uncover(*edge);
            }

            path
        })
        .next()
}

pub struct RemovalParams<'a, 'graph> {
    pub solution_coverage: &'a mut Coverage<EdgeIndex>,
    pub route: &'a mut Route<'graph>,
    pub path_restriction: &'a dyn Restriction,
    pub special_edge: &'a dyn Fn(EdgeIndex) -> bool,
}

/// Post-optimization procedure based on removing sections of the route and reconnecting the
/// ends without going through the previous section.
pub fn section_removal<'a, 'graph>(
    params: &mut RemovalParams<'a, 'graph>,
    removal_size: usize,
    rng: &mut impl Rng,
) -> Option<()> {
    let path = find_first_improving_path(params, removal_size, rng)?;

    local_search::path::replace(params.route, removal_size, path, params.solution_coverage);
    Some(())
}
