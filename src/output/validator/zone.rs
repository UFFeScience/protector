use std::collections::HashMap;

use anyhow::{Context, anyhow};

use crate::output::{Arc, Node, Zone};

pub fn validate(route: &[Arc], map: &HashMap<Node, Zone>) -> anyhow::Result<()> {
    let expected = match route.first() {
        Some(arc) => *map.get(&arc.0).context("Failed to get zone")?,
        None => return Ok(()),
    };

    let check_zone = |node| check(map, node, expected);
    for (source, target) in route {
        check_zone(source)?;
        check_zone(target)?;
    }
    Ok(())
}

pub fn infer_zone(route: &[Arc], map: &HashMap<Node, Zone>) -> Option<Zone> {
    route.first().and_then(|arc| map.get(&arc.0).copied())
}

pub fn route_group_should_be_from_same_zone<'a>(
    mut group: impl Iterator<Item = &'a [Arc]>,
    map: &HashMap<Node, Zone>,
) -> anyhow::Result<()> {
    let (first_route, first_zone) = match group
        .next()
        .and_then(|first| infer_zone(first, map).map(|zone| (first, zone)))
    {
        Some(first) => first,
        None => return Ok(()),
    };

    for route in group {
        if let Some(zone) = infer_zone(route, map) {
            if zone != first_zone {
                return Err(anyhow!(
                    "routes from the same group belong to different zones.\n\
                    \n\
                    Route starting with {:?} has zone {}, while the one starting with {:?} has zone {}",
                    first_route.first().unwrap(),
                    first_zone,
                    route.first().unwrap(),
                    zone
                ));
            }
        }
    }
    Ok(())
}

fn check(map: &HashMap<Node, Zone>, node: &Node, expected: Zone) -> Result<(), anyhow::Error> {
    let zone = *map.get(node).context("Failed to get zone")?;
    if zone == expected {
        Ok(())
    } else {
        Err(anyhow!(
            "Node {} has zone {}, which is different from the route zone ({}).",
            node,
            zone,
            expected
        ))
    }
}
