use std::fmt::Display;

use petgraph::graph::EdgeIndex;

use crate::input::{CrimeFactor, Distance};

#[derive(Debug, Copy, Clone)]
pub struct Weight {
    pub crime_factor: CrimeFactor,
    pub distance: Distance,
    pub reverse: Option<EdgeIndex>,
}

impl Display for Weight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cf={}\nd={}", self.crime_factor, self.distance)
    }
}

impl Weight {
    pub fn new(crime_factor: CrimeFactor, distance: Distance) -> Self {
        Self {
            crime_factor,
            distance,
            reverse: None,
        }
    }
}
