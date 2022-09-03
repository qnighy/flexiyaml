use crate::Node;

#[derive(Debug)]
pub struct Stream {
    pub documents: Vec<Document>,
}

#[derive(Debug)]
pub struct Document {
    pub root: Node,
}
