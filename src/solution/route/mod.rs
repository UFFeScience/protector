pub mod path_builder;

use std::fmt::Debug;

use petgraph::{
    EdgeDirection,
    graph::{EdgeIndex, NodeIndex},
    visit::EdgeRef,
};

use self::path_builder::{PathBuilder, Restriction};
use crate::{
    graph::Graph,
    graph::vertex::Vertex,
    input::{CrimeFactor, Distance, Id},
    shared::Coverage,
};

use super::contribution::actual_contribution;

#[derive(Clone)]
pub struct Route<'a> {
    pub edges: Vec<EdgeIndex>,
    pub(crate) current_distance: Distance,
    pub graph: &'a Graph,
    max_distance: Distance,
    pub coverage: Coverage<EdgeIndex>,
}

impl Debug for Route<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = to_text(&self.edges, self.graph);

        f.debug_struct("Route")
            .field("distance", &self.current_distance.as_ref().as_ref())
            .field("arcs", &content.trim_end())
            .finish()
    }
}

pub fn to_text(edges: &[EdgeIndex], graph: &Graph) -> String {
    edges
        .iter()
        .map(|&edge| {
            let origin = get_id(graph, graph[edge].source());
            let destiny = get_id(graph, graph[edge].target());
            format!("({}, {})", origin, destiny)
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn get_id(graph: &Graph, node: NodeIndex) -> Id {
    graph[node].weight.id()
}

impl<'a> Route<'a> {
    /// Get a reference to the route's max distance.
    pub fn max_distance(&self) -> Distance {
        self.max_distance
    }

    pub fn current_distance(&self) -> Distance {
        self.current_distance
    }

    pub fn remaining_distance(&self) -> Distance {
        self.max_distance - self.current_distance
    }

    /// Sum of the crime factor of `Self`
    pub fn crime_factor(&self, coverage: &Coverage<EdgeIndex>) -> CrimeFactor {
        self.edges
            .iter()
            .map(|&edge| actual_contribution(self, edge, coverage))
            .sum()
    }

    pub fn insert_node<F>(
        &mut self,
        target: NodeIndex,
        builder: &impl PathBuilder,
        restriction: F,
    ) -> Result<(), ()>
    where
        F: Restriction,
    {
        let last_inserted = self.edges.last().unwrap();
        let origin = self.graph[*last_inserted].target();

        let build = builder.build(
            self.graph,
            origin,
            target,
            self.max_distance - self.current_distance,
            restriction,
        );

        let (path, distance) = build.ok_or(())?;

        self.edges.extend(path.iter());

        self.current_distance += distance;

        for edge in path {
            self.add_coverage(edge);
        }

        Ok(())
    }

    pub fn insert_edge<F>(
        &mut self,
        index: EdgeIndex,
        builder: &impl PathBuilder,
        restriction: F,
    ) -> Result<(), ()>
    where
        F: Restriction,
    {
        let edge = &self.graph[index];

        // We're covering the edges that enter on the target vertex because
        // we don't want to allow cases where the path found by the route is
        // a reverse of the edge which is desired.
        for edge in self
            .graph
            .inner()
            .edges_directed(edge.target(), EdgeDirection::Incoming)
        {
            self.coverage.cover(edge.id());
        }

        // Dirty trick to ensure that the edge itself will fit in the route later
        self.current_distance += edge.weight.distance;

        let result = self.insert_node(edge.source(), builder, &restriction);

        // After inserting `edge.source()` in the route, we need to adjust
        // the current distance due to the previous trick.
        self.current_distance -= edge.weight.distance;

        // We need to uncover the edges covered previously to keep the coverage
        // in a consistent state.
        for edge in self
            .graph
            .inner()
            .edges_directed(edge.target(), EdgeDirection::Incoming)
        {
            self.coverage.uncover(edge.id());
        }

        if result.is_ok() {
            // This insertion is always successful because:
            // 1) We "reserved" distance by artificially adding it in the beginning
            // 2) If we already have `edge.source()` inserted, adding `edge.target()`
            //    would just imply in adding the `edge` itself on the route.
            let _ = self.insert_node(edge.target(), builder, restriction);

            Ok(())
        } else {
            Err(())
        }
    }

    /// Makes the route go back to the state where the `last_edge`
    /// position was the last in the route.
    pub fn reset_to(&mut self, len: usize) {
        for edge in self.edges[len..].iter() {
            self.coverage.uncover(*edge);

            let distance = self.graph[*edge].weight.distance;
            self.current_distance -= distance;
        }
        self.edges.truncate(len);
    }

    pub fn new(graph: &'a Graph, initial_arc: EdgeIndex, max_distance: Distance) -> Self {
        let arcs = vec![initial_arc];
        let mut coverage = Coverage::new(graph.inner().edge_count());

        coverage.cover(initial_arc);
        let arc_distance = graph[initial_arc].weight.distance;

        Self {
            edges: arcs,
            current_distance: arc_distance,
            max_distance,
            graph,
            coverage,
        }
    }

    /// Update coverage state based on the edge removal
    pub(crate) fn remove_coverage(&mut self, index: EdgeIndex) {
        self.coverage.uncover(index);
    }

    /// Update coverage state based on the edge insertion
    pub(crate) fn add_coverage(&mut self, index: EdgeIndex) {
        self.coverage.cover(index);
    }

    pub fn len(&self) -> usize {
        self.edges.len()
    }
}
