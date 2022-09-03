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
    pub style: Option<ScalarStyle>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarStyle {
    Plain,
    DoubleQuoted,
    SingleQuoted,
    Literal,
    Folded,
}

#[derive(Debug, Default)]
pub struct Sequence {
    pub elements: Vec<Node>,
    pub style: Option<SequenceStyle>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequenceStyle {
    Block,
    Flow,
}

#[derive(Debug, Default)]
pub struct Mapping {
    pub entries: Vec<(Node, Node)>,
    pub style: Option<MappingStyle>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MappingStyle {
    Block,
    Flow,
}
