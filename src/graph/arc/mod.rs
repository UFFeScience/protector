pub mod input;
pub mod weight;

pub use input::InputArc;
use petgraph::graph::EdgeIndex;
pub use weight::Weight;

use crate::{
    graph::Graph,
    shared::coverage::{Coverage, Position},
};

/// Created to reuse logic on EdgeIndex [Coverage]s due to the "reverse" case.
///
/// All these methods should take into account the reverse of `self`
pub(crate) trait Cover {
    fn is_covered(&self, coverage: &Coverage<Self>, graph: &Graph) -> bool;
}

impl Cover for EdgeIndex {
    fn is_covered(&self, coverage: &Coverage<Self>, graph: &Graph) -> bool {
        coverage.is_covered(*self)
            || graph[*self]
                .weight
                .reverse
                .filter(|reverse| coverage.is_covered(*reverse))
                .is_some()
    }
}

impl Position for EdgeIndex {
    fn position(&self) -> usize {
        self.index()
    }
}
