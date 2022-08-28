use flexiyaml::{Mapping, Node, NodeContent, NodeHost, Scalar, Sequence};

#[test]
fn test_create_scalar() {
    let node = Node::Owned(NodeContent::Scalar(Scalar {
        value: "Hello, world!".into(),
    }));
    format!("{:?}", node);
    Scalar::default();
}

#[test]
fn test_create_seq() {
    let node = Node::Owned(NodeContent::Sequence(Sequence { elements: vec![] }));
    format!("{:?}", node);
    Sequence::default();
}

#[test]
fn test_create_map() {
    let node = Node::Owned(NodeContent::Mapping(Mapping { entries: vec![] }));
    format!("{:?}", node);
    Mapping::default();
}

#[test]
fn test_node_default() {
    assert!(matches!(NodeContent::default(), NodeContent::Scalar(_)));
    assert!(matches!(
        Node::default(),
        Node::Owned(NodeContent::Scalar(_))
    ));
}

#[test]
fn test_create_shared_node() {
    let host = NodeHost::new();
    let _ = host.clone();
    format!("{:?}", host);
    let node = host.make_shared(NodeContent::Mapping(Mapping { entries: vec![] }));
    format!("{:?}", node);
}
