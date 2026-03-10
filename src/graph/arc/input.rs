use std::fmt::Display;

use super::Weight;
use crate::input::Id;

#[derive(Debug, Copy, Clone)]
pub struct InputArc {
    pub origin: Id,
    pub destiny: Id,
    pub weight: Weight,
}

impl PartialEq for InputArc {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destiny == other.destiny
    }
}

impl Eq for InputArc {}

impl std::hash::Hash for InputArc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.origin.hash(state);
        self.destiny.hash(state);
    }
}

impl InputArc {
    pub fn new(origin: Id, destiny: Id, weight: Weight) -> Self {
        Self {
            origin,
            destiny,
            weight,
        }
    }
}

impl Display for InputArc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({} -> {})", self.origin, self.destiny))
    }
}
