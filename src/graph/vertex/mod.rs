mod input;

pub use input::InputVertex;

use crate::input::{Id, Zone};

pub trait Vertex {
    fn id(&self) -> Id;
    fn zone(&self) -> Zone;
}

pub type Weight = InputVertex;
