use super::Arc;
use crate::{
    Output, graph::Graph, input::Id, output::Node, shared::Coverage, solution::compute_score,
};

use log::debug;
use ordered_float::NotNan;
use petgraph::{graph::EdgeIndex, visit::EdgeRef};

pub fn validate(output: &Output, graph: &Graph) -> Result<(), Error> {
    let mut coverage = Coverage::<EdgeIndex>::new(graph.inner().edge_count());

    cover_nodes(&output.fixed_units, graph, &mut coverage)?;
    cover_edges(output, graph, &mut coverage)?;

    let calculated = compute_score(graph, &coverage);
    debug!("Score calculated: {:.2}", calculated);
    let received = NotNan::new(output.score).expect("Output score should not be NaN!");

    compare_scores(calculated, received)
}

fn cover_nodes(
    fixed_units: &[Node],
    graph: &Graph,
    coverage: &mut Coverage<EdgeIndex>,
) -> Result<(), Error> {
    for unit in fixed_units.iter().map(|&unit| Id::new(unit as u32)) {
        let node = graph
            .find_node(&unit)
            .map_err(|_| Error::MissingNode(unit))?;
        graph
            .all_edges(node)
            .for_each(|edge| coverage.cover(edge.id()));
    }
    Ok(())
}

fn cover_edges(
    output: &Output,
    graph: &Graph,
    coverage: &mut Coverage<EdgeIndex>,
) -> Result<(), Error> {
    output
        .global_routes
        .iter()
        .chain(output.zone_routes.iter().map(|(_, route)| route))
        .flatten()
        .copied()
        .map(|(origin, destiny)| {
            graph
                .find_edge(&Id::new(origin as u32), &Id::new(destiny as u32))
                .map_err(|_| Error::MissingEdge((origin, destiny)))
        })
        .try_for_each(|result| result.map(|edge| coverage.cover(edge)))
}

fn compare_scores(calculated: NotNan<f64>, received: NotNan<f64>) -> Result<(), Error> {
    let abs_diff = (calculated - received).as_ref().abs();
    if abs_diff <= 0.1 {
        Ok(())
    } else {
        Err(Error::ScoreMismatch {
            received,
            calculated,
        })
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("The score on the output is {}, but the value calculated is {}", .received.as_ref(), .calculated.as_ref())]
    ScoreMismatch {
        calculated: NotNan<f64>,
        received: NotNan<f64>,
    },
    #[error("Couldn't find edge for {0:?} in the graph")]
    MissingEdge(Arc),
    #[error("Couldn't find node {0:?} in the graph")]
    MissingNode(Id),
}
