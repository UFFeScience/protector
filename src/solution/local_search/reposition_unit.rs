use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    visit::EdgeRef,
};

use crate::{graph::Graph, shared::Coverage, solution::constructor::greedy_fixed_unit_insertion};

/// Removes `unit` and inserts another fixed unit on the graph.
pub fn reposition_unit(
    graph: &Graph,
    solution_coverage: &mut Coverage<EdgeIndex>,
    unit: NodeIndex,
) -> NodeIndex {
    graph
        .all_edges(unit)
        .for_each(|edge| solution_coverage.uncover(edge.id()));

    greedy_fixed_unit_insertion(graph, solution_coverage)
}
