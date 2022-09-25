use flexiyaml::{parse, MappingStyle, Node, NodeContent, SequenceStyle};
use std::fmt::Write;
use testdata::{pending, TestFile};

#[testdata::files(rebuild = "tests/test_suite.rs")]
#[test]
fn test_error(#[glob = "yaml-test-suite/**/*/==="] _name: &TestFile) {
    // TODO
}

#[testdata::files(rebuild = "tests/test_suite.rs")]
#[test]
fn test_events(
    #[glob = "yaml-test-suite/**/*/in.yaml"] in_yaml: &TestFile,
    #[glob = "yaml-test-suite/**/*/test.event"] test_event: &TestFile,
    #[glob = "test-suite-annex/**/*/pending.txt"] pending_file: &TestFile,
) {
    pending(pending_file, || {
        let indata = in_yaml.raw_read();
        let log_expected = test_event.raw_read();
        let log_expected = String::from_utf8(log_expected).unwrap();

        let stream = parse(&indata);
        let mut log = String::new();
        log.push_str("+STR\n");
        for document in stream.documents {
            log.push_str("+DOC\n");
            node_events(&document.root, &mut log);
            log.push_str("-DOC\n");
        }
        log.push_str("-STR\n");

        fn node_events(node: &Node, log: &mut String) {
            match node {
                Node::Owned(node) => match node {
                    NodeContent::Mapping(node) => {
                        if matches!(node.style, Some(MappingStyle::Flow)) {
                            log.push_str("+MAP {}\n");
                        } else {
                            log.push_str("+MAP\n");
                        }
                        for entry in &node.entries {
                            node_events(&entry.0, log);
                            node_events(&entry.1, log);
                        }
                        log.push_str("-MAP\n");
                    }
                    NodeContent::Sequence(node) => {
                        if matches!(node.style, Some(SequenceStyle::Flow)) {
                            log.push_str("+SEQ []\n");
                        } else {
                            log.push_str("+SEQ\n");
                        }
                        for elem in &node.elements {
                            node_events(elem, log);
                        }
                        log.push_str("-SEQ\n");
                    }
                    NodeContent::Scalar(node) => {
                        writeln!(log, "=VAL :{}", node.value).unwrap();
                    }
                },
                Node::Shared(_) => todo!("Node::Shared in test_events"),
            }
        }

        assert_eq!(log, log_expected);
    });
}

#[testdata::files(rebuild = "tests/test_suite.rs")]
#[test]
fn test_json(#[glob = "yaml-test-suite/**/*/==="] _name: &TestFile) {
    // TODO
}

#[testdata::files(rebuild = "tests/test_suite.rs")]
#[test]
fn test_out(#[glob = "yaml-test-suite/**/*/==="] _name: &TestFile) {
    // TODO
}

#[testdata::files(rebuild = "tests/test_suite.rs")]
#[test]
fn test_emit(#[glob = "yaml-test-suite/**/*/==="] _name: &TestFile) {
    // TODO
}
