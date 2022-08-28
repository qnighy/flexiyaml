use std::sync::{Arc, Mutex, RwLock, Weak};

use crate::node::NodeContent;

#[derive(Debug, Clone)]
pub struct NodeHost {
    inner: Arc<Mutex<NodeHostInner>>,
}

impl NodeHost {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(NodeHostInner::new())),
        }
    }

    pub fn make_shared(&self, content: NodeContent) -> SharedNode {
        let node_strong = Arc::new(RwLock::new(content));
        let node = Arc::downgrade(&node_strong);
        {
            let mut inner = self.inner.lock().unwrap();
            inner.nodes.push(node_strong);
        }
        SharedNode {
            node,
            host: Arc::downgrade(&self.inner),
        }
    }
}

#[derive(Debug)]
struct NodeHostInner {
    nodes: Vec<Arc<RwLock<NodeContent>>>,
}

impl NodeHostInner {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }
}

#[derive(Debug)]
pub struct SharedNode {
    node: Weak<RwLock<NodeContent>>,
    host: Weak<Mutex<NodeHostInner>>,
}
