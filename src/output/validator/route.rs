use std::collections::HashMap;

use log::debug;

use super::Arc;
use crate::{input::Distance, output::Node};

pub struct Inform {
    pub origin: Vec<(Node, usize)>,
    pub target: Vec<(Node, usize)>,
    pub arc: Vec<(Arc, usize)>,
}

impl Inform {
    pub fn log(&self) {
        for (vertex, apparitions) in &self.origin {
            debug!("Vertex {} appeared as origin {} times", vertex, apparitions);
        }

        for (vertex, apparitions) in &self.target {
            debug!("Vertex {} appeared as target {} times", vertex, apparitions);
        }

        for (arc, apparitions) in &self.arc {
            debug!("Arc {:?} appeared {} times", arc, apparitions);
        }
    }
}

/// Log information about repeated arcs and vertices in the route.
pub fn inform(route: &[Arc]) -> Inform {
    let mut origin_map = HashMap::new();
    let mut target_map = HashMap::new();
    let mut arc_map = HashMap::new();

    for arc in route {
        *origin_map.entry(arc.0).or_insert(0) += 1;
        *target_map.entry(arc.1).or_insert(0) += 1;

        *arc_map.entry(*arc).or_insert(0) += 1;
    }

    // Origin must always repeat at least twice, so we'll skip it.
    let return_point = route[0].0;

    let origin = origin_map
        .into_iter()
        .filter(|&(vertex, apparitions)| apparitions > 1 && vertex != return_point)
        .collect();

    let target = target_map
        .into_iter()
        .filter(|&(vertex, apparitions)| apparitions > 1 && vertex != return_point)
        .collect();

    let arc = arc_map
        .into_iter()
        .filter(|&(_, apparitions)| apparitions > 1)
        .collect();

    Inform {
        origin,
        target,
        arc,
    }
}

pub fn validate(route: &[Arc], map: &HashMap<Arc, Distance>, max: Distance) -> Result<(), Error> {
    check_sequence(route)?;
    check_distance(route, map, max)
}

fn check_sequence(route: &[Arc]) -> Result<(), Error> {
    if route.is_empty() {
        return Ok(());
    }
    for (first, second) in route.windows(2).map(|it| (it[0], it[1])) {
        if second.0 != first.1 {
            return Err(Error::NotChained(first, second));
        }
    }
    let first = route.first().unwrap();
    let last = route.last().unwrap();
    if last.1 != first.0 {
        return Err(Error::NotClosed);
    }
    Ok(())
}

fn check_distance(route: &[Arc], map: &HashMap<Arc, Distance>, max: Distance) -> Result<(), Error> {
    let total_distance: Distance = route
        .iter()
        .map(|arc| match map.get(arc) {
            Some(&distance) => Ok(distance),
            None => Err(Error::MissingDistance(*arc)),
        })
        .sum::<Result<_, _>>()?;

    let diff = total_distance.as_ref().as_ref() - max.as_ref().as_ref();
    let margin = 0.01;

    if diff > margin {
        Err(Error::MaxDistanceExceeded {
            maximum: max,
            actual: total_distance,
        })
    } else {
        Ok(())
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("Route is not a chained sequence: arc {0:?} is not connected with {1:?}")]
    NotChained(Arc, Arc),
    #[error("The last arc is not connected with the first one")]
    NotClosed,
    #[error("Route has distance of {actual}, but the maximum is {maximum}")]
    MaxDistanceExceeded { maximum: Distance, actual: Distance },
    #[error("Couldn't find distance for {0:?} in the graph")]
    MissingDistance(Arc),
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::*;

    fn test_route(route: &[Arc], expected: Result<(), Error>) {
        assert_eq!(check_sequence(route), expected)
    }

    #[test]
    fn check_sequence_works() {
        let normal_route = vec![(1, 2), (2, 3), (3, 1)];
        test_route(&normal_route, Ok(()));

        test_route(&[], Ok(()));

        let not_chained = vec![(1, 2), (3, 4), (4, 1)];
        test_route(&not_chained, Err(Error::NotChained((1, 2), (3, 4))));

        test_route(&[(1, 2), (2, 3)], Err(Error::NotClosed));
        test_route(&[(1, 2)], Err(Error::NotClosed));
    }

    fn distance(number: f64) -> Distance {
        Distance::try_from(number).unwrap()
    }

    fn distance_map(input: Vec<(Arc, f64)>) -> HashMap<Arc, Distance> {
        input
            .into_iter()
            .map(|(arc, number)| (arc, distance(number)))
            .collect()
    }

    #[test]
    fn check_distance_works() {
        let map = distance_map(vec![((1, 2), 1.0), ((2, 3), 2.0), ((3, 1), 1.0)]);
        let route = map.keys().copied().collect::<Vec<_>>();
        let result = check_distance(&route, &map, distance(5.0));
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn check_distance_recognizes_exceeded() {
        let map = distance_map(vec![((1, 2), 3.0), ((2, 3), 4.0), ((3, 1), 3.0)]);
        let route = map.keys().copied().collect::<Vec<_>>();
        let result = check_distance(&route, &map, distance(5.0));

        let expected = Err(Error::MaxDistanceExceeded {
            maximum: distance(5.0),
            actual: distance(10.0),
        });
        assert_eq!(result, expected);
    }
}
