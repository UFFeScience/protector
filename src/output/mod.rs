mod parser;
mod validator;

pub use parser::Output;
pub use validator::{Validate, validate};

type Node = usize;
type Arc = (Node, Node);
type Route = Vec<Arc>;
type Zone = usize;
