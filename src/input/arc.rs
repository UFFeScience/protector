use std::str::FromStr;

use anyhow::{Error, anyhow};

use crate::{
    graph::arc::{Weight, input::InputArc},
    input::{CrimeFactor, Distance, Id},
};

impl FromStr for InputArc {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Vec<&str> = s.split_whitespace().collect();
        if segments.len() != 4 {
            return Err(anyhow!("Number of data of an Arc line not expected"));
        }
        let origin: Id = segments[0].parse()?;
        let destiny: Id = segments[1].parse()?;
        let crime_factor = CrimeFactor::from_str(segments[2])?;
        let distance = Distance::from_str(segments[3])?;
        let weight = Weight::new(crime_factor, distance);
        Ok(InputArc::new(origin, destiny, weight))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::arc::input::InputArc;

    #[test]
    fn conversion() {
        let tests = [
            "328 343 12.75 119.44",
            "329 313 4.14 108.51",
            "341 350 4.67 90.31",
            "343 358 8.2 119.44",
        ];
        for instance in tests.iter() {
            let result = InputArc::from_str(instance);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn failed_conversion() {
        let tests = ["328 343 12.75", "313 4.14", ""];
        for instance in tests.iter() {
            let result = InputArc::from_str(instance);
            assert!(result.is_err());
        }
    }
}
