use crate::graph::{arc::InputArc, vertex::InputVertex};

#[derive(Debug)]
pub struct GraphInput {
    pub vertices: Vec<InputVertex>,
    pub arcs: Vec<InputArc>,
}

impl GraphInput {
    pub fn new<V, A>(vertices: V, arcs: A) -> Self
    where
        V: Iterator<Item = InputVertex>,
        A: Iterator<Item = InputArc>,
    {
        let vertices = vertices.collect();
        let arcs = arcs.collect();
        Self { vertices, arcs }
    }
}
