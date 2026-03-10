//! # Path Builder
//!
//! A type which implements [PathBuilder] can create a path between a pair or vertices.
//!
//!
mod custom;
mod dijkstra;
mod greedy_search;

use petgraph::graph::{EdgeIndex, NodeIndex};

use crate::{graph::Graph, input::Distance};

pub trait Restriction: Fn(EdgeIndex, NodeIndex) -> bool {}

impl<T> Restriction for T where T: Fn(EdgeIndex, NodeIndex) -> bool {}

/// TODO remover anotações abaixo
///
/// ## coisas necessárias para construir um caminho:
/// - o grafo (acesso aos nós e arestas)
/// - cobertura da rota
/// - origem e destino
/// - uma restrição na hora de ir fazendo a busca (opcional)
/// - a distância máxima permitida para esse trecho
///
pub trait PathBuilder {
    fn build<F>(
        &self,
        graph: &Graph,
        origin: NodeIndex,
        target: NodeIndex,
        max_distance: Distance,
        restriction: F,
    ) -> Option<(Vec<EdgeIndex>, Distance)>
    where
        F: Restriction;
}

pub use custom::Custom;
pub use dijkstra::Dijkstra;
pub use greedy_search::GreedySearch;
