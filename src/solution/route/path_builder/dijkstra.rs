use std::{cmp::Reverse, collections::BinaryHeap, convert::TryFrom};

use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    visit::EdgeRef,
};

use super::{PathBuilder, Restriction};
use crate::{graph::Graph, input::Distance};

/// Builds a path using the Dijkstra's algorithm
pub struct Dijkstra;

impl PathBuilder for Dijkstra {
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
        if origin == target {
            return Some((vec![], Distance::try_from(0.0).unwrap()));
        }

        let mut previous: Vec<Option<(EdgeIndex, Distance)>> =
            vec![None; graph.inner().node_count()];

        let mut to_visit: BinaryHeap<Reverse<(Distance, NodeIndex)>> = BinaryHeap::new();
        let initial = (Distance::try_from(0.0).unwrap(), origin);
        to_visit.push(Reverse(initial));

        let mut done = vec![false; graph.inner().node_count()];

        while let Some(Reverse((node_distance, node))) = to_visit.pop() {
            if node == target {
                break;
            }

            if done[node.index()] {
                continue;
            }

            for edge in graph
                .inner()
                .edges(node)
                .filter(|edge| !done[edge.target().index()])
            {
                let neighbor = edge.target();
                let edge_distance = edge.weight().distance;
                let candidate_distance = node_distance + edge_distance;

                if restriction(edge.id(), neighbor) || candidate_distance > max_distance {
                    continue;
                }

                let previous_distance = previous[neighbor.index()].map(|(_, d)| d);

                if should_update(previous_distance, candidate_distance) {
                    previous[neighbor.index()] = Some((edge.id(), candidate_distance));
                }

                to_visit.push(Reverse((previous[neighbor.index()].unwrap().1, neighbor)));
            }

            done[node.index()] = true;
        }

        let mut inverted_path = Vec::new();
        let mut current = target;
        while let Some((edge, _distance)) = previous[current.index()] {
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

fn should_update(previous: Option<Distance>, candidate: Distance) -> bool {
    previous.filter(|&distance| distance <= candidate).is_none()
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::input::Distance;

    use super::should_update;

    fn distance(number: f64) -> Distance {
        Distance::try_from(number).unwrap()
    }

    #[test]
    fn should_update_works() {
        let cases = [
            ((None, distance(20.0)), true),
            ((Some(distance(10.0)), distance(20.0)), false),
            ((Some(distance(10.0)), distance(5.0)), true),
            ((Some(distance(10.0)), distance(10.0)), false),
        ];
        for &((previous, candidate), expected) in cases.iter() {
            assert_eq!(should_update(previous, candidate), expected);
        }
    }
}
