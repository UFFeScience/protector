use petgraph::graph::EdgeIndex;

use crate::{
    graph::Graph,
    input::{CrimeFactor, Distance},
    shared::Coverage,
    solution::{contribution::addition_contribution, route::Route},
};

/// A [Path] replaces a [Section][super::section_moves::Section] of a route.
#[derive(Debug)]
pub struct Path {
    pub insert_position: usize,
    edges: Vec<EdgeIndex>,
    crime_factor: CrimeFactor,
    distance: Distance,
}

impl Path {
    /// Get a copy of path's crime factor.
    pub fn crime_factor(&self) -> CrimeFactor {
        self.crime_factor
    }

    pub fn new(
        graph: &Graph,
        insert_position: usize,
        edges: Vec<EdgeIndex>,
        distance: Distance,
        coverage: &Coverage<EdgeIndex>,
    ) -> Self {
        let crime_factor = edges
            .iter()
            .map(|edge| addition_contribution(graph, *edge, coverage))
            .sum();

        Self {
            insert_position,
            edges,
            crime_factor,
            distance,
        }
    }
}

pub fn replace(
    route: &mut Route,
    size: usize,
    new_path: Path,
    solution_coverage: &mut Coverage<EdgeIndex>,
) {
    for _ in 0..size {
        let current = route.edges.remove(new_path.insert_position);

        route.remove_coverage(current);
        solution_coverage.uncover(current);

        route.current_distance -= route.graph[current].weight.distance;
    }

    // The new path is reversed before insertion because we always insert in the initial position.
    for index in new_path.edges.into_iter().rev() {
        route.edges.insert(new_path.insert_position, index);

        route.add_coverage(index);
        solution_coverage.cover(index);
    }

    route.current_distance += new_path.distance;
}
