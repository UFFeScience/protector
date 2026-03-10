use std::collections::HashMap;

use super::{Arc, Node, Output, Zone};
use crate::{
    Instance, Solution,
    graph::{Graph, arc, vertex::Vertex},
};

use anyhow::anyhow;
use itertools::Itertools;
use petgraph::visit::{EdgeRef, IntoNodeReferences};

mod route;
mod score;
mod vip;
mod zone;

pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}

impl Validate for Solution<'_> {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut output = Vec::<u8>::new();
        self.export(&mut output)
            .map_err(|_| anyhow!("Couldn't export the solution to the output format."))?;
        let output = Output::new(&String::from_utf8(output).unwrap())
            .map_err(|_| anyhow!("Couldn't parse the output of the solution to validate it."))?;
        validate(self.instance, &output)
    }
}

pub fn validate(instance: &Instance, output: &Output) -> Result<(), ValidationError> {
    let map = build_arc_map(instance.graph(), |weight| weight.distance);

    output
        .global_routes
        .iter()
        .try_for_each(|route| route::validate(route, &map, instance.global_distance()))?;

    output
        .zone_routes
        .iter()
        .map(|(_, route)| route)
        .try_for_each(|route| route::validate(route, &map, instance.zone_distance()))?;

    let zone_map = build_zone_map(instance.graph());
    output
        .zone_routes
        .iter()
        .try_for_each(|(_, route)| zone::validate(route, &zone_map))?;

    output
        .zone_routes
        .iter()
        .group_by(|(zone, _)| zone)
        .into_iter()
        .try_for_each(|(_, group)| {
            zone::route_group_should_be_from_same_zone(
                group.map(|(_, routes)| routes.as_slice()),
                &zone_map,
            )
        })?;

    vip::validate(instance.input_vips(), output)?;
    score::validate(output, instance.graph())?;

    output
        .global_routes
        .iter()
        .chain(output.zone_routes.iter().map(|(_, route)| route))
        .map(Vec::as_slice)
        .map(route::inform)
        .for_each(|i| i.log());

    Ok(())
}

fn build_arc_map<T>(graph: &Graph, field: impl Fn(&arc::Weight) -> T) -> HashMap<Arc, T> {
    graph
        .inner()
        .edge_references()
        .map(|edge| {
            let source = *graph[edge.source()].weight.id().as_ref() as usize;
            let target = *graph[edge.target()].weight.id().as_ref() as usize;
            ((source, target), field(edge.weight()))
        })
        .collect::<HashMap<_, _>>()
}

fn build_zone_map(graph: &Graph) -> HashMap<Node, Zone> {
    graph
        .inner()
        .node_references()
        .map(|(_, weight)| {
            (
                *weight.id().as_ref() as usize,
                *weight.zone().as_ref() as usize,
            )
        })
        .collect()
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error(transparent)]
    Route(#[from] route::Error),
    #[error(transparent)]
    Vip(#[from] vip::Error),
    #[error(transparent)]
    Score(#[from] score::Error),
    #[error("An unexpected error occurred")]
    Other(#[from] anyhow::Error),
}
