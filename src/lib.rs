pub use crate::node::{Mapping, Node, NodeContent, Scalar, Sequence};
pub use crate::node_host::{NodeHost, SharedNode};

mod node;
mod node_host;

pub fn parse(buf: &[u8]) -> Node {
    Node::default()
}
