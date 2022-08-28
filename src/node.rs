use crate::node_host::SharedNode;

#[derive(Debug)]
pub enum Node {
    Owned(NodeContent),
    Shared(SharedNode),
}

impl Default for Node {
    fn default() -> Self {
        Node::Owned(NodeContent::default())
    }
}

#[derive(Debug)]
pub enum NodeContent {
    Scalar(Scalar),
    Sequence(Sequence),
    Mapping(Mapping),
}

impl Default for NodeContent {
    fn default() -> Self {
        NodeContent::Scalar(Scalar::default())
    }
}

#[derive(Debug, Default)]
pub struct Scalar {
    pub value: String,
}

#[derive(Debug, Default)]
pub struct Sequence {
    pub elements: Vec<Node>,
}

#[derive(Debug, Default)]
pub struct Mapping {
    pub entries: Vec<(Node, Node)>,
}
