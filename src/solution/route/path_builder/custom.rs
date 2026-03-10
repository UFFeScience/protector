use std::{cmp::Reverse, collections::BinaryHeap};

use petgraph::{
    EdgeDirection,
    graph::{EdgeIndex, NodeIndex},
    visit::EdgeRef,
};

use super::{PathBuilder, Restriction};
use crate::{graph::Graph, input::Distance};

/// Path builder using a custom algorithm
pub struct Custom;

impl PathBuilder for Custom {
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

        let mut queue: BinaryHeap<Reverse<(Distance, EdgeIndex)>> = BinaryHeap::new();
        for edge in graph.inner().edges(origin).map(|it| it.id()) {
            queue.push(Reverse((graph[edge].weight.distance, edge)));
        }

        while let Some(Reverse((neighbor_distance, index))) = queue.pop() {
            let edge = &graph[index];
            let neighbor = edge.target();

            // Condition required by the algorithm
            if previous[neighbor.index()].is_some() {
                continue;
            }

            // Conditions required to support the application logic
            if restriction(index, neighbor) || neighbor_distance > max_distance {
                continue;
            }

            previous[neighbor.index()] = Some((index, neighbor_distance));

            if neighbor == target {
                break;
            } else {
                for id in graph.inner().edges(neighbor).map(|it| it.id()) {
                    let edge_distance = graph[id].weight.distance;
                    queue.push(Reverse((neighbor_distance + edge_distance, id)));
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
