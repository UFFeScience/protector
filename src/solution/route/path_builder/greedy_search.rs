use std::collections::BinaryHeap;

use petgraph::{
    EdgeDirection,
    graph::{EdgeIndex, NodeIndex},
    visit::EdgeRef,
};

use super::{PathBuilder, Restriction};
use crate::{
    graph::Graph,
    input::{CrimeFactor, Distance},
    shared::Coverage,
    solution::contribution::addition_contribution,
};

/// Path builder using a greedy approach
pub struct GreedySearch<'a> {
    coverage: &'a Coverage<EdgeIndex>,
}

impl<'a> GreedySearch<'a> {
    pub fn new(coverage: &'a Coverage<EdgeIndex>) -> Self {
        Self { coverage }
    }
}

impl PathBuilder for GreedySearch<'_> {
    fn build<F>(
        &self,
        graph: &Graph,
        origin: NodeIndex,
        target: NodeIndex,
        max_distance: Distance,
        restriction: F,
    ) -> Option<(Vec<EdgeIndex>, Distance)>
    where
        F: Restriction,
    {
        let mut previous: Vec<Option<(EdgeIndex, Distance)>> =
            vec![None; graph.inner().node_count()];

        // `any_edge` is just a way to flag that `origin` is already marked.
        let any_edge = graph[origin].next_edge(EdgeDirection::Outgoing);
        previous[origin.index()] = Some((any_edge, Distance::default()));

        let mut to_visit: BinaryHeap<(CrimeFactor, NodeIndex)> = BinaryHeap::new();
        to_visit.push((CrimeFactor::default(), origin));

        'outer: while let Some((_, node)) = to_visit.pop() {
            for edge in graph.inner().edges(node) {
                let neighbor = edge.target();

                if restriction(edge.id(), neighbor) {
                    continue;
                }

                let edge_distance = edge.weight().distance;
                let node_distance = previous[node.index()].unwrap().1;
                let neighbor_distance = node_distance + edge_distance;

                if previous[neighbor.index()].is_none() && neighbor_distance <= max_distance {
                    previous[neighbor.index()] = Some((edge.id(), neighbor_distance));

                    if neighbor == target {
                        break 'outer;
                    } else {
                        let crime_factor = addition_contribution(graph, edge.id(), self.coverage);
                        to_visit.push((crime_factor, neighbor));
                    }
                }
            }
        }

        let mut inverted_path = Vec::new();
        let mut current = target;
        while let Some((edge, _distance)) = previous[current.index()] {
            if current == origin {
                break;
            }

            inverted_path.push(edge);
            current = graph[edge].source();
        }

        if inverted_path.is_empty() {
            None
        } else {
            let path = inverted_path.into_iter().rev().collect();
            let distance = previous[target.index()].unwrap().1;
            Some((path, distance))
        }
    }
}
