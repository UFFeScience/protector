pub mod arc;
pub mod vertex;
pub mod vip;

use std::{iter::Iterator, ops::Index};

use anyhow::{Context, Result, anyhow};
use petgraph::{
    Direction,
    graph::{DiGraph, Edge, EdgeIndex, EdgeReference, Node, NodeIndex},
    visit::{EdgeRef, IntoNodeReferences},
};
use vertex::Vertex;

use crate::input::{GraphInput, Id};

pub type GraphData = DiGraph<vertex::Weight, arc::Weight>;

#[derive(Debug)]
pub struct Graph {
    graph: GraphData,
}

impl Graph {
    #[allow(clippy::needless_collect)] // False Positive
    pub fn new(input: GraphInput) -> Result<Self> {
        let mut graph = GraphData::with_capacity(input.vertices.len(), input.arcs.len());

        for vertex in input.vertices.into_iter() {
            graph.add_node(vertex);
        }

        for arc in input.arcs.into_iter() {
            let origin = graph
                .node_indices()
                .find(|v| graph[*v].id() == arc.origin)
                .ok_or_else(|| anyhow!("Arc {}: origin node missing", arc))?;
            let destiny = graph
                .node_indices()
                .find(|v| graph[*v].id() == arc.destiny)
                .ok_or_else(|| anyhow!("Arc {}: destiny node missing", arc))?;

            graph.add_edge(origin, destiny, arc.weight);
        }

        let pairs = graph
            .edge_indices()
            .map(|edge| {
                let reverse = graph
                    .edge_indices()
                    .find(|other| is_reverse(get_edge(&graph, edge), get_edge(&graph, *other)));
                (edge, reverse)
            })
            .filter_map(|(edge, reverse)| Some((edge, reverse?)))
            .collect::<Vec<_>>();

        for (edge, reverse) in pairs.into_iter() {
            graph[edge].reverse = Some(reverse);
        }

        Ok(Self { graph })
    }

    pub fn inner(&self) -> &GraphData {
        &self.graph
    }

    pub fn all_edges(&self, node: NodeIndex) -> impl Iterator<Item = EdgeReference<'_, arc::Weight>> {
        self.graph
            .edges_directed(node, Direction::Outgoing)
            .chain(self.graph.edges_directed(node, Direction::Incoming))
    }

    pub fn find_edge(&self, origin: &Id, destiny: &Id) -> Result<EdgeIndex> {
        let node = self
            .find_node(origin)
            .context(format!("While looking for arc ({}, {})", origin, destiny))?;

        for edge in self.inner().edges(node) {
            if destiny == &self[edge.target()].weight.id() {
                return Ok(edge.id());
            }
        }

        Err(anyhow!(
            "Arc ({}, {}) not found in the graph",
            origin,
            destiny
        ))
    }

    pub fn find_node(&self, desired: &Id) -> Result<NodeIndex> {
        for (index, weight) in self.inner().node_references() {
            if weight.id() == *desired {
                return Ok(index);
            }
        }
        Err(anyhow!("Node ({}) not found in the graph", desired))
    }
}

fn is_reverse<T>(a: &Edge<T>, b: &Edge<T>) -> bool {
    a.source() == b.target() && a.target() == b.source()
}

#[inline]
fn get_edge(graph: &GraphData, index: EdgeIndex) -> &Edge<arc::Weight> {
    &graph.raw_edges()[index.index()]
}

impl Index<EdgeIndex> for Graph {
    type Output = Edge<arc::Weight>;

    #[inline]
    fn index(&self, index: EdgeIndex) -> &Self::Output {
        get_edge(&self.graph, index)
    }
}

impl Index<NodeIndex> for Graph {
    type Output = Node<vertex::Weight>;

    #[inline]
    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self.graph.raw_nodes()[index.index()]
    }
}
