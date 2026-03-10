use std::collections::HashSet;

use thiserror::Error;

use super::{Distance, Input};
use crate::graph::{arc::InputArc, vertex::InputVertex};

pub fn check(input: &Input) -> Result<(), InputCheckError> {
    each_vertex_should_appear_once(&input.graph_input.vertices)?;
    each_arc_should_appear_once(&input.graph_input.arcs)?;
    arc_distance_must_be_greater_than_zero(&input.graph_input.arcs)?;
    reverse_arcs_must_have_same_attributes(&input.graph_input.arcs)?;

    if input.vips.len() != input.general.global_routes.quantity as usize {
        return Err(InputCheckError::VipsNumberDifferentFromGlobalRoutes);
    }

    Ok(())
}

fn reverse_arcs_must_have_same_attributes(arcs: &[InputArc]) -> Result<(), InputCheckError> {
    for arc in arcs {
        for candidate in arcs {
            if (arc.origin, arc.destiny) == (candidate.destiny, candidate.origin)
                && (arc.weight.crime_factor != candidate.weight.crime_factor
                    || arc.weight.distance != candidate.weight.distance)
            {
                return Err(InputCheckError::ReverseWithAttributeMismatch(*arc));
            }
        }
    }

    Ok(())
}

fn arc_distance_must_be_greater_than_zero(arcs: &[InputArc]) -> Result<(), InputCheckError> {
    for arc in arcs.iter() {
        if arc.weight.distance <= Distance::default() {
            return Err(InputCheckError::ArcWithInvalidDistance(*arc));
        }
    }

    Ok(())
}

fn each_vertex_should_appear_once(vertices: &[InputVertex]) -> Result<(), InputCheckError> {
    let mut set = HashSet::new();

    for vertex in vertices.iter() {
        if !set.insert(vertex) {
            return Err(InputCheckError::SameVertexAppearedTwice(vertex.clone()));
        }
    }

    Ok(())
}

fn each_arc_should_appear_once(arcs: &[InputArc]) -> Result<(), InputCheckError> {
    let mut set = HashSet::new();

    for arc in arcs.iter() {
        if !set.insert(arc) {
            return Err(InputCheckError::SameArcAppearedTwice(*arc));
        }
    }

    Ok(())
}

#[derive(Debug, Error)]
pub enum InputCheckError {
    #[error("Number of VIP routes is different from the number of Global Routes")]
    VipsNumberDifferentFromGlobalRoutes,
    #[error("A vertex appeared more than once on the input: {0}")]
    SameVertexAppearedTwice(InputVertex),
    #[error("An arc appeared more than once on the input: {0}")]
    SameArcAppearedTwice(InputArc),
    #[error("Arc {0} doesn't have distance greater than 0.0")]
    ArcWithInvalidDistance(InputArc),
    #[error("Arc {0} doesn't have the same attributes as its reverse")]
    ReverseWithAttributeMismatch(InputArc),
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::{each_arc_should_appear_once, each_vertex_should_appear_once};
    use crate::{
        graph::{
            arc::{InputArc, Weight},
            vertex::InputVertex,
        },
        input::{CrimeFactor, Distance, Id, Zone},
    };

    #[test]
    fn vertex_checks() {
        let vertices = vertices_vec(&[(2, 1), (3, 1), (2, 2)]);
        assert!(each_vertex_should_appear_once(&vertices).is_err());

        let vertices = vertices_vec(&[(2, 1), (3, 1), (1, 2)]);
        assert!(each_vertex_should_appear_once(&vertices).is_ok());
    }

    fn vertices_vec(vertices: &[(u32, u32)]) -> Vec<InputVertex> {
        vertices
            .iter()
            .map(|(id, zone)| InputVertex::new(Id::new(*id), Zone::new(*zone)))
            .collect()
    }

    #[test]
    fn arc_checks() {
        let arcs = arcs_vec(&[(2, 1), (3, 1), (2, 1)]);
        assert!(each_arc_should_appear_once(&arcs).is_err());

        let arcs = arcs_vec(&[(2, 1), (3, 1), (1, 2), (2, 2)]);
        assert!(each_arc_should_appear_once(&arcs).is_ok());
    }

    fn arcs_vec(arcs: &[(u32, u32)]) -> Vec<InputArc> {
        let dummy_weight = Weight::new(
            CrimeFactor::try_from(10.0).unwrap(),
            Distance::try_from(12.0).unwrap(),
        );
        arcs.iter()
            .map(|(origin, destiny)| {
                InputArc::new(Id::new(*origin), Id::new(*destiny), dummy_weight)
            })
            .collect()
    }
}
