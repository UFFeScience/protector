use std::str::FromStr;

use anyhow::{Error, Result, anyhow};

use super::Distance;

#[derive(Debug)]
pub struct GeneralValues {
    pub vertex_count: u32,
    pub edges: u32,
    pub zones: u32,
    pub fixed_units: u32,
    pub global_routes: RouteInput,
    pub zone_routes: RouteInput,
}

impl FromStr for GeneralValues {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let segments: Vec<&str> = text.split_whitespace().collect();
        if segments.len() != 8 {
            return Err(anyhow!(
                "Number of data on first line must be 8, got {}",
                segments.len()
            ));
        }

        let global_routes = RouteInput {
            quantity: segments[4].parse()?,
            max_distance: Distance::from_str(segments[6])?,
        };

        let zone_routes = RouteInput {
            quantity: segments[5].parse()?,
            max_distance: Distance::from_str(segments[7])?,
        };

        Ok(GeneralValues {
            vertex_count: segments[0].parse()?,
            edges: segments[1].parse()?,
            zones: segments[2].parse()?,
            fixed_units: segments[3].parse()?,
            global_routes,
            zone_routes,
        })
    }
}

#[derive(Debug)]
pub struct RouteInput {
    pub quantity: u32,
    pub max_distance: Distance,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conversion() {
        let test = "64 124 2 2 2 2 1247.19 691.74";
        assert!(GeneralValues::from_str(test).is_ok());
    }

    #[test]
    fn failed_conversion() {
        let tests = ["64 124 2 2", ""];
        for test in tests.iter() {
            assert!(GeneralValues::from_str(test).is_err());
        }
    }
}
