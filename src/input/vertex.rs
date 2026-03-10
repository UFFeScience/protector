use std::str::FromStr;

use anyhow::{Error, Result, anyhow};

use crate::{
    graph::vertex::InputVertex,
    input::{Id, Zone},
};

impl FromStr for InputVertex {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Vec<&str> = s.split_whitespace().collect();
        if segments.len() != 2 {
            return Err(anyhow!("Number of data on a Vertex line not expected"));
        }
        let id: Id = segments[0].parse()?;
        let zone: Zone = segments[1].parse()?;
        Ok(InputVertex::new(id, zone))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conversion() {
        let tests = ["384 1", "374 2", "2 5"];
        for instance in tests.iter() {
            let result = InputVertex::from_str(instance);
            assert!(result.is_ok());
        }
    }
}
