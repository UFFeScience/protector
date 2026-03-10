use std::fmt::Display;

use super::{Id, Vertex, Zone};

#[derive(Clone, Debug)]
pub struct InputVertex {
    id: Id,
    zone: Zone,
}

impl std::hash::Hash for InputVertex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for InputVertex {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for InputVertex {}

impl InputVertex {
    pub fn new(id: Id, zone: Zone) -> Self {
        InputVertex { id, zone }
    }
}

impl Vertex for InputVertex {
    fn id(&self) -> Id {
        self.id
    }

    fn zone(&self) -> Zone {
        self.zone
    }
}

impl Display for InputVertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Z{} - {}", self.zone, self.id))
    }
}
