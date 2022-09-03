use flexiyaml::{parse, Node, NodeContent};
use std::fmt::Write;
use std::fs;
use std::path::Path;

fn test_error(path: &str) {
    todo!("test_error");
}

fn test_events(path: &str) {
    let path = Path::new(path);
    let indata = fs::read(path.join("in.yaml")).unwrap();
    let log_expected = fs::read(path.join("test.event")).unwrap();
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
                    log.push_str("+MAP\n");
                    for entry in &node.entries {
                        node_events(&entry.0, log);
                        node_events(&entry.1, log);
                    }
                    log.push_str("-MAP\n");
                }
                NodeContent::Sequence(node) => {
                    log.push_str("+SEQ\n");
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
}

fn test_json(path: &str) {
    todo!("test_json");
}

fn test_out(path: &str) {
    todo!("test_out");
}

fn test_emit(path: &str) {
    todo!("test_emit");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_229Q_Spec_Example_2_4_Sequence_of_Mappings_events() {
    test_events("./yaml-test-suite/229Q");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_229Q_Spec_Example_2_4_Sequence_of_Mappings_json() {
    test_json("./yaml-test-suite/229Q");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_229Q_Spec_Example_2_4_Sequence_of_Mappings_out() {
    test_out("./yaml-test-suite/229Q");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_229Q_Spec_Example_2_4_Sequence_of_Mappings_emit() {
    test_emit("./yaml-test-suite/229Q");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_236B_Invalid_value_after_mapping_error() {
    test_error("./yaml-test-suite/236B");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_236B_Invalid_value_after_mapping_events() {
    test_events("./yaml-test-suite/236B");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_26DV_Whitespace_around_colon_in_mappings_events() {
    test_events("./yaml-test-suite/26DV");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_26DV_Whitespace_around_colon_in_mappings_json() {
    test_json("./yaml-test-suite/26DV");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_26DV_Whitespace_around_colon_in_mappings_out() {
    test_out("./yaml-test-suite/26DV");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_26DV_Whitespace_around_colon_in_mappings_emit() {
    test_emit("./yaml-test-suite/26DV");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_27NA_Spec_Example_5_9_Directive_Indicator_events() {
    test_events("./yaml-test-suite/27NA");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_27NA_Spec_Example_5_9_Directive_Indicator_json() {
    test_json("./yaml-test-suite/27NA");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_27NA_Spec_Example_5_9_Directive_Indicator_out() {
    test_out("./yaml-test-suite/27NA");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_27NA_Spec_Example_5_9_Directive_Indicator_emit() {
    test_emit("./yaml-test-suite/27NA");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2AUY_Tags_in_Block_Sequence_events() {
    test_events("./yaml-test-suite/2AUY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2AUY_Tags_in_Block_Sequence_json() {
    test_json("./yaml-test-suite/2AUY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2AUY_Tags_in_Block_Sequence_out() {
    test_out("./yaml-test-suite/2AUY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2AUY_Tags_in_Block_Sequence_emit() {
    test_emit("./yaml-test-suite/2AUY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2CMS_Invalid_mapping_in_plain_multiline_error() {
    test_error("./yaml-test-suite/2CMS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2CMS_Invalid_mapping_in_plain_multiline_events() {
    test_events("./yaml-test-suite/2CMS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2EBW_Allowed_characters_in_keys_events() {
    test_events("./yaml-test-suite/2EBW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2EBW_Allowed_characters_in_keys_json() {
    test_json("./yaml-test-suite/2EBW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2EBW_Allowed_characters_in_keys_out() {
    test_out("./yaml-test-suite/2EBW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2EBW_Allowed_characters_in_keys_emit() {
    test_emit("./yaml-test-suite/2EBW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2G84_00_Literal_modifers_error() {
    test_error("./yaml-test-suite/2G84/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2G84_00_Literal_modifers_events() {
    test_events("./yaml-test-suite/2G84/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2G84_01_Literal_modifers_error() {
    test_error("./yaml-test-suite/2G84/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2G84_01_Literal_modifers_events() {
    test_events("./yaml-test-suite/2G84/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2G84_02_Literal_modifers_events() {
    test_events("./yaml-test-suite/2G84/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2G84_02_Literal_modifers_json() {
    test_json("./yaml-test-suite/2G84/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2G84_02_Literal_modifers_out() {
    test_out("./yaml-test-suite/2G84/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2G84_02_Literal_modifers_emit() {
    test_emit("./yaml-test-suite/2G84/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2G84_03_Literal_modifers_events() {
    test_events("./yaml-test-suite/2G84/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2G84_03_Literal_modifers_json() {
    test_json("./yaml-test-suite/2G84/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2G84_03_Literal_modifers_out() {
    test_out("./yaml-test-suite/2G84/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2G84_03_Literal_modifers_emit() {
    test_emit("./yaml-test-suite/2G84/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2JQS_Block_Mapping_with_Missing_Keys_events() {
    test_events("./yaml-test-suite/2JQS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2JQS_Block_Mapping_with_Missing_Keys_json() {
    test_json("./yaml-test-suite/2JQS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2JQS_Block_Mapping_with_Missing_Keys_out() {
    test_out("./yaml-test-suite/2JQS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2JQS_Block_Mapping_with_Missing_Keys_emit() {
    test_emit("./yaml-test-suite/2JQS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2LFX_Spec_Example_6_13_Reserved_Directives_1_3_events() {
    test_events("./yaml-test-suite/2LFX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2LFX_Spec_Example_6_13_Reserved_Directives_1_3_json() {
    test_json("./yaml-test-suite/2LFX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2LFX_Spec_Example_6_13_Reserved_Directives_1_3_out() {
    test_out("./yaml-test-suite/2LFX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2LFX_Spec_Example_6_13_Reserved_Directives_1_3_emit() {
    test_emit("./yaml-test-suite/2LFX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2SXE_Anchors_With_Colon_in_Name_events() {
    test_events("./yaml-test-suite/2SXE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2SXE_Anchors_With_Colon_in_Name_json() {
    test_json("./yaml-test-suite/2SXE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2SXE_Anchors_With_Colon_in_Name_out() {
    test_out("./yaml-test-suite/2SXE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2SXE_Anchors_With_Colon_in_Name_emit() {
    test_emit("./yaml-test-suite/2SXE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2XXW_Spec_Example_2_25_Unordered_Sets_events() {
    test_events("./yaml-test-suite/2XXW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2XXW_Spec_Example_2_25_Unordered_Sets_json() {
    test_json("./yaml-test-suite/2XXW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2XXW_Spec_Example_2_25_Unordered_Sets_out() {
    test_out("./yaml-test-suite/2XXW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_2XXW_Spec_Example_2_25_Unordered_Sets_emit() {
    test_emit("./yaml-test-suite/2XXW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_33X3_Three_explicit_integers_in_a_block_sequence_events() {
    test_events("./yaml-test-suite/33X3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_33X3_Three_explicit_integers_in_a_block_sequence_json() {
    test_json("./yaml-test-suite/33X3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_33X3_Three_explicit_integers_in_a_block_sequence_out() {
    test_out("./yaml-test-suite/33X3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_33X3_Three_explicit_integers_in_a_block_sequence_emit() {
    test_emit("./yaml-test-suite/33X3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_35KP_Tags_for_Root_Objects_events() {
    test_events("./yaml-test-suite/35KP");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_35KP_Tags_for_Root_Objects_json() {
    test_json("./yaml-test-suite/35KP");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_35KP_Tags_for_Root_Objects_out() {
    test_out("./yaml-test-suite/35KP");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_35KP_Tags_for_Root_Objects_emit() {
    test_emit("./yaml-test-suite/35KP");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_36F6_Multiline_plain_scalar_with_empty_line_events() {
    test_events("./yaml-test-suite/36F6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_36F6_Multiline_plain_scalar_with_empty_line_json() {
    test_json("./yaml-test-suite/36F6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_36F6_Multiline_plain_scalar_with_empty_line_out() {
    test_out("./yaml-test-suite/36F6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_36F6_Multiline_plain_scalar_with_empty_line_emit() {
    test_emit("./yaml-test-suite/36F6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3ALJ_Block_Sequence_in_Block_Sequence_events() {
    test_events("./yaml-test-suite/3ALJ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3ALJ_Block_Sequence_in_Block_Sequence_json() {
    test_json("./yaml-test-suite/3ALJ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3ALJ_Block_Sequence_in_Block_Sequence_out() {
    test_out("./yaml-test-suite/3ALJ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3ALJ_Block_Sequence_in_Block_Sequence_emit() {
    test_emit("./yaml-test-suite/3ALJ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3GZX_Spec_Example_7_1_Alias_Nodes_events() {
    test_events("./yaml-test-suite/3GZX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3GZX_Spec_Example_7_1_Alias_Nodes_json() {
    test_json("./yaml-test-suite/3GZX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3GZX_Spec_Example_7_1_Alias_Nodes_out() {
    test_out("./yaml-test-suite/3GZX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3GZX_Spec_Example_7_1_Alias_Nodes_emit() {
    test_emit("./yaml-test-suite/3GZX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3HFZ_Invalid_content_after_document_end_marker_error() {
    test_error("./yaml-test-suite/3HFZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3HFZ_Invalid_content_after_document_end_marker_events() {
    test_events("./yaml-test-suite/3HFZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3MYT_Plain_Scalar_looking_like_key_comment_anchor_and_tag_events() {
    test_events("./yaml-test-suite/3MYT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3MYT_Plain_Scalar_looking_like_key_comment_anchor_and_tag_json() {
    test_json("./yaml-test-suite/3MYT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3MYT_Plain_Scalar_looking_like_key_comment_anchor_and_tag_out() {
    test_out("./yaml-test-suite/3MYT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3MYT_Plain_Scalar_looking_like_key_comment_anchor_and_tag_emit() {
    test_emit("./yaml-test-suite/3MYT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3R3P_Single_block_sequence_with_anchor_events() {
    test_events("./yaml-test-suite/3R3P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3R3P_Single_block_sequence_with_anchor_json() {
    test_json("./yaml-test-suite/3R3P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3R3P_Single_block_sequence_with_anchor_out() {
    test_out("./yaml-test-suite/3R3P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3R3P_Single_block_sequence_with_anchor_emit() {
    test_emit("./yaml-test-suite/3R3P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_00_Leading_tabs_in_double_quoted_events() {
    test_events("./yaml-test-suite/3RLN/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_00_Leading_tabs_in_double_quoted_json() {
    test_json("./yaml-test-suite/3RLN/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_00_Leading_tabs_in_double_quoted_out() {
    test_out("./yaml-test-suite/3RLN/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_00_Leading_tabs_in_double_quoted_emit() {
    test_emit("./yaml-test-suite/3RLN/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_01_Leading_tabs_in_double_quoted_events() {
    test_events("./yaml-test-suite/3RLN/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_01_Leading_tabs_in_double_quoted_json() {
    test_json("./yaml-test-suite/3RLN/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_01_Leading_tabs_in_double_quoted_out() {
    test_out("./yaml-test-suite/3RLN/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_01_Leading_tabs_in_double_quoted_emit() {
    test_emit("./yaml-test-suite/3RLN/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_02_Leading_tabs_in_double_quoted_events() {
    test_events("./yaml-test-suite/3RLN/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_02_Leading_tabs_in_double_quoted_json() {
    test_json("./yaml-test-suite/3RLN/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_02_Leading_tabs_in_double_quoted_out() {
    test_out("./yaml-test-suite/3RLN/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_02_Leading_tabs_in_double_quoted_emit() {
    test_emit("./yaml-test-suite/3RLN/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_03_Leading_tabs_in_double_quoted_events() {
    test_events("./yaml-test-suite/3RLN/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_03_Leading_tabs_in_double_quoted_json() {
    test_json("./yaml-test-suite/3RLN/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_03_Leading_tabs_in_double_quoted_out() {
    test_out("./yaml-test-suite/3RLN/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_03_Leading_tabs_in_double_quoted_emit() {
    test_emit("./yaml-test-suite/3RLN/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_04_Leading_tabs_in_double_quoted_events() {
    test_events("./yaml-test-suite/3RLN/04");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_04_Leading_tabs_in_double_quoted_json() {
    test_json("./yaml-test-suite/3RLN/04");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_04_Leading_tabs_in_double_quoted_out() {
    test_out("./yaml-test-suite/3RLN/04");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_04_Leading_tabs_in_double_quoted_emit() {
    test_emit("./yaml-test-suite/3RLN/04");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_05_Leading_tabs_in_double_quoted_events() {
    test_events("./yaml-test-suite/3RLN/05");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_05_Leading_tabs_in_double_quoted_json() {
    test_json("./yaml-test-suite/3RLN/05");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_05_Leading_tabs_in_double_quoted_out() {
    test_out("./yaml-test-suite/3RLN/05");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3RLN_05_Leading_tabs_in_double_quoted_emit() {
    test_emit("./yaml-test-suite/3RLN/05");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3UYS_Escaped_slash_in_double_quotes_events() {
    test_events("./yaml-test-suite/3UYS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3UYS_Escaped_slash_in_double_quotes_json() {
    test_json("./yaml-test-suite/3UYS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3UYS_Escaped_slash_in_double_quotes_out() {
    test_out("./yaml-test-suite/3UYS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_3UYS_Escaped_slash_in_double_quotes_emit() {
    test_emit("./yaml-test-suite/3UYS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4ABK_Flow_Mapping_Separate_Values_events() {
    test_events("./yaml-test-suite/4ABK");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4ABK_Flow_Mapping_Separate_Values_json() {
    test_json("./yaml-test-suite/4ABK");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4ABK_Flow_Mapping_Separate_Values_out() {
    test_out("./yaml-test-suite/4ABK");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4ABK_Flow_Mapping_Separate_Values_emit() {
    test_emit("./yaml-test-suite/4ABK");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4CQQ_Spec_Example_2_18_Multi_line_Flow_Scalars_events() {
    test_events("./yaml-test-suite/4CQQ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4CQQ_Spec_Example_2_18_Multi_line_Flow_Scalars_json() {
    test_json("./yaml-test-suite/4CQQ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4CQQ_Spec_Example_2_18_Multi_line_Flow_Scalars_out() {
    test_out("./yaml-test-suite/4CQQ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4CQQ_Spec_Example_2_18_Multi_line_Flow_Scalars_emit() {
    test_emit("./yaml-test-suite/4CQQ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4EJS_Invalid_tabs_as_indendation_in_a_mapping_error() {
    test_error("./yaml-test-suite/4EJS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4EJS_Invalid_tabs_as_indendation_in_a_mapping_events() {
    test_events("./yaml-test-suite/4EJS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4FJ6_Nested_implicit_complex_keys_events() {
    test_events("./yaml-test-suite/4FJ6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4FJ6_Nested_implicit_complex_keys_json() {
    test_json("./yaml-test-suite/4FJ6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4FJ6_Nested_implicit_complex_keys_out() {
    test_out("./yaml-test-suite/4FJ6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4FJ6_Nested_implicit_complex_keys_emit() {
    test_emit("./yaml-test-suite/4FJ6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4GC6_Spec_Example_7_7_Single_Quoted_Characters_events() {
    test_events("./yaml-test-suite/4GC6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4GC6_Spec_Example_7_7_Single_Quoted_Characters_json() {
    test_json("./yaml-test-suite/4GC6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4GC6_Spec_Example_7_7_Single_Quoted_Characters_out() {
    test_out("./yaml-test-suite/4GC6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4GC6_Spec_Example_7_7_Single_Quoted_Characters_emit() {
    test_emit("./yaml-test-suite/4GC6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4H7K_Flow_sequence_with_invalid_extra_closing_bracket_error() {
    test_error("./yaml-test-suite/4H7K");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4H7K_Flow_sequence_with_invalid_extra_closing_bracket_events() {
    test_events("./yaml-test-suite/4H7K");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4HVU_Wrong_indendation_in_Sequence_error() {
    test_error("./yaml-test-suite/4HVU");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4HVU_Wrong_indendation_in_Sequence_events() {
    test_events("./yaml-test-suite/4HVU");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4JVG_Scalar_value_with_two_anchors_error() {
    test_error("./yaml-test-suite/4JVG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4JVG_Scalar_value_with_two_anchors_events() {
    test_events("./yaml-test-suite/4JVG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4MUZ_00_Flow_mapping_colon_on_line_after_key_events() {
    test_events("./yaml-test-suite/4MUZ/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4MUZ_00_Flow_mapping_colon_on_line_after_key_json() {
    test_json("./yaml-test-suite/4MUZ/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4MUZ_00_Flow_mapping_colon_on_line_after_key_out() {
    test_out("./yaml-test-suite/4MUZ/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4MUZ_00_Flow_mapping_colon_on_line_after_key_emit() {
    test_emit("./yaml-test-suite/4MUZ/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4MUZ_01_Flow_mapping_colon_on_line_after_key_events() {
    test_events("./yaml-test-suite/4MUZ/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4MUZ_01_Flow_mapping_colon_on_line_after_key_json() {
    test_json("./yaml-test-suite/4MUZ/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4MUZ_01_Flow_mapping_colon_on_line_after_key_out() {
    test_out("./yaml-test-suite/4MUZ/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4MUZ_01_Flow_mapping_colon_on_line_after_key_emit() {
    test_emit("./yaml-test-suite/4MUZ/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4MUZ_02_Flow_mapping_colon_on_line_after_key_events() {
    test_events("./yaml-test-suite/4MUZ/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4MUZ_02_Flow_mapping_colon_on_line_after_key_json() {
    test_json("./yaml-test-suite/4MUZ/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4MUZ_02_Flow_mapping_colon_on_line_after_key_out() {
    test_out("./yaml-test-suite/4MUZ/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4MUZ_02_Flow_mapping_colon_on_line_after_key_emit() {
    test_emit("./yaml-test-suite/4MUZ/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4Q9F_Folded_Block_Scalar_1_3_events() {
    test_events("./yaml-test-suite/4Q9F");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4Q9F_Folded_Block_Scalar_1_3_json() {
    test_json("./yaml-test-suite/4Q9F");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4Q9F_Folded_Block_Scalar_1_3_out() {
    test_out("./yaml-test-suite/4Q9F");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4Q9F_Folded_Block_Scalar_1_3_emit() {
    test_emit("./yaml-test-suite/4Q9F");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4QFQ_Spec_Example_8_2_Block_Indentation_Indicator_1_3_events() {
    test_events("./yaml-test-suite/4QFQ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4QFQ_Spec_Example_8_2_Block_Indentation_Indicator_1_3_json() {
    test_json("./yaml-test-suite/4QFQ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4QFQ_Spec_Example_8_2_Block_Indentation_Indicator_1_3_out() {
    test_out("./yaml-test-suite/4QFQ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4QFQ_Spec_Example_8_2_Block_Indentation_Indicator_1_3_emit() {
    test_emit("./yaml-test-suite/4QFQ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4RWC_Trailing_spaces_after_flow_collection_events() {
    test_events("./yaml-test-suite/4RWC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4RWC_Trailing_spaces_after_flow_collection_json() {
    test_json("./yaml-test-suite/4RWC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4RWC_Trailing_spaces_after_flow_collection_out() {
    test_out("./yaml-test-suite/4RWC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4RWC_Trailing_spaces_after_flow_collection_emit() {
    test_emit("./yaml-test-suite/4RWC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4UYU_Colon_in_Double_Quoted_String_events() {
    test_events("./yaml-test-suite/4UYU");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4UYU_Colon_in_Double_Quoted_String_json() {
    test_json("./yaml-test-suite/4UYU");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4UYU_Colon_in_Double_Quoted_String_out() {
    test_out("./yaml-test-suite/4UYU");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4UYU_Colon_in_Double_Quoted_String_emit() {
    test_emit("./yaml-test-suite/4UYU");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4V8U_Plain_scalar_with_backslashes_events() {
    test_events("./yaml-test-suite/4V8U");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4V8U_Plain_scalar_with_backslashes_json() {
    test_json("./yaml-test-suite/4V8U");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4V8U_Plain_scalar_with_backslashes_out() {
    test_out("./yaml-test-suite/4V8U");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4V8U_Plain_scalar_with_backslashes_emit() {
    test_emit("./yaml-test-suite/4V8U");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4WA9_Literal_scalars_events() {
    test_events("./yaml-test-suite/4WA9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4WA9_Literal_scalars_json() {
    test_json("./yaml-test-suite/4WA9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4WA9_Literal_scalars_out() {
    test_out("./yaml-test-suite/4WA9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4WA9_Literal_scalars_emit() {
    test_emit("./yaml-test-suite/4WA9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4ZYM_Spec_Example_6_4_Line_Prefixes_events() {
    test_events("./yaml-test-suite/4ZYM");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4ZYM_Spec_Example_6_4_Line_Prefixes_json() {
    test_json("./yaml-test-suite/4ZYM");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4ZYM_Spec_Example_6_4_Line_Prefixes_out() {
    test_out("./yaml-test-suite/4ZYM");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_4ZYM_Spec_Example_6_4_Line_Prefixes_emit() {
    test_emit("./yaml-test-suite/4ZYM");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_52DL_Explicit_Non_Specific_Tag_1_3_events() {
    test_events("./yaml-test-suite/52DL");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_52DL_Explicit_Non_Specific_Tag_1_3_json() {
    test_json("./yaml-test-suite/52DL");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_52DL_Explicit_Non_Specific_Tag_1_3_out() {
    test_out("./yaml-test-suite/52DL");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_52DL_Explicit_Non_Specific_Tag_1_3_emit() {
    test_emit("./yaml-test-suite/52DL");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_54T7_Flow_Mapping_events() {
    test_events("./yaml-test-suite/54T7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_54T7_Flow_Mapping_json() {
    test_json("./yaml-test-suite/54T7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_54T7_Flow_Mapping_out() {
    test_out("./yaml-test-suite/54T7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_54T7_Flow_Mapping_emit() {
    test_emit("./yaml-test-suite/54T7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_55WF_Invalid_escape_in_double_quoted_string_error() {
    test_error("./yaml-test-suite/55WF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_55WF_Invalid_escape_in_double_quoted_string_events() {
    test_events("./yaml-test-suite/55WF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_565N_Construct_Binary_events() {
    test_events("./yaml-test-suite/565N");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_565N_Construct_Binary_json() {
    test_json("./yaml-test-suite/565N");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_565N_Construct_Binary_out() {
    test_out("./yaml-test-suite/565N");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_565N_Construct_Binary_emit() {
    test_emit("./yaml-test-suite/565N");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_57H4_Spec_Example_8_22_Block_Collection_Nodes_events() {
    test_events("./yaml-test-suite/57H4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_57H4_Spec_Example_8_22_Block_Collection_Nodes_json() {
    test_json("./yaml-test-suite/57H4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_57H4_Spec_Example_8_22_Block_Collection_Nodes_out() {
    test_out("./yaml-test-suite/57H4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_57H4_Spec_Example_8_22_Block_Collection_Nodes_emit() {
    test_emit("./yaml-test-suite/57H4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_58MP_Flow_mapping_edge_cases_events() {
    test_events("./yaml-test-suite/58MP");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_58MP_Flow_mapping_edge_cases_json() {
    test_json("./yaml-test-suite/58MP");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_58MP_Flow_mapping_edge_cases_out() {
    test_out("./yaml-test-suite/58MP");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_58MP_Flow_mapping_edge_cases_emit() {
    test_emit("./yaml-test-suite/58MP");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5BVJ_Spec_Example_5_7_Block_Scalar_Indicators_events() {
    test_events("./yaml-test-suite/5BVJ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5BVJ_Spec_Example_5_7_Block_Scalar_Indicators_json() {
    test_json("./yaml-test-suite/5BVJ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5BVJ_Spec_Example_5_7_Block_Scalar_Indicators_out() {
    test_out("./yaml-test-suite/5BVJ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5BVJ_Spec_Example_5_7_Block_Scalar_Indicators_emit() {
    test_emit("./yaml-test-suite/5BVJ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5C5M_Spec_Example_7_15_Flow_Mappings_events() {
    test_events("./yaml-test-suite/5C5M");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5C5M_Spec_Example_7_15_Flow_Mappings_json() {
    test_json("./yaml-test-suite/5C5M");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5C5M_Spec_Example_7_15_Flow_Mappings_out() {
    test_out("./yaml-test-suite/5C5M");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5C5M_Spec_Example_7_15_Flow_Mappings_emit() {
    test_emit("./yaml-test-suite/5C5M");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5GBF_Spec_Example_6_5_Empty_Lines_events() {
    test_events("./yaml-test-suite/5GBF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5GBF_Spec_Example_6_5_Empty_Lines_json() {
    test_json("./yaml-test-suite/5GBF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5GBF_Spec_Example_6_5_Empty_Lines_out() {
    test_out("./yaml-test-suite/5GBF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5GBF_Spec_Example_6_5_Empty_Lines_emit() {
    test_emit("./yaml-test-suite/5GBF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5KJE_Spec_Example_7_13_Flow_Sequence_events() {
    test_events("./yaml-test-suite/5KJE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5KJE_Spec_Example_7_13_Flow_Sequence_json() {
    test_json("./yaml-test-suite/5KJE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5KJE_Spec_Example_7_13_Flow_Sequence_out() {
    test_out("./yaml-test-suite/5KJE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5KJE_Spec_Example_7_13_Flow_Sequence_emit() {
    test_emit("./yaml-test-suite/5KJE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5LLU_Block_scalar_with_wrong_indented_line_after_spaces_only_error() {
    test_error("./yaml-test-suite/5LLU");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5LLU_Block_scalar_with_wrong_indented_line_after_spaces_only_events() {
    test_events("./yaml-test-suite/5LLU");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5MUD_Colon_and_adjacent_value_on_next_line_events() {
    test_events("./yaml-test-suite/5MUD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5MUD_Colon_and_adjacent_value_on_next_line_json() {
    test_json("./yaml-test-suite/5MUD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5MUD_Colon_and_adjacent_value_on_next_line_out() {
    test_out("./yaml-test-suite/5MUD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5MUD_Colon_and_adjacent_value_on_next_line_emit() {
    test_emit("./yaml-test-suite/5MUD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5NYZ_Spec_Example_6_9_Separated_Comment_events() {
    test_events("./yaml-test-suite/5NYZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5NYZ_Spec_Example_6_9_Separated_Comment_json() {
    test_json("./yaml-test-suite/5NYZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5NYZ_Spec_Example_6_9_Separated_Comment_out() {
    test_out("./yaml-test-suite/5NYZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5NYZ_Spec_Example_6_9_Separated_Comment_emit() {
    test_emit("./yaml-test-suite/5NYZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5T43_Colon_at_the_beginning_of_adjacent_flow_scalar_events() {
    test_events("./yaml-test-suite/5T43");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5T43_Colon_at_the_beginning_of_adjacent_flow_scalar_json() {
    test_json("./yaml-test-suite/5T43");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5T43_Colon_at_the_beginning_of_adjacent_flow_scalar_out() {
    test_out("./yaml-test-suite/5T43");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5T43_Colon_at_the_beginning_of_adjacent_flow_scalar_emit() {
    test_emit("./yaml-test-suite/5T43");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5TRB_Invalid_document_start_marker_in_doublequoted_tring_error() {
    test_error("./yaml-test-suite/5TRB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5TRB_Invalid_document_start_marker_in_doublequoted_tring_events() {
    test_events("./yaml-test-suite/5TRB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5TYM_Spec_Example_6_21_Local_Tag_Prefix_events() {
    test_events("./yaml-test-suite/5TYM");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5TYM_Spec_Example_6_21_Local_Tag_Prefix_json() {
    test_json("./yaml-test-suite/5TYM");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5TYM_Spec_Example_6_21_Local_Tag_Prefix_out() {
    test_out("./yaml-test-suite/5TYM");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5TYM_Spec_Example_6_21_Local_Tag_Prefix_emit() {
    test_emit("./yaml-test-suite/5TYM");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5U3A_Sequence_on_same_Line_as_Mapping_Key_error() {
    test_error("./yaml-test-suite/5U3A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5U3A_Sequence_on_same_Line_as_Mapping_Key_events() {
    test_events("./yaml-test-suite/5U3A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5WE3_Spec_Example_8_17_Explicit_Block_Mapping_Entries_events() {
    test_events("./yaml-test-suite/5WE3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5WE3_Spec_Example_8_17_Explicit_Block_Mapping_Entries_json() {
    test_json("./yaml-test-suite/5WE3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5WE3_Spec_Example_8_17_Explicit_Block_Mapping_Entries_out() {
    test_out("./yaml-test-suite/5WE3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_5WE3_Spec_Example_8_17_Explicit_Block_Mapping_Entries_emit() {
    test_emit("./yaml-test-suite/5WE3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_62EZ_Invalid_block_mapping_key_on_same_line_as_previous_key_error() {
    test_error("./yaml-test-suite/62EZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_62EZ_Invalid_block_mapping_key_on_same_line_as_previous_key_events() {
    test_events("./yaml-test-suite/62EZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_652Z_Question_mark_at_start_of_flow_key_events() {
    test_events("./yaml-test-suite/652Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_652Z_Question_mark_at_start_of_flow_key_json() {
    test_json("./yaml-test-suite/652Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_652Z_Question_mark_at_start_of_flow_key_out() {
    test_out("./yaml-test-suite/652Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_652Z_Question_mark_at_start_of_flow_key_emit() {
    test_emit("./yaml-test-suite/652Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_65WH_Single_Entry_Block_Sequence_events() {
    test_events("./yaml-test-suite/65WH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_65WH_Single_Entry_Block_Sequence_json() {
    test_json("./yaml-test-suite/65WH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_65WH_Single_Entry_Block_Sequence_out() {
    test_out("./yaml-test-suite/65WH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_65WH_Single_Entry_Block_Sequence_emit() {
    test_emit("./yaml-test-suite/65WH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6BCT_Spec_Example_6_3_Separation_Spaces_events() {
    test_events("./yaml-test-suite/6BCT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6BCT_Spec_Example_6_3_Separation_Spaces_json() {
    test_json("./yaml-test-suite/6BCT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6BCT_Spec_Example_6_3_Separation_Spaces_out() {
    test_out("./yaml-test-suite/6BCT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6BCT_Spec_Example_6_3_Separation_Spaces_emit() {
    test_emit("./yaml-test-suite/6BCT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6BFJ_Mapping_key_and_flow_sequence_item_anchors_events() {
    test_events("./yaml-test-suite/6BFJ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6BFJ_Mapping_key_and_flow_sequence_item_anchors_json() {
    test_json("./yaml-test-suite/6BFJ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6BFJ_Mapping_key_and_flow_sequence_item_anchors_out() {
    test_out("./yaml-test-suite/6BFJ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6BFJ_Mapping_key_and_flow_sequence_item_anchors_emit() {
    test_emit("./yaml-test-suite/6BFJ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6CA3_Tab_indented_top_flow_events() {
    test_events("./yaml-test-suite/6CA3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6CA3_Tab_indented_top_flow_json() {
    test_json("./yaml-test-suite/6CA3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6CA3_Tab_indented_top_flow_out() {
    test_out("./yaml-test-suite/6CA3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6CA3_Tab_indented_top_flow_emit() {
    test_emit("./yaml-test-suite/6CA3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6CK3_Spec_Example_6_26_Tag_Shorthands_events() {
    test_events("./yaml-test-suite/6CK3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6CK3_Spec_Example_6_26_Tag_Shorthands_json() {
    test_json("./yaml-test-suite/6CK3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6CK3_Spec_Example_6_26_Tag_Shorthands_out() {
    test_out("./yaml-test-suite/6CK3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6CK3_Spec_Example_6_26_Tag_Shorthands_emit() {
    test_emit("./yaml-test-suite/6CK3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6FWR_Block_Scalar_Keep_events() {
    test_events("./yaml-test-suite/6FWR");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6FWR_Block_Scalar_Keep_json() {
    test_json("./yaml-test-suite/6FWR");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6FWR_Block_Scalar_Keep_out() {
    test_out("./yaml-test-suite/6FWR");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6FWR_Block_Scalar_Keep_emit() {
    test_emit("./yaml-test-suite/6FWR");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6H3V_Backslashes_in_singlequotes_events() {
    test_events("./yaml-test-suite/6H3V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6H3V_Backslashes_in_singlequotes_json() {
    test_json("./yaml-test-suite/6H3V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6H3V_Backslashes_in_singlequotes_out() {
    test_out("./yaml-test-suite/6H3V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6H3V_Backslashes_in_singlequotes_emit() {
    test_emit("./yaml-test-suite/6H3V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6HB6_Spec_Example_6_1_Indentation_Spaces_events() {
    test_events("./yaml-test-suite/6HB6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6HB6_Spec_Example_6_1_Indentation_Spaces_json() {
    test_json("./yaml-test-suite/6HB6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6HB6_Spec_Example_6_1_Indentation_Spaces_out() {
    test_out("./yaml-test-suite/6HB6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6HB6_Spec_Example_6_1_Indentation_Spaces_emit() {
    test_emit("./yaml-test-suite/6HB6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6JQW_Spec_Example_2_13_In_literals_newlines_are_preserved_events() {
    test_events("./yaml-test-suite/6JQW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6JQW_Spec_Example_2_13_In_literals_newlines_are_preserved_json() {
    test_json("./yaml-test-suite/6JQW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6JQW_Spec_Example_2_13_In_literals_newlines_are_preserved_out() {
    test_out("./yaml-test-suite/6JQW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6JQW_Spec_Example_2_13_In_literals_newlines_are_preserved_emit() {
    test_emit("./yaml-test-suite/6JQW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6JTT_Flow_sequence_without_closing_bracket_error() {
    test_error("./yaml-test-suite/6JTT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6JTT_Flow_sequence_without_closing_bracket_events() {
    test_events("./yaml-test-suite/6JTT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6JWB_Tags_for_Block_Objects_events() {
    test_events("./yaml-test-suite/6JWB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6JWB_Tags_for_Block_Objects_json() {
    test_json("./yaml-test-suite/6JWB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6JWB_Tags_for_Block_Objects_out() {
    test_out("./yaml-test-suite/6JWB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6JWB_Tags_for_Block_Objects_emit() {
    test_emit("./yaml-test-suite/6JWB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6KGN_Anchor_for_empty_node_events() {
    test_events("./yaml-test-suite/6KGN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6KGN_Anchor_for_empty_node_json() {
    test_json("./yaml-test-suite/6KGN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6KGN_Anchor_for_empty_node_out() {
    test_out("./yaml-test-suite/6KGN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6KGN_Anchor_for_empty_node_emit() {
    test_emit("./yaml-test-suite/6KGN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6LVF_Spec_Example_6_13_Reserved_Directives_events() {
    test_events("./yaml-test-suite/6LVF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6LVF_Spec_Example_6_13_Reserved_Directives_json() {
    test_json("./yaml-test-suite/6LVF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6LVF_Spec_Example_6_13_Reserved_Directives_out() {
    test_out("./yaml-test-suite/6LVF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6LVF_Spec_Example_6_13_Reserved_Directives_emit() {
    test_emit("./yaml-test-suite/6LVF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6M2F_Aliases_in_Explicit_Block_Mapping_events() {
    test_events("./yaml-test-suite/6M2F");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6M2F_Aliases_in_Explicit_Block_Mapping_json() {
    test_json("./yaml-test-suite/6M2F");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6M2F_Aliases_in_Explicit_Block_Mapping_out() {
    test_out("./yaml-test-suite/6M2F");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6M2F_Aliases_in_Explicit_Block_Mapping_emit() {
    test_emit("./yaml-test-suite/6M2F");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6PBE_Zero_indented_sequences_in_explicit_mapping_keys_events() {
    test_events("./yaml-test-suite/6PBE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6PBE_Zero_indented_sequences_in_explicit_mapping_keys_json() {
    test_json("./yaml-test-suite/6PBE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6PBE_Zero_indented_sequences_in_explicit_mapping_keys_out() {
    test_out("./yaml-test-suite/6PBE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6PBE_Zero_indented_sequences_in_explicit_mapping_keys_emit() {
    test_emit("./yaml-test-suite/6PBE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6S55_Invalid_scalar_at_the_end_of_sequence_error() {
    test_error("./yaml-test-suite/6S55");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6S55_Invalid_scalar_at_the_end_of_sequence_events() {
    test_events("./yaml-test-suite/6S55");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6SLA_Allowed_characters_in_quoted_mapping_key_events() {
    test_events("./yaml-test-suite/6SLA");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6SLA_Allowed_characters_in_quoted_mapping_key_json() {
    test_json("./yaml-test-suite/6SLA");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6SLA_Allowed_characters_in_quoted_mapping_key_out() {
    test_out("./yaml-test-suite/6SLA");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6SLA_Allowed_characters_in_quoted_mapping_key_emit() {
    test_emit("./yaml-test-suite/6SLA");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6VJK_Spec_Example_2_15_Folded_newlines_are_preserved_for_more_indented_and_blank_lines_events(
) {
    test_events("./yaml-test-suite/6VJK");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6VJK_Spec_Example_2_15_Folded_newlines_are_preserved_for_more_indented_and_blank_lines_json(
) {
    test_json("./yaml-test-suite/6VJK");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6VJK_Spec_Example_2_15_Folded_newlines_are_preserved_for_more_indented_and_blank_lines_out()
{
    test_out("./yaml-test-suite/6VJK");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6VJK_Spec_Example_2_15_Folded_newlines_are_preserved_for_more_indented_and_blank_lines_emit(
) {
    test_emit("./yaml-test-suite/6VJK");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6WLZ_Spec_Example_6_18_Primary_Tag_Handle_1_3_events() {
    test_events("./yaml-test-suite/6WLZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6WLZ_Spec_Example_6_18_Primary_Tag_Handle_1_3_json() {
    test_json("./yaml-test-suite/6WLZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6WLZ_Spec_Example_6_18_Primary_Tag_Handle_1_3_out() {
    test_out("./yaml-test-suite/6WLZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6WLZ_Spec_Example_6_18_Primary_Tag_Handle_1_3_emit() {
    test_emit("./yaml-test-suite/6WLZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6WPF_Spec_Example_6_8_Flow_Folding_1_3_events() {
    test_events("./yaml-test-suite/6WPF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6WPF_Spec_Example_6_8_Flow_Folding_1_3_json() {
    test_json("./yaml-test-suite/6WPF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6WPF_Spec_Example_6_8_Flow_Folding_1_3_out() {
    test_out("./yaml-test-suite/6WPF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6WPF_Spec_Example_6_8_Flow_Folding_1_3_emit() {
    test_emit("./yaml-test-suite/6WPF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6XDY_Two_document_start_markers_events() {
    test_events("./yaml-test-suite/6XDY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6XDY_Two_document_start_markers_json() {
    test_json("./yaml-test-suite/6XDY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6XDY_Two_document_start_markers_out() {
    test_out("./yaml-test-suite/6XDY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6XDY_Two_document_start_markers_emit() {
    test_emit("./yaml-test-suite/6XDY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6ZKB_Spec_Example_9_6_Stream_events() {
    test_events("./yaml-test-suite/6ZKB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6ZKB_Spec_Example_9_6_Stream_json() {
    test_json("./yaml-test-suite/6ZKB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6ZKB_Spec_Example_9_6_Stream_out() {
    test_out("./yaml-test-suite/6ZKB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_6ZKB_Spec_Example_9_6_Stream_emit() {
    test_emit("./yaml-test-suite/6ZKB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_735Y_Spec_Example_8_20_Block_Node_Types_events() {
    test_events("./yaml-test-suite/735Y");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_735Y_Spec_Example_8_20_Block_Node_Types_json() {
    test_json("./yaml-test-suite/735Y");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_735Y_Spec_Example_8_20_Block_Node_Types_out() {
    test_out("./yaml-test-suite/735Y");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_735Y_Spec_Example_8_20_Block_Node_Types_emit() {
    test_emit("./yaml-test-suite/735Y");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_74H7_Tags_in_Implicit_Mapping_events() {
    test_events("./yaml-test-suite/74H7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_74H7_Tags_in_Implicit_Mapping_json() {
    test_json("./yaml-test-suite/74H7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_74H7_Tags_in_Implicit_Mapping_out() {
    test_out("./yaml-test-suite/74H7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_74H7_Tags_in_Implicit_Mapping_emit() {
    test_emit("./yaml-test-suite/74H7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_753E_Block_Scalar_Strip_1_3_events() {
    test_events("./yaml-test-suite/753E");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_753E_Block_Scalar_Strip_1_3_json() {
    test_json("./yaml-test-suite/753E");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_753E_Block_Scalar_Strip_1_3_out() {
    test_out("./yaml-test-suite/753E");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_753E_Block_Scalar_Strip_1_3_emit() {
    test_emit("./yaml-test-suite/753E");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7A4E_Spec_Example_7_6_Double_Quoted_Lines_events() {
    test_events("./yaml-test-suite/7A4E");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7A4E_Spec_Example_7_6_Double_Quoted_Lines_json() {
    test_json("./yaml-test-suite/7A4E");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7A4E_Spec_Example_7_6_Double_Quoted_Lines_out() {
    test_out("./yaml-test-suite/7A4E");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7A4E_Spec_Example_7_6_Double_Quoted_Lines_emit() {
    test_emit("./yaml-test-suite/7A4E");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7BMT_Node_and_Mapping_Key_Anchors_1_3_events() {
    test_events("./yaml-test-suite/7BMT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7BMT_Node_and_Mapping_Key_Anchors_1_3_json() {
    test_json("./yaml-test-suite/7BMT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7BMT_Node_and_Mapping_Key_Anchors_1_3_out() {
    test_out("./yaml-test-suite/7BMT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7BMT_Node_and_Mapping_Key_Anchors_1_3_emit() {
    test_emit("./yaml-test-suite/7BMT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7BUB_Spec_Example_2_10_Node_for_Sammy_Sosa_appears_twice_in_this_document_events() {
    test_events("./yaml-test-suite/7BUB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7BUB_Spec_Example_2_10_Node_for_Sammy_Sosa_appears_twice_in_this_document_json() {
    test_json("./yaml-test-suite/7BUB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7BUB_Spec_Example_2_10_Node_for_Sammy_Sosa_appears_twice_in_this_document_out() {
    test_out("./yaml-test-suite/7BUB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7BUB_Spec_Example_2_10_Node_for_Sammy_Sosa_appears_twice_in_this_document_emit() {
    test_emit("./yaml-test-suite/7BUB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7FWL_Spec_Example_6_24_Verbatim_Tags_events() {
    test_events("./yaml-test-suite/7FWL");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7FWL_Spec_Example_6_24_Verbatim_Tags_json() {
    test_json("./yaml-test-suite/7FWL");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7FWL_Spec_Example_6_24_Verbatim_Tags_out() {
    test_out("./yaml-test-suite/7FWL");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7FWL_Spec_Example_6_24_Verbatim_Tags_emit() {
    test_emit("./yaml-test-suite/7FWL");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7LBH_Multiline_double_quoted_implicit_keys_error() {
    test_error("./yaml-test-suite/7LBH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7LBH_Multiline_double_quoted_implicit_keys_events() {
    test_events("./yaml-test-suite/7LBH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7MNF_Missing_colon_error() {
    test_error("./yaml-test-suite/7MNF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7MNF_Missing_colon_events() {
    test_events("./yaml-test-suite/7MNF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7T8X_Spec_Example_8_10_Folded_Lines_8_13_Final_Empty_Lines_events() {
    test_events("./yaml-test-suite/7T8X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7T8X_Spec_Example_8_10_Folded_Lines_8_13_Final_Empty_Lines_json() {
    test_json("./yaml-test-suite/7T8X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7T8X_Spec_Example_8_10_Folded_Lines_8_13_Final_Empty_Lines_out() {
    test_out("./yaml-test-suite/7T8X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7T8X_Spec_Example_8_10_Folded_Lines_8_13_Final_Empty_Lines_emit() {
    test_emit("./yaml-test-suite/7T8X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7TMG_Comment_in_flow_sequence_before_comma_events() {
    test_events("./yaml-test-suite/7TMG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7TMG_Comment_in_flow_sequence_before_comma_json() {
    test_json("./yaml-test-suite/7TMG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7TMG_Comment_in_flow_sequence_before_comma_out() {
    test_out("./yaml-test-suite/7TMG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7TMG_Comment_in_flow_sequence_before_comma_emit() {
    test_emit("./yaml-test-suite/7TMG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7W2P_Block_Mapping_with_Missing_Values_events() {
    test_events("./yaml-test-suite/7W2P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7W2P_Block_Mapping_with_Missing_Values_json() {
    test_json("./yaml-test-suite/7W2P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7W2P_Block_Mapping_with_Missing_Values_out() {
    test_out("./yaml-test-suite/7W2P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7W2P_Block_Mapping_with_Missing_Values_emit() {
    test_emit("./yaml-test-suite/7W2P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7Z25_Bare_document_after_document_end_marker_events() {
    test_events("./yaml-test-suite/7Z25");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7Z25_Bare_document_after_document_end_marker_json() {
    test_json("./yaml-test-suite/7Z25");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7Z25_Bare_document_after_document_end_marker_out() {
    test_out("./yaml-test-suite/7Z25");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7Z25_Bare_document_after_document_end_marker_emit() {
    test_emit("./yaml-test-suite/7Z25");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7ZZ5_Empty_flow_collections_events() {
    test_events("./yaml-test-suite/7ZZ5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7ZZ5_Empty_flow_collections_json() {
    test_json("./yaml-test-suite/7ZZ5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7ZZ5_Empty_flow_collections_out() {
    test_out("./yaml-test-suite/7ZZ5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_7ZZ5_Empty_flow_collections_emit() {
    test_emit("./yaml-test-suite/7ZZ5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_82AN_Three_dashes_and_content_without_space_events() {
    test_events("./yaml-test-suite/82AN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_82AN_Three_dashes_and_content_without_space_json() {
    test_json("./yaml-test-suite/82AN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_82AN_Three_dashes_and_content_without_space_out() {
    test_out("./yaml-test-suite/82AN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_82AN_Three_dashes_and_content_without_space_emit() {
    test_emit("./yaml-test-suite/82AN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_87E4_Spec_Example_7_8_Single_Quoted_Implicit_Keys_events() {
    test_events("./yaml-test-suite/87E4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_87E4_Spec_Example_7_8_Single_Quoted_Implicit_Keys_json() {
    test_json("./yaml-test-suite/87E4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_87E4_Spec_Example_7_8_Single_Quoted_Implicit_Keys_out() {
    test_out("./yaml-test-suite/87E4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_87E4_Spec_Example_7_8_Single_Quoted_Implicit_Keys_emit() {
    test_emit("./yaml-test-suite/87E4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8CWC_Plain_mapping_key_ending_with_colon_events() {
    test_events("./yaml-test-suite/8CWC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8CWC_Plain_mapping_key_ending_with_colon_json() {
    test_json("./yaml-test-suite/8CWC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8CWC_Plain_mapping_key_ending_with_colon_out() {
    test_out("./yaml-test-suite/8CWC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8CWC_Plain_mapping_key_ending_with_colon_emit() {
    test_emit("./yaml-test-suite/8CWC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8G76_Spec_Example_6_10_Comment_Lines_events() {
    test_events("./yaml-test-suite/8G76");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8G76_Spec_Example_6_10_Comment_Lines_json() {
    test_json("./yaml-test-suite/8G76");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8G76_Spec_Example_6_10_Comment_Lines_out() {
    test_out("./yaml-test-suite/8G76");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8G76_Spec_Example_6_10_Comment_Lines_emit() {
    test_emit("./yaml-test-suite/8G76");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8KB6_Multiline_plain_flow_mapping_key_without_value_events() {
    test_events("./yaml-test-suite/8KB6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8KB6_Multiline_plain_flow_mapping_key_without_value_json() {
    test_json("./yaml-test-suite/8KB6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8KB6_Multiline_plain_flow_mapping_key_without_value_out() {
    test_out("./yaml-test-suite/8KB6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8KB6_Multiline_plain_flow_mapping_key_without_value_emit() {
    test_emit("./yaml-test-suite/8KB6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8MK2_Explicit_Non_Specific_Tag_events() {
    test_events("./yaml-test-suite/8MK2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8MK2_Explicit_Non_Specific_Tag_json() {
    test_json("./yaml-test-suite/8MK2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8MK2_Explicit_Non_Specific_Tag_out() {
    test_out("./yaml-test-suite/8MK2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8MK2_Explicit_Non_Specific_Tag_emit() {
    test_emit("./yaml-test-suite/8MK2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8QBE_Block_Sequence_in_Block_Mapping_events() {
    test_events("./yaml-test-suite/8QBE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8QBE_Block_Sequence_in_Block_Mapping_json() {
    test_json("./yaml-test-suite/8QBE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8QBE_Block_Sequence_in_Block_Mapping_out() {
    test_out("./yaml-test-suite/8QBE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8QBE_Block_Sequence_in_Block_Mapping_emit() {
    test_emit("./yaml-test-suite/8QBE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8UDB_Spec_Example_7_14_Flow_Sequence_Entries_events() {
    test_events("./yaml-test-suite/8UDB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8UDB_Spec_Example_7_14_Flow_Sequence_Entries_json() {
    test_json("./yaml-test-suite/8UDB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8UDB_Spec_Example_7_14_Flow_Sequence_Entries_out() {
    test_out("./yaml-test-suite/8UDB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8UDB_Spec_Example_7_14_Flow_Sequence_Entries_emit() {
    test_emit("./yaml-test-suite/8UDB");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8XDJ_Comment_in_plain_multiline_value_error() {
    test_error("./yaml-test-suite/8XDJ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8XDJ_Comment_in_plain_multiline_value_events() {
    test_events("./yaml-test-suite/8XDJ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8XYN_Anchor_with_unicode_character_events() {
    test_events("./yaml-test-suite/8XYN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8XYN_Anchor_with_unicode_character_json() {
    test_json("./yaml-test-suite/8XYN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8XYN_Anchor_with_unicode_character_out() {
    test_out("./yaml-test-suite/8XYN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_8XYN_Anchor_with_unicode_character_emit() {
    test_emit("./yaml-test-suite/8XYN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_93JH_Block_Mappings_in_Block_Sequence_events() {
    test_events("./yaml-test-suite/93JH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_93JH_Block_Mappings_in_Block_Sequence_json() {
    test_json("./yaml-test-suite/93JH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_93JH_Block_Mappings_in_Block_Sequence_out() {
    test_out("./yaml-test-suite/93JH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_93JH_Block_Mappings_in_Block_Sequence_emit() {
    test_emit("./yaml-test-suite/93JH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_93WF_Spec_Example_6_6_Line_Folding_1_3_events() {
    test_events("./yaml-test-suite/93WF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_93WF_Spec_Example_6_6_Line_Folding_1_3_json() {
    test_json("./yaml-test-suite/93WF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_93WF_Spec_Example_6_6_Line_Folding_1_3_out() {
    test_out("./yaml-test-suite/93WF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_93WF_Spec_Example_6_6_Line_Folding_1_3_emit() {
    test_emit("./yaml-test-suite/93WF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_96L6_Spec_Example_2_14_In_the_folded_scalars_newlines_become_spaces_events() {
    test_events("./yaml-test-suite/96L6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_96L6_Spec_Example_2_14_In_the_folded_scalars_newlines_become_spaces_json() {
    test_json("./yaml-test-suite/96L6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_96L6_Spec_Example_2_14_In_the_folded_scalars_newlines_become_spaces_out() {
    test_out("./yaml-test-suite/96L6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_96L6_Spec_Example_2_14_In_the_folded_scalars_newlines_become_spaces_emit() {
    test_emit("./yaml-test-suite/96L6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_96NN_00_Leading_tab_content_in_literals_events() {
    test_events("./yaml-test-suite/96NN/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_96NN_00_Leading_tab_content_in_literals_json() {
    test_json("./yaml-test-suite/96NN/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_96NN_00_Leading_tab_content_in_literals_out() {
    test_out("./yaml-test-suite/96NN/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_96NN_00_Leading_tab_content_in_literals_emit() {
    test_emit("./yaml-test-suite/96NN/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_96NN_01_Leading_tab_content_in_literals_events() {
    test_events("./yaml-test-suite/96NN/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_96NN_01_Leading_tab_content_in_literals_json() {
    test_json("./yaml-test-suite/96NN/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_96NN_01_Leading_tab_content_in_literals_out() {
    test_out("./yaml-test-suite/96NN/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_96NN_01_Leading_tab_content_in_literals_emit() {
    test_emit("./yaml-test-suite/96NN/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_98YD_Spec_Example_5_5_Comment_Indicator_events() {
    test_events("./yaml-test-suite/98YD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_98YD_Spec_Example_5_5_Comment_Indicator_json() {
    test_json("./yaml-test-suite/98YD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_98YD_Spec_Example_5_5_Comment_Indicator_out() {
    test_out("./yaml-test-suite/98YD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_98YD_Spec_Example_5_5_Comment_Indicator_emit() {
    test_emit("./yaml-test-suite/98YD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9BXH_Multiline_doublequoted_flow_mapping_key_without_value_events() {
    test_events("./yaml-test-suite/9BXH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9BXH_Multiline_doublequoted_flow_mapping_key_without_value_json() {
    test_json("./yaml-test-suite/9BXH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9BXH_Multiline_doublequoted_flow_mapping_key_without_value_out() {
    test_out("./yaml-test-suite/9BXH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9BXH_Multiline_doublequoted_flow_mapping_key_without_value_emit() {
    test_emit("./yaml-test-suite/9BXH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9C9N_Wrong_indented_flow_sequence_error() {
    test_error("./yaml-test-suite/9C9N");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9C9N_Wrong_indented_flow_sequence_events() {
    test_events("./yaml-test-suite/9C9N");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9CWY_Invalid_scalar_at_the_end_of_mapping_error() {
    test_error("./yaml-test-suite/9CWY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9CWY_Invalid_scalar_at_the_end_of_mapping_events() {
    test_events("./yaml-test-suite/9CWY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9DXL_Spec_Example_9_6_Stream_1_3_events() {
    test_events("./yaml-test-suite/9DXL");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9DXL_Spec_Example_9_6_Stream_1_3_json() {
    test_json("./yaml-test-suite/9DXL");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9DXL_Spec_Example_9_6_Stream_1_3_out() {
    test_out("./yaml-test-suite/9DXL");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9DXL_Spec_Example_9_6_Stream_1_3_emit() {
    test_emit("./yaml-test-suite/9DXL");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9FMG_Multi_level_Mapping_Indent_events() {
    test_events("./yaml-test-suite/9FMG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9FMG_Multi_level_Mapping_Indent_json() {
    test_json("./yaml-test-suite/9FMG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9FMG_Multi_level_Mapping_Indent_out() {
    test_out("./yaml-test-suite/9FMG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9FMG_Multi_level_Mapping_Indent_emit() {
    test_emit("./yaml-test-suite/9FMG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9HCY_Need_document_footer_before_directives_error() {
    test_error("./yaml-test-suite/9HCY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9HCY_Need_document_footer_before_directives_events() {
    test_events("./yaml-test-suite/9HCY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9J7A_Simple_Mapping_Indent_events() {
    test_events("./yaml-test-suite/9J7A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9J7A_Simple_Mapping_Indent_json() {
    test_json("./yaml-test-suite/9J7A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9J7A_Simple_Mapping_Indent_out() {
    test_out("./yaml-test-suite/9J7A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9J7A_Simple_Mapping_Indent_emit() {
    test_emit("./yaml-test-suite/9J7A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9JBA_Invalid_comment_after_end_of_flow_sequence_error() {
    test_error("./yaml-test-suite/9JBA");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9JBA_Invalid_comment_after_end_of_flow_sequence_events() {
    test_events("./yaml-test-suite/9JBA");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9KAX_Various_combinations_of_tags_and_anchors_events() {
    test_events("./yaml-test-suite/9KAX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9KAX_Various_combinations_of_tags_and_anchors_json() {
    test_json("./yaml-test-suite/9KAX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9KAX_Various_combinations_of_tags_and_anchors_out() {
    test_out("./yaml-test-suite/9KAX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9KAX_Various_combinations_of_tags_and_anchors_emit() {
    test_emit("./yaml-test-suite/9KAX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9KBC_Mapping_starting_at_line_error() {
    test_error("./yaml-test-suite/9KBC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9KBC_Mapping_starting_at_line_events() {
    test_events("./yaml-test-suite/9KBC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9MAG_Flow_sequence_with_invalid_comma_at_the_beginning_error() {
    test_error("./yaml-test-suite/9MAG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9MAG_Flow_sequence_with_invalid_comma_at_the_beginning_events() {
    test_events("./yaml-test-suite/9MAG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9MMA_Directive_by_itself_with_no_document_error() {
    test_error("./yaml-test-suite/9MMA");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9MMA_Directive_by_itself_with_no_document_events() {
    test_events("./yaml-test-suite/9MMA");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9MMW_Single_Pair_Implicit_Entries_events() {
    test_events("./yaml-test-suite/9MMW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9MMW_Single_Pair_Implicit_Entries_json() {
    test_json("./yaml-test-suite/9MMW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9MMW_Single_Pair_Implicit_Entries_out() {
    test_out("./yaml-test-suite/9MMW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9MMW_Single_Pair_Implicit_Entries_emit() {
    test_emit("./yaml-test-suite/9MMW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9MQT_00_Scalar_doc_with_in_content_events() {
    test_events("./yaml-test-suite/9MQT/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9MQT_00_Scalar_doc_with_in_content_json() {
    test_json("./yaml-test-suite/9MQT/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9MQT_00_Scalar_doc_with_in_content_out() {
    test_out("./yaml-test-suite/9MQT/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9MQT_00_Scalar_doc_with_in_content_emit() {
    test_emit("./yaml-test-suite/9MQT/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9MQT_01_Scalar_doc_with_in_content_error() {
    test_error("./yaml-test-suite/9MQT/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9MQT_01_Scalar_doc_with_in_content_events() {
    test_events("./yaml-test-suite/9MQT/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9SA2_Multiline_double_quoted_flow_mapping_key_events() {
    test_events("./yaml-test-suite/9SA2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9SA2_Multiline_double_quoted_flow_mapping_key_json() {
    test_json("./yaml-test-suite/9SA2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9SA2_Multiline_double_quoted_flow_mapping_key_out() {
    test_out("./yaml-test-suite/9SA2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9SA2_Multiline_double_quoted_flow_mapping_key_emit() {
    test_emit("./yaml-test-suite/9SA2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9SHH_Spec_Example_5_8_Quoted_Scalar_Indicators_events() {
    test_events("./yaml-test-suite/9SHH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9SHH_Spec_Example_5_8_Quoted_Scalar_Indicators_json() {
    test_json("./yaml-test-suite/9SHH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9SHH_Spec_Example_5_8_Quoted_Scalar_Indicators_out() {
    test_out("./yaml-test-suite/9SHH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9SHH_Spec_Example_5_8_Quoted_Scalar_Indicators_emit() {
    test_emit("./yaml-test-suite/9SHH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9TFX_Spec_Example_7_6_Double_Quoted_Lines_1_3_events() {
    test_events("./yaml-test-suite/9TFX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9TFX_Spec_Example_7_6_Double_Quoted_Lines_1_3_json() {
    test_json("./yaml-test-suite/9TFX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9TFX_Spec_Example_7_6_Double_Quoted_Lines_1_3_out() {
    test_out("./yaml-test-suite/9TFX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9TFX_Spec_Example_7_6_Double_Quoted_Lines_1_3_emit() {
    test_emit("./yaml-test-suite/9TFX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9U5K_Spec_Example_2_12_Compact_Nested_Mapping_events() {
    test_events("./yaml-test-suite/9U5K");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9U5K_Spec_Example_2_12_Compact_Nested_Mapping_json() {
    test_json("./yaml-test-suite/9U5K");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9U5K_Spec_Example_2_12_Compact_Nested_Mapping_out() {
    test_out("./yaml-test-suite/9U5K");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9U5K_Spec_Example_2_12_Compact_Nested_Mapping_emit() {
    test_emit("./yaml-test-suite/9U5K");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9WXW_Spec_Example_6_18_Primary_Tag_Handle_events() {
    test_events("./yaml-test-suite/9WXW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9WXW_Spec_Example_6_18_Primary_Tag_Handle_json() {
    test_json("./yaml-test-suite/9WXW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9WXW_Spec_Example_6_18_Primary_Tag_Handle_out() {
    test_out("./yaml-test-suite/9WXW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9WXW_Spec_Example_6_18_Primary_Tag_Handle_emit() {
    test_emit("./yaml-test-suite/9WXW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9YRD_Multiline_Scalar_at_Top_Level_events() {
    test_events("./yaml-test-suite/9YRD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9YRD_Multiline_Scalar_at_Top_Level_json() {
    test_json("./yaml-test-suite/9YRD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9YRD_Multiline_Scalar_at_Top_Level_out() {
    test_out("./yaml-test-suite/9YRD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_9YRD_Multiline_Scalar_at_Top_Level_emit() {
    test_emit("./yaml-test-suite/9YRD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_A2M4_Spec_Example_6_2_Indentation_Indicators_events() {
    test_events("./yaml-test-suite/A2M4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_A2M4_Spec_Example_6_2_Indentation_Indicators_json() {
    test_json("./yaml-test-suite/A2M4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_A2M4_Spec_Example_6_2_Indentation_Indicators_out() {
    test_out("./yaml-test-suite/A2M4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_A2M4_Spec_Example_6_2_Indentation_Indicators_emit() {
    test_emit("./yaml-test-suite/A2M4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_A6F9_Spec_Example_8_4_Chomping_Final_Line_Break_events() {
    test_events("./yaml-test-suite/A6F9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_A6F9_Spec_Example_8_4_Chomping_Final_Line_Break_json() {
    test_json("./yaml-test-suite/A6F9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_A6F9_Spec_Example_8_4_Chomping_Final_Line_Break_out() {
    test_out("./yaml-test-suite/A6F9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_A6F9_Spec_Example_8_4_Chomping_Final_Line_Break_emit() {
    test_emit("./yaml-test-suite/A6F9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_A984_Multiline_Scalar_in_Mapping_events() {
    test_events("./yaml-test-suite/A984");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_A984_Multiline_Scalar_in_Mapping_json() {
    test_json("./yaml-test-suite/A984");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_A984_Multiline_Scalar_in_Mapping_out() {
    test_out("./yaml-test-suite/A984");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_A984_Multiline_Scalar_in_Mapping_emit() {
    test_emit("./yaml-test-suite/A984");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_AB8U_Sequence_entry_that_looks_like_two_with_wrong_indentation_events() {
    test_events("./yaml-test-suite/AB8U");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_AB8U_Sequence_entry_that_looks_like_two_with_wrong_indentation_json() {
    test_json("./yaml-test-suite/AB8U");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_AB8U_Sequence_entry_that_looks_like_two_with_wrong_indentation_out() {
    test_out("./yaml-test-suite/AB8U");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_AB8U_Sequence_entry_that_looks_like_two_with_wrong_indentation_emit() {
    test_emit("./yaml-test-suite/AB8U");
}

#[test]
#[allow(non_snake_case)]
fn test_AVM7_Empty_Stream_events() {
    test_events("./yaml-test-suite/AVM7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_AVM7_Empty_Stream_json() {
    test_json("./yaml-test-suite/AVM7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_AVM7_Empty_Stream_out() {
    test_out("./yaml-test-suite/AVM7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_AVM7_Empty_Stream_emit() {
    test_emit("./yaml-test-suite/AVM7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_AZ63_Sequence_With_Same_Indentation_as_Parent_Mapping_events() {
    test_events("./yaml-test-suite/AZ63");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_AZ63_Sequence_With_Same_Indentation_as_Parent_Mapping_json() {
    test_json("./yaml-test-suite/AZ63");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_AZ63_Sequence_With_Same_Indentation_as_Parent_Mapping_out() {
    test_out("./yaml-test-suite/AZ63");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_AZ63_Sequence_With_Same_Indentation_as_Parent_Mapping_emit() {
    test_emit("./yaml-test-suite/AZ63");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_AZW3_Lookahead_test_cases_events() {
    test_events("./yaml-test-suite/AZW3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_AZW3_Lookahead_test_cases_json() {
    test_json("./yaml-test-suite/AZW3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_AZW3_Lookahead_test_cases_out() {
    test_out("./yaml-test-suite/AZW3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_AZW3_Lookahead_test_cases_emit() {
    test_emit("./yaml-test-suite/AZW3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_B3HG_Spec_Example_8_9_Folded_Scalar_1_3_events() {
    test_events("./yaml-test-suite/B3HG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_B3HG_Spec_Example_8_9_Folded_Scalar_1_3_json() {
    test_json("./yaml-test-suite/B3HG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_B3HG_Spec_Example_8_9_Folded_Scalar_1_3_out() {
    test_out("./yaml-test-suite/B3HG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_B3HG_Spec_Example_8_9_Folded_Scalar_1_3_emit() {
    test_emit("./yaml-test-suite/B3HG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_B63P_Directive_without_document_error() {
    test_error("./yaml-test-suite/B63P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_B63P_Directive_without_document_events() {
    test_events("./yaml-test-suite/B63P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_BD7L_Invalid_mapping_after_sequence_error() {
    test_error("./yaml-test-suite/BD7L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_BD7L_Invalid_mapping_after_sequence_events() {
    test_events("./yaml-test-suite/BD7L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_BEC7_Spec_Example_6_14_YAML_directive_events() {
    test_events("./yaml-test-suite/BEC7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_BEC7_Spec_Example_6_14_YAML_directive_json() {
    test_json("./yaml-test-suite/BEC7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_BEC7_Spec_Example_6_14_YAML_directive_out() {
    test_out("./yaml-test-suite/BEC7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_BEC7_Spec_Example_6_14_YAML_directive_emit() {
    test_emit("./yaml-test-suite/BEC7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_BF9H_Trailing_comment_in_multiline_plain_scalar_error() {
    test_error("./yaml-test-suite/BF9H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_BF9H_Trailing_comment_in_multiline_plain_scalar_events() {
    test_events("./yaml-test-suite/BF9H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_BS4K_Comment_between_plain_scalar_lines_error() {
    test_error("./yaml-test-suite/BS4K");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_BS4K_Comment_between_plain_scalar_lines_events() {
    test_events("./yaml-test-suite/BS4K");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_BU8L_Node_Anchor_and_Tag_on_Seperate_Lines_events() {
    test_events("./yaml-test-suite/BU8L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_BU8L_Node_Anchor_and_Tag_on_Seperate_Lines_json() {
    test_json("./yaml-test-suite/BU8L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_BU8L_Node_Anchor_and_Tag_on_Seperate_Lines_out() {
    test_out("./yaml-test-suite/BU8L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_BU8L_Node_Anchor_and_Tag_on_Seperate_Lines_emit() {
    test_emit("./yaml-test-suite/BU8L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_C2DT_Spec_Example_7_18_Flow_Mapping_Adjacent_Values_events() {
    test_events("./yaml-test-suite/C2DT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_C2DT_Spec_Example_7_18_Flow_Mapping_Adjacent_Values_json() {
    test_json("./yaml-test-suite/C2DT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_C2DT_Spec_Example_7_18_Flow_Mapping_Adjacent_Values_out() {
    test_out("./yaml-test-suite/C2DT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_C2DT_Spec_Example_7_18_Flow_Mapping_Adjacent_Values_emit() {
    test_emit("./yaml-test-suite/C2DT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_C2SP_Flow_Mapping_Key_on_two_lines_error() {
    test_error("./yaml-test-suite/C2SP");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_C2SP_Flow_Mapping_Key_on_two_lines_events() {
    test_events("./yaml-test-suite/C2SP");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_C4HZ_Spec_Example_2_24_Global_Tags_events() {
    test_events("./yaml-test-suite/C4HZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_C4HZ_Spec_Example_2_24_Global_Tags_json() {
    test_json("./yaml-test-suite/C4HZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_C4HZ_Spec_Example_2_24_Global_Tags_out() {
    test_out("./yaml-test-suite/C4HZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_C4HZ_Spec_Example_2_24_Global_Tags_emit() {
    test_emit("./yaml-test-suite/C4HZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CC74_Spec_Example_6_20_Tag_Handles_events() {
    test_events("./yaml-test-suite/CC74");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CC74_Spec_Example_6_20_Tag_Handles_json() {
    test_json("./yaml-test-suite/CC74");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CC74_Spec_Example_6_20_Tag_Handles_out() {
    test_out("./yaml-test-suite/CC74");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CC74_Spec_Example_6_20_Tag_Handles_emit() {
    test_emit("./yaml-test-suite/CC74");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CFD4_Empty_implicit_key_in_single_pair_flow_sequences_events() {
    test_events("./yaml-test-suite/CFD4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CFD4_Empty_implicit_key_in_single_pair_flow_sequences_json() {
    test_json("./yaml-test-suite/CFD4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CFD4_Empty_implicit_key_in_single_pair_flow_sequences_out() {
    test_out("./yaml-test-suite/CFD4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CFD4_Empty_implicit_key_in_single_pair_flow_sequences_emit() {
    test_emit("./yaml-test-suite/CFD4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CML9_Missing_comma_in_flow_error() {
    test_error("./yaml-test-suite/CML9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CML9_Missing_comma_in_flow_events() {
    test_events("./yaml-test-suite/CML9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CN3R_Various_location_of_anchors_in_flow_sequence_events() {
    test_events("./yaml-test-suite/CN3R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CN3R_Various_location_of_anchors_in_flow_sequence_json() {
    test_json("./yaml-test-suite/CN3R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CN3R_Various_location_of_anchors_in_flow_sequence_out() {
    test_out("./yaml-test-suite/CN3R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CN3R_Various_location_of_anchors_in_flow_sequence_emit() {
    test_emit("./yaml-test-suite/CN3R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CPZ3_Doublequoted_scalar_starting_with_a_tab_events() {
    test_events("./yaml-test-suite/CPZ3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CPZ3_Doublequoted_scalar_starting_with_a_tab_json() {
    test_json("./yaml-test-suite/CPZ3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CPZ3_Doublequoted_scalar_starting_with_a_tab_out() {
    test_out("./yaml-test-suite/CPZ3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CPZ3_Doublequoted_scalar_starting_with_a_tab_emit() {
    test_emit("./yaml-test-suite/CPZ3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CQ3W_Double_quoted_string_without_closing_quote_error() {
    test_error("./yaml-test-suite/CQ3W");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CQ3W_Double_quoted_string_without_closing_quote_events() {
    test_events("./yaml-test-suite/CQ3W");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CT4Q_Spec_Example_7_20_Single_Pair_Explicit_Entry_events() {
    test_events("./yaml-test-suite/CT4Q");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CT4Q_Spec_Example_7_20_Single_Pair_Explicit_Entry_json() {
    test_json("./yaml-test-suite/CT4Q");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CT4Q_Spec_Example_7_20_Single_Pair_Explicit_Entry_out() {
    test_out("./yaml-test-suite/CT4Q");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CT4Q_Spec_Example_7_20_Single_Pair_Explicit_Entry_emit() {
    test_emit("./yaml-test-suite/CT4Q");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CTN5_Flow_sequence_with_invalid_extra_comma_error() {
    test_error("./yaml-test-suite/CTN5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CTN5_Flow_sequence_with_invalid_extra_comma_events() {
    test_events("./yaml-test-suite/CTN5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CUP7_Spec_Example_5_6_Node_Property_Indicators_events() {
    test_events("./yaml-test-suite/CUP7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CUP7_Spec_Example_5_6_Node_Property_Indicators_json() {
    test_json("./yaml-test-suite/CUP7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CUP7_Spec_Example_5_6_Node_Property_Indicators_out() {
    test_out("./yaml-test-suite/CUP7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CUP7_Spec_Example_5_6_Node_Property_Indicators_emit() {
    test_emit("./yaml-test-suite/CUP7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CVW2_Invalid_comment_after_comma_error() {
    test_error("./yaml-test-suite/CVW2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CVW2_Invalid_comment_after_comma_events() {
    test_events("./yaml-test-suite/CVW2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CXX2_Mapping_with_anchor_on_document_start_line_error() {
    test_error("./yaml-test-suite/CXX2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_CXX2_Mapping_with_anchor_on_document_start_line_events() {
    test_events("./yaml-test-suite/CXX2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_D49Q_Multiline_single_quoted_implicit_keys_error() {
    test_error("./yaml-test-suite/D49Q");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_D49Q_Multiline_single_quoted_implicit_keys_events() {
    test_events("./yaml-test-suite/D49Q");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_D83L_Block_scalar_indicator_order_events() {
    test_events("./yaml-test-suite/D83L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_D83L_Block_scalar_indicator_order_json() {
    test_json("./yaml-test-suite/D83L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_D83L_Block_scalar_indicator_order_out() {
    test_out("./yaml-test-suite/D83L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_D83L_Block_scalar_indicator_order_emit() {
    test_emit("./yaml-test-suite/D83L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_D88J_Flow_Sequence_in_Block_Mapping_events() {
    test_events("./yaml-test-suite/D88J");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_D88J_Flow_Sequence_in_Block_Mapping_json() {
    test_json("./yaml-test-suite/D88J");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_D88J_Flow_Sequence_in_Block_Mapping_out() {
    test_out("./yaml-test-suite/D88J");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_D88J_Flow_Sequence_in_Block_Mapping_emit() {
    test_emit("./yaml-test-suite/D88J");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_D9TU_Single_Pair_Block_Mapping_events() {
    test_events("./yaml-test-suite/D9TU");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_D9TU_Single_Pair_Block_Mapping_json() {
    test_json("./yaml-test-suite/D9TU");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_D9TU_Single_Pair_Block_Mapping_out() {
    test_out("./yaml-test-suite/D9TU");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_D9TU_Single_Pair_Block_Mapping_emit() {
    test_emit("./yaml-test-suite/D9TU");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DBG4_Spec_Example_7_10_Plain_Characters_events() {
    test_events("./yaml-test-suite/DBG4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DBG4_Spec_Example_7_10_Plain_Characters_json() {
    test_json("./yaml-test-suite/DBG4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DBG4_Spec_Example_7_10_Plain_Characters_out() {
    test_out("./yaml-test-suite/DBG4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DBG4_Spec_Example_7_10_Plain_Characters_emit() {
    test_emit("./yaml-test-suite/DBG4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DC7X_Various_trailing_tabs_events() {
    test_events("./yaml-test-suite/DC7X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DC7X_Various_trailing_tabs_json() {
    test_json("./yaml-test-suite/DC7X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DC7X_Various_trailing_tabs_out() {
    test_out("./yaml-test-suite/DC7X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DC7X_Various_trailing_tabs_emit() {
    test_emit("./yaml-test-suite/DC7X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_00_Trailing_tabs_in_double_quoted_events() {
    test_events("./yaml-test-suite/DE56/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_00_Trailing_tabs_in_double_quoted_json() {
    test_json("./yaml-test-suite/DE56/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_00_Trailing_tabs_in_double_quoted_out() {
    test_out("./yaml-test-suite/DE56/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_00_Trailing_tabs_in_double_quoted_emit() {
    test_emit("./yaml-test-suite/DE56/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_01_Trailing_tabs_in_double_quoted_events() {
    test_events("./yaml-test-suite/DE56/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_01_Trailing_tabs_in_double_quoted_json() {
    test_json("./yaml-test-suite/DE56/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_01_Trailing_tabs_in_double_quoted_out() {
    test_out("./yaml-test-suite/DE56/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_01_Trailing_tabs_in_double_quoted_emit() {
    test_emit("./yaml-test-suite/DE56/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_02_Trailing_tabs_in_double_quoted_events() {
    test_events("./yaml-test-suite/DE56/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_02_Trailing_tabs_in_double_quoted_json() {
    test_json("./yaml-test-suite/DE56/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_02_Trailing_tabs_in_double_quoted_out() {
    test_out("./yaml-test-suite/DE56/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_02_Trailing_tabs_in_double_quoted_emit() {
    test_emit("./yaml-test-suite/DE56/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_03_Trailing_tabs_in_double_quoted_events() {
    test_events("./yaml-test-suite/DE56/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_03_Trailing_tabs_in_double_quoted_json() {
    test_json("./yaml-test-suite/DE56/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_03_Trailing_tabs_in_double_quoted_out() {
    test_out("./yaml-test-suite/DE56/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_03_Trailing_tabs_in_double_quoted_emit() {
    test_emit("./yaml-test-suite/DE56/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_04_Trailing_tabs_in_double_quoted_events() {
    test_events("./yaml-test-suite/DE56/04");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_04_Trailing_tabs_in_double_quoted_json() {
    test_json("./yaml-test-suite/DE56/04");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_04_Trailing_tabs_in_double_quoted_out() {
    test_out("./yaml-test-suite/DE56/04");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_04_Trailing_tabs_in_double_quoted_emit() {
    test_emit("./yaml-test-suite/DE56/04");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_05_Trailing_tabs_in_double_quoted_events() {
    test_events("./yaml-test-suite/DE56/05");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_05_Trailing_tabs_in_double_quoted_json() {
    test_json("./yaml-test-suite/DE56/05");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_05_Trailing_tabs_in_double_quoted_out() {
    test_out("./yaml-test-suite/DE56/05");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DE56_05_Trailing_tabs_in_double_quoted_emit() {
    test_emit("./yaml-test-suite/DE56/05");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DFF7_Spec_Example_7_16_Flow_Mapping_Entries_events() {
    test_events("./yaml-test-suite/DFF7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DFF7_Spec_Example_7_16_Flow_Mapping_Entries_json() {
    test_json("./yaml-test-suite/DFF7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DFF7_Spec_Example_7_16_Flow_Mapping_Entries_out() {
    test_out("./yaml-test-suite/DFF7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DFF7_Spec_Example_7_16_Flow_Mapping_Entries_emit() {
    test_emit("./yaml-test-suite/DFF7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DHP8_Flow_Sequence_events() {
    test_events("./yaml-test-suite/DHP8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DHP8_Flow_Sequence_json() {
    test_json("./yaml-test-suite/DHP8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DHP8_Flow_Sequence_out() {
    test_out("./yaml-test-suite/DHP8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DHP8_Flow_Sequence_emit() {
    test_emit("./yaml-test-suite/DHP8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK3J_Zero_indented_block_scalar_with_line_that_looks_like_a_comment_events() {
    test_events("./yaml-test-suite/DK3J");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK3J_Zero_indented_block_scalar_with_line_that_looks_like_a_comment_json() {
    test_json("./yaml-test-suite/DK3J");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK3J_Zero_indented_block_scalar_with_line_that_looks_like_a_comment_out() {
    test_out("./yaml-test-suite/DK3J");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK3J_Zero_indented_block_scalar_with_line_that_looks_like_a_comment_emit() {
    test_emit("./yaml-test-suite/DK3J");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK4H_Implicit_key_followed_by_newline_error() {
    test_error("./yaml-test-suite/DK4H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK4H_Implicit_key_followed_by_newline_events() {
    test_events("./yaml-test-suite/DK4H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_00_Tabs_that_look_like_indentation_events() {
    test_events("./yaml-test-suite/DK95/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_00_Tabs_that_look_like_indentation_json() {
    test_json("./yaml-test-suite/DK95/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_00_Tabs_that_look_like_indentation_out() {
    test_out("./yaml-test-suite/DK95/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_00_Tabs_that_look_like_indentation_emit() {
    test_emit("./yaml-test-suite/DK95/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_01_Tabs_that_look_like_indentation_error() {
    test_error("./yaml-test-suite/DK95/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_01_Tabs_that_look_like_indentation_events() {
    test_events("./yaml-test-suite/DK95/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_02_Tabs_that_look_like_indentation_events() {
    test_events("./yaml-test-suite/DK95/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_02_Tabs_that_look_like_indentation_json() {
    test_json("./yaml-test-suite/DK95/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_02_Tabs_that_look_like_indentation_out() {
    test_out("./yaml-test-suite/DK95/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_02_Tabs_that_look_like_indentation_emit() {
    test_emit("./yaml-test-suite/DK95/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_03_Tabs_that_look_like_indentation_events() {
    test_events("./yaml-test-suite/DK95/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_03_Tabs_that_look_like_indentation_json() {
    test_json("./yaml-test-suite/DK95/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_03_Tabs_that_look_like_indentation_out() {
    test_out("./yaml-test-suite/DK95/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_03_Tabs_that_look_like_indentation_emit() {
    test_emit("./yaml-test-suite/DK95/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_04_Tabs_that_look_like_indentation_events() {
    test_events("./yaml-test-suite/DK95/04");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_04_Tabs_that_look_like_indentation_json() {
    test_json("./yaml-test-suite/DK95/04");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_04_Tabs_that_look_like_indentation_out() {
    test_out("./yaml-test-suite/DK95/04");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_04_Tabs_that_look_like_indentation_emit() {
    test_emit("./yaml-test-suite/DK95/04");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_05_Tabs_that_look_like_indentation_events() {
    test_events("./yaml-test-suite/DK95/05");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_05_Tabs_that_look_like_indentation_json() {
    test_json("./yaml-test-suite/DK95/05");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_05_Tabs_that_look_like_indentation_out() {
    test_out("./yaml-test-suite/DK95/05");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_05_Tabs_that_look_like_indentation_emit() {
    test_emit("./yaml-test-suite/DK95/05");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_06_Tabs_that_look_like_indentation_error() {
    test_error("./yaml-test-suite/DK95/06");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_06_Tabs_that_look_like_indentation_events() {
    test_events("./yaml-test-suite/DK95/06");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_07_Tabs_that_look_like_indentation_events() {
    test_events("./yaml-test-suite/DK95/07");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_07_Tabs_that_look_like_indentation_json() {
    test_json("./yaml-test-suite/DK95/07");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_07_Tabs_that_look_like_indentation_out() {
    test_out("./yaml-test-suite/DK95/07");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_07_Tabs_that_look_like_indentation_emit() {
    test_emit("./yaml-test-suite/DK95/07");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_08_Tabs_that_look_like_indentation_events() {
    test_events("./yaml-test-suite/DK95/08");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_08_Tabs_that_look_like_indentation_json() {
    test_json("./yaml-test-suite/DK95/08");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_08_Tabs_that_look_like_indentation_out() {
    test_out("./yaml-test-suite/DK95/08");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DK95_08_Tabs_that_look_like_indentation_emit() {
    test_emit("./yaml-test-suite/DK95/08");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DMG6_Wrong_indendation_in_Map_error() {
    test_error("./yaml-test-suite/DMG6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DMG6_Wrong_indendation_in_Map_events() {
    test_events("./yaml-test-suite/DMG6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DWX9_Spec_Example_8_8_Literal_Content_events() {
    test_events("./yaml-test-suite/DWX9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DWX9_Spec_Example_8_8_Literal_Content_json() {
    test_json("./yaml-test-suite/DWX9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DWX9_Spec_Example_8_8_Literal_Content_out() {
    test_out("./yaml-test-suite/DWX9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_DWX9_Spec_Example_8_8_Literal_Content_emit() {
    test_emit("./yaml-test-suite/DWX9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_E76Z_Aliases_in_Implicit_Block_Mapping_events() {
    test_events("./yaml-test-suite/E76Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_E76Z_Aliases_in_Implicit_Block_Mapping_json() {
    test_json("./yaml-test-suite/E76Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_E76Z_Aliases_in_Implicit_Block_Mapping_out() {
    test_out("./yaml-test-suite/E76Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_E76Z_Aliases_in_Implicit_Block_Mapping_emit() {
    test_emit("./yaml-test-suite/E76Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_EB22_Missing_document_end_marker_before_directive_error() {
    test_error("./yaml-test-suite/EB22");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_EB22_Missing_document_end_marker_before_directive_events() {
    test_events("./yaml-test-suite/EB22");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_EHF6_Tags_for_Flow_Objects_events() {
    test_events("./yaml-test-suite/EHF6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_EHF6_Tags_for_Flow_Objects_json() {
    test_json("./yaml-test-suite/EHF6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_EHF6_Tags_for_Flow_Objects_out() {
    test_out("./yaml-test-suite/EHF6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_EHF6_Tags_for_Flow_Objects_emit() {
    test_emit("./yaml-test-suite/EHF6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_EW3V_Wrong_indendation_in_mapping_error() {
    test_error("./yaml-test-suite/EW3V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_EW3V_Wrong_indendation_in_mapping_events() {
    test_events("./yaml-test-suite/EW3V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_EX5H_Multiline_Scalar_at_Top_Level_1_3_events() {
    test_events("./yaml-test-suite/EX5H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_EX5H_Multiline_Scalar_at_Top_Level_1_3_json() {
    test_json("./yaml-test-suite/EX5H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_EX5H_Multiline_Scalar_at_Top_Level_1_3_out() {
    test_out("./yaml-test-suite/EX5H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_EX5H_Multiline_Scalar_at_Top_Level_1_3_emit() {
    test_emit("./yaml-test-suite/EX5H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_EXG3_Three_dashes_and_content_without_space_1_3_events() {
    test_events("./yaml-test-suite/EXG3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_EXG3_Three_dashes_and_content_without_space_1_3_json() {
    test_json("./yaml-test-suite/EXG3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_EXG3_Three_dashes_and_content_without_space_1_3_out() {
    test_out("./yaml-test-suite/EXG3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_EXG3_Three_dashes_and_content_without_space_1_3_emit() {
    test_emit("./yaml-test-suite/EXG3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_F2C7_Anchors_and_Tags_events() {
    test_events("./yaml-test-suite/F2C7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_F2C7_Anchors_and_Tags_json() {
    test_json("./yaml-test-suite/F2C7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_F2C7_Anchors_and_Tags_out() {
    test_out("./yaml-test-suite/F2C7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_F2C7_Anchors_and_Tags_emit() {
    test_emit("./yaml-test-suite/F2C7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_F3CP_Nested_flow_collections_on_one_line_events() {
    test_events("./yaml-test-suite/F3CP");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_F3CP_Nested_flow_collections_on_one_line_json() {
    test_json("./yaml-test-suite/F3CP");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_F3CP_Nested_flow_collections_on_one_line_out() {
    test_out("./yaml-test-suite/F3CP");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_F3CP_Nested_flow_collections_on_one_line_emit() {
    test_emit("./yaml-test-suite/F3CP");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_F6MC_More_indented_lines_at_the_beginning_of_folded_block_scalars_events() {
    test_events("./yaml-test-suite/F6MC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_F6MC_More_indented_lines_at_the_beginning_of_folded_block_scalars_json() {
    test_json("./yaml-test-suite/F6MC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_F6MC_More_indented_lines_at_the_beginning_of_folded_block_scalars_out() {
    test_out("./yaml-test-suite/F6MC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_F6MC_More_indented_lines_at_the_beginning_of_folded_block_scalars_emit() {
    test_emit("./yaml-test-suite/F6MC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_F8F9_Spec_Example_8_5_Chomping_Trailing_Lines_events() {
    test_events("./yaml-test-suite/F8F9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_F8F9_Spec_Example_8_5_Chomping_Trailing_Lines_json() {
    test_json("./yaml-test-suite/F8F9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_F8F9_Spec_Example_8_5_Chomping_Trailing_Lines_out() {
    test_out("./yaml-test-suite/F8F9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_F8F9_Spec_Example_8_5_Chomping_Trailing_Lines_emit() {
    test_emit("./yaml-test-suite/F8F9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FBC9_Allowed_characters_in_plain_scalars_events() {
    test_events("./yaml-test-suite/FBC9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FBC9_Allowed_characters_in_plain_scalars_json() {
    test_json("./yaml-test-suite/FBC9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FBC9_Allowed_characters_in_plain_scalars_out() {
    test_out("./yaml-test-suite/FBC9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FBC9_Allowed_characters_in_plain_scalars_emit() {
    test_emit("./yaml-test-suite/FBC9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FH7J_Tags_on_Empty_Scalars_events() {
    test_events("./yaml-test-suite/FH7J");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FH7J_Tags_on_Empty_Scalars_json() {
    test_json("./yaml-test-suite/FH7J");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FH7J_Tags_on_Empty_Scalars_out() {
    test_out("./yaml-test-suite/FH7J");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FH7J_Tags_on_Empty_Scalars_emit() {
    test_emit("./yaml-test-suite/FH7J");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FP8R_Zero_indented_block_scalar_events() {
    test_events("./yaml-test-suite/FP8R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FP8R_Zero_indented_block_scalar_json() {
    test_json("./yaml-test-suite/FP8R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FP8R_Zero_indented_block_scalar_out() {
    test_out("./yaml-test-suite/FP8R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FP8R_Zero_indented_block_scalar_emit() {
    test_emit("./yaml-test-suite/FP8R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FQ7F_Spec_Example_2_1_Sequence_of_Scalars_events() {
    test_events("./yaml-test-suite/FQ7F");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FQ7F_Spec_Example_2_1_Sequence_of_Scalars_json() {
    test_json("./yaml-test-suite/FQ7F");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FQ7F_Spec_Example_2_1_Sequence_of_Scalars_out() {
    test_out("./yaml-test-suite/FQ7F");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FQ7F_Spec_Example_2_1_Sequence_of_Scalars_emit() {
    test_emit("./yaml-test-suite/FQ7F");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FRK4_Spec_Example_7_3_Completely_Empty_Flow_Nodes_events() {
    test_events("./yaml-test-suite/FRK4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FRK4_Spec_Example_7_3_Completely_Empty_Flow_Nodes_json() {
    test_json("./yaml-test-suite/FRK4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FRK4_Spec_Example_7_3_Completely_Empty_Flow_Nodes_out() {
    test_out("./yaml-test-suite/FRK4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FRK4_Spec_Example_7_3_Completely_Empty_Flow_Nodes_emit() {
    test_emit("./yaml-test-suite/FRK4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FTA2_Single_block_sequence_with_anchor_and_explicit_document_start_events() {
    test_events("./yaml-test-suite/FTA2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FTA2_Single_block_sequence_with_anchor_and_explicit_document_start_json() {
    test_json("./yaml-test-suite/FTA2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FTA2_Single_block_sequence_with_anchor_and_explicit_document_start_out() {
    test_out("./yaml-test-suite/FTA2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FTA2_Single_block_sequence_with_anchor_and_explicit_document_start_emit() {
    test_emit("./yaml-test-suite/FTA2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FUP4_Flow_Sequence_in_Flow_Sequence_events() {
    test_events("./yaml-test-suite/FUP4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FUP4_Flow_Sequence_in_Flow_Sequence_json() {
    test_json("./yaml-test-suite/FUP4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FUP4_Flow_Sequence_in_Flow_Sequence_out() {
    test_out("./yaml-test-suite/FUP4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_FUP4_Flow_Sequence_in_Flow_Sequence_emit() {
    test_emit("./yaml-test-suite/FUP4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_G4RS_Spec_Example_2_17_Quoted_Scalars_events() {
    test_events("./yaml-test-suite/G4RS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_G4RS_Spec_Example_2_17_Quoted_Scalars_json() {
    test_json("./yaml-test-suite/G4RS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_G4RS_Spec_Example_2_17_Quoted_Scalars_out() {
    test_out("./yaml-test-suite/G4RS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_G4RS_Spec_Example_2_17_Quoted_Scalars_emit() {
    test_emit("./yaml-test-suite/G4RS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_G5U8_Plain_dashes_in_flow_sequence_error() {
    test_error("./yaml-test-suite/G5U8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_G5U8_Plain_dashes_in_flow_sequence_events() {
    test_events("./yaml-test-suite/G5U8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_G7JE_Multiline_implicit_keys_error() {
    test_error("./yaml-test-suite/G7JE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_G7JE_Multiline_implicit_keys_events() {
    test_events("./yaml-test-suite/G7JE");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_G992_Spec_Example_8_9_Folded_Scalar_events() {
    test_events("./yaml-test-suite/G992");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_G992_Spec_Example_8_9_Folded_Scalar_json() {
    test_json("./yaml-test-suite/G992");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_G992_Spec_Example_8_9_Folded_Scalar_out() {
    test_out("./yaml-test-suite/G992");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_G992_Spec_Example_8_9_Folded_Scalar_emit() {
    test_emit("./yaml-test-suite/G992");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_G9HC_Invalid_anchor_in_zero_indented_sequence_error() {
    test_error("./yaml-test-suite/G9HC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_G9HC_Invalid_anchor_in_zero_indented_sequence_events() {
    test_events("./yaml-test-suite/G9HC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_GDY7_Comment_that_looks_like_a_mapping_key_error() {
    test_error("./yaml-test-suite/GDY7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_GDY7_Comment_that_looks_like_a_mapping_key_events() {
    test_events("./yaml-test-suite/GDY7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_GH63_Mixed_Block_Mapping_explicit_to_implicit_events() {
    test_events("./yaml-test-suite/GH63");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_GH63_Mixed_Block_Mapping_explicit_to_implicit_json() {
    test_json("./yaml-test-suite/GH63");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_GH63_Mixed_Block_Mapping_explicit_to_implicit_out() {
    test_out("./yaml-test-suite/GH63");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_GH63_Mixed_Block_Mapping_explicit_to_implicit_emit() {
    test_emit("./yaml-test-suite/GH63");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_GT5M_Node_anchor_in_sequence_error() {
    test_error("./yaml-test-suite/GT5M");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_GT5M_Node_anchor_in_sequence_events() {
    test_events("./yaml-test-suite/GT5M");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_H2RW_Blank_lines_events() {
    test_events("./yaml-test-suite/H2RW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_H2RW_Blank_lines_json() {
    test_json("./yaml-test-suite/H2RW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_H2RW_Blank_lines_out() {
    test_out("./yaml-test-suite/H2RW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_H2RW_Blank_lines_emit() {
    test_emit("./yaml-test-suite/H2RW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_H3Z8_Literal_unicode_events() {
    test_events("./yaml-test-suite/H3Z8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_H3Z8_Literal_unicode_json() {
    test_json("./yaml-test-suite/H3Z8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_H3Z8_Literal_unicode_out() {
    test_out("./yaml-test-suite/H3Z8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_H3Z8_Literal_unicode_emit() {
    test_emit("./yaml-test-suite/H3Z8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_H7J7_Node_anchor_not_indented_error() {
    test_error("./yaml-test-suite/H7J7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_H7J7_Node_anchor_not_indented_events() {
    test_events("./yaml-test-suite/H7J7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_H7TQ_Extra_words_on_YAML_directive_error() {
    test_error("./yaml-test-suite/H7TQ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_H7TQ_Extra_words_on_YAML_directive_events() {
    test_events("./yaml-test-suite/H7TQ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HM87_00_Scalars_in_flow_start_with_syntax_char_events() {
    test_events("./yaml-test-suite/HM87/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HM87_00_Scalars_in_flow_start_with_syntax_char_json() {
    test_json("./yaml-test-suite/HM87/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HM87_00_Scalars_in_flow_start_with_syntax_char_out() {
    test_out("./yaml-test-suite/HM87/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HM87_00_Scalars_in_flow_start_with_syntax_char_emit() {
    test_emit("./yaml-test-suite/HM87/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HM87_01_Scalars_in_flow_start_with_syntax_char_events() {
    test_events("./yaml-test-suite/HM87/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HM87_01_Scalars_in_flow_start_with_syntax_char_json() {
    test_json("./yaml-test-suite/HM87/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HM87_01_Scalars_in_flow_start_with_syntax_char_out() {
    test_out("./yaml-test-suite/HM87/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HM87_01_Scalars_in_flow_start_with_syntax_char_emit() {
    test_emit("./yaml-test-suite/HM87/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HMK4_Spec_Example_2_16_Indentation_determines_scope_events() {
    test_events("./yaml-test-suite/HMK4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HMK4_Spec_Example_2_16_Indentation_determines_scope_json() {
    test_json("./yaml-test-suite/HMK4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HMK4_Spec_Example_2_16_Indentation_determines_scope_out() {
    test_out("./yaml-test-suite/HMK4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HMK4_Spec_Example_2_16_Indentation_determines_scope_emit() {
    test_emit("./yaml-test-suite/HMK4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HMQ5_Spec_Example_6_23_Node_Properties_events() {
    test_events("./yaml-test-suite/HMQ5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HMQ5_Spec_Example_6_23_Node_Properties_json() {
    test_json("./yaml-test-suite/HMQ5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HMQ5_Spec_Example_6_23_Node_Properties_out() {
    test_out("./yaml-test-suite/HMQ5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HMQ5_Spec_Example_6_23_Node_Properties_emit() {
    test_emit("./yaml-test-suite/HMQ5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HRE5_Double_quoted_scalar_with_escaped_single_quote_error() {
    test_error("./yaml-test-suite/HRE5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HRE5_Double_quoted_scalar_with_escaped_single_quote_events() {
    test_events("./yaml-test-suite/HRE5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HS5T_Spec_Example_7_12_Plain_Lines_events() {
    test_events("./yaml-test-suite/HS5T");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HS5T_Spec_Example_7_12_Plain_Lines_json() {
    test_json("./yaml-test-suite/HS5T");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HS5T_Spec_Example_7_12_Plain_Lines_out() {
    test_out("./yaml-test-suite/HS5T");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HS5T_Spec_Example_7_12_Plain_Lines_emit() {
    test_emit("./yaml-test-suite/HS5T");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HU3P_Invalid_Mapping_in_plain_scalar_error() {
    test_error("./yaml-test-suite/HU3P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HU3P_Invalid_Mapping_in_plain_scalar_events() {
    test_events("./yaml-test-suite/HU3P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HWV9_Document_end_marker_events() {
    test_events("./yaml-test-suite/HWV9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HWV9_Document_end_marker_json() {
    test_json("./yaml-test-suite/HWV9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HWV9_Document_end_marker_out() {
    test_out("./yaml-test-suite/HWV9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_HWV9_Document_end_marker_emit() {
    test_emit("./yaml-test-suite/HWV9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J3BT_Spec_Example_5_12_Tabs_and_Spaces_events() {
    test_events("./yaml-test-suite/J3BT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J3BT_Spec_Example_5_12_Tabs_and_Spaces_json() {
    test_json("./yaml-test-suite/J3BT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J3BT_Spec_Example_5_12_Tabs_and_Spaces_out() {
    test_out("./yaml-test-suite/J3BT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J3BT_Spec_Example_5_12_Tabs_and_Spaces_emit() {
    test_emit("./yaml-test-suite/J3BT");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J5UC_Multiple_Pair_Block_Mapping_events() {
    test_events("./yaml-test-suite/J5UC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J5UC_Multiple_Pair_Block_Mapping_json() {
    test_json("./yaml-test-suite/J5UC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J5UC_Multiple_Pair_Block_Mapping_out() {
    test_out("./yaml-test-suite/J5UC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J5UC_Multiple_Pair_Block_Mapping_emit() {
    test_emit("./yaml-test-suite/J5UC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J7PZ_Spec_Example_2_26_Ordered_Mappings_events() {
    test_events("./yaml-test-suite/J7PZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J7PZ_Spec_Example_2_26_Ordered_Mappings_json() {
    test_json("./yaml-test-suite/J7PZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J7PZ_Spec_Example_2_26_Ordered_Mappings_out() {
    test_out("./yaml-test-suite/J7PZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J7PZ_Spec_Example_2_26_Ordered_Mappings_emit() {
    test_emit("./yaml-test-suite/J7PZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J7VC_Empty_Lines_Between_Mapping_Elements_events() {
    test_events("./yaml-test-suite/J7VC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J7VC_Empty_Lines_Between_Mapping_Elements_json() {
    test_json("./yaml-test-suite/J7VC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J7VC_Empty_Lines_Between_Mapping_Elements_out() {
    test_out("./yaml-test-suite/J7VC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J7VC_Empty_Lines_Between_Mapping_Elements_emit() {
    test_emit("./yaml-test-suite/J7VC");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J9HZ_Spec_Example_2_9_Single_Document_with_Two_Comments_events() {
    test_events("./yaml-test-suite/J9HZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J9HZ_Spec_Example_2_9_Single_Document_with_Two_Comments_json() {
    test_json("./yaml-test-suite/J9HZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J9HZ_Spec_Example_2_9_Single_Document_with_Two_Comments_out() {
    test_out("./yaml-test-suite/J9HZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_J9HZ_Spec_Example_2_9_Single_Document_with_Two_Comments_emit() {
    test_emit("./yaml-test-suite/J9HZ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JEF9_00_Trailing_whitespace_in_streams_events() {
    test_events("./yaml-test-suite/JEF9/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JEF9_00_Trailing_whitespace_in_streams_json() {
    test_json("./yaml-test-suite/JEF9/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JEF9_00_Trailing_whitespace_in_streams_out() {
    test_out("./yaml-test-suite/JEF9/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JEF9_00_Trailing_whitespace_in_streams_emit() {
    test_emit("./yaml-test-suite/JEF9/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JEF9_01_Trailing_whitespace_in_streams_events() {
    test_events("./yaml-test-suite/JEF9/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JEF9_01_Trailing_whitespace_in_streams_json() {
    test_json("./yaml-test-suite/JEF9/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JEF9_01_Trailing_whitespace_in_streams_out() {
    test_out("./yaml-test-suite/JEF9/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JEF9_01_Trailing_whitespace_in_streams_emit() {
    test_emit("./yaml-test-suite/JEF9/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JEF9_02_Trailing_whitespace_in_streams_events() {
    test_events("./yaml-test-suite/JEF9/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JEF9_02_Trailing_whitespace_in_streams_json() {
    test_json("./yaml-test-suite/JEF9/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JEF9_02_Trailing_whitespace_in_streams_out() {
    test_out("./yaml-test-suite/JEF9/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JEF9_02_Trailing_whitespace_in_streams_emit() {
    test_emit("./yaml-test-suite/JEF9/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JHB9_Spec_Example_2_7_Two_Documents_in_a_Stream_events() {
    test_events("./yaml-test-suite/JHB9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JHB9_Spec_Example_2_7_Two_Documents_in_a_Stream_json() {
    test_json("./yaml-test-suite/JHB9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JHB9_Spec_Example_2_7_Two_Documents_in_a_Stream_out() {
    test_out("./yaml-test-suite/JHB9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JHB9_Spec_Example_2_7_Two_Documents_in_a_Stream_emit() {
    test_emit("./yaml-test-suite/JHB9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JKF3_Multiline_unidented_double_quoted_block_key_error() {
    test_error("./yaml-test-suite/JKF3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JKF3_Multiline_unidented_double_quoted_block_key_events() {
    test_events("./yaml-test-suite/JKF3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JQ4R_Spec_Example_8_14_Block_Sequence_events() {
    test_events("./yaml-test-suite/JQ4R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JQ4R_Spec_Example_8_14_Block_Sequence_json() {
    test_json("./yaml-test-suite/JQ4R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JQ4R_Spec_Example_8_14_Block_Sequence_out() {
    test_out("./yaml-test-suite/JQ4R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JQ4R_Spec_Example_8_14_Block_Sequence_emit() {
    test_emit("./yaml-test-suite/JQ4R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JR7V_Question_marks_in_scalars_events() {
    test_events("./yaml-test-suite/JR7V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JR7V_Question_marks_in_scalars_json() {
    test_json("./yaml-test-suite/JR7V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JR7V_Question_marks_in_scalars_out() {
    test_out("./yaml-test-suite/JR7V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JR7V_Question_marks_in_scalars_emit() {
    test_emit("./yaml-test-suite/JR7V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JS2J_Spec_Example_6_29_Node_Anchors_events() {
    test_events("./yaml-test-suite/JS2J");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JS2J_Spec_Example_6_29_Node_Anchors_json() {
    test_json("./yaml-test-suite/JS2J");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JS2J_Spec_Example_6_29_Node_Anchors_out() {
    test_out("./yaml-test-suite/JS2J");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JS2J_Spec_Example_6_29_Node_Anchors_emit() {
    test_emit("./yaml-test-suite/JS2J");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JTV5_Block_Mapping_with_Multiline_Scalars_events() {
    test_events("./yaml-test-suite/JTV5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JTV5_Block_Mapping_with_Multiline_Scalars_json() {
    test_json("./yaml-test-suite/JTV5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JTV5_Block_Mapping_with_Multiline_Scalars_out() {
    test_out("./yaml-test-suite/JTV5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JTV5_Block_Mapping_with_Multiline_Scalars_emit() {
    test_emit("./yaml-test-suite/JTV5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JY7Z_Trailing_content_that_looks_like_a_mapping_error() {
    test_error("./yaml-test-suite/JY7Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_JY7Z_Trailing_content_that_looks_like_a_mapping_events() {
    test_events("./yaml-test-suite/JY7Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K3WX_Colon_and_adjacent_value_after_comment_on_next_line_events() {
    test_events("./yaml-test-suite/K3WX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K3WX_Colon_and_adjacent_value_after_comment_on_next_line_json() {
    test_json("./yaml-test-suite/K3WX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K3WX_Colon_and_adjacent_value_after_comment_on_next_line_out() {
    test_out("./yaml-test-suite/K3WX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K3WX_Colon_and_adjacent_value_after_comment_on_next_line_emit() {
    test_emit("./yaml-test-suite/K3WX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K4SU_Multiple_Entry_Block_Sequence_events() {
    test_events("./yaml-test-suite/K4SU");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K4SU_Multiple_Entry_Block_Sequence_json() {
    test_json("./yaml-test-suite/K4SU");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K4SU_Multiple_Entry_Block_Sequence_out() {
    test_out("./yaml-test-suite/K4SU");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K4SU_Multiple_Entry_Block_Sequence_emit() {
    test_emit("./yaml-test-suite/K4SU");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K527_Spec_Example_6_6_Line_Folding_events() {
    test_events("./yaml-test-suite/K527");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K527_Spec_Example_6_6_Line_Folding_json() {
    test_json("./yaml-test-suite/K527");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K527_Spec_Example_6_6_Line_Folding_out() {
    test_out("./yaml-test-suite/K527");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K527_Spec_Example_6_6_Line_Folding_emit() {
    test_emit("./yaml-test-suite/K527");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K54U_Tab_after_document_header_events() {
    test_events("./yaml-test-suite/K54U");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K54U_Tab_after_document_header_json() {
    test_json("./yaml-test-suite/K54U");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K54U_Tab_after_document_header_out() {
    test_out("./yaml-test-suite/K54U");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K54U_Tab_after_document_header_emit() {
    test_emit("./yaml-test-suite/K54U");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K858_Spec_Example_8_6_Empty_Scalar_Chomping_events() {
    test_events("./yaml-test-suite/K858");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K858_Spec_Example_8_6_Empty_Scalar_Chomping_json() {
    test_json("./yaml-test-suite/K858");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K858_Spec_Example_8_6_Empty_Scalar_Chomping_out() {
    test_out("./yaml-test-suite/K858");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_K858_Spec_Example_8_6_Empty_Scalar_Chomping_emit() {
    test_emit("./yaml-test-suite/K858");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KH5V_00_Inline_tabs_in_double_quoted_events() {
    test_events("./yaml-test-suite/KH5V/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KH5V_00_Inline_tabs_in_double_quoted_json() {
    test_json("./yaml-test-suite/KH5V/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KH5V_00_Inline_tabs_in_double_quoted_out() {
    test_out("./yaml-test-suite/KH5V/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KH5V_00_Inline_tabs_in_double_quoted_emit() {
    test_emit("./yaml-test-suite/KH5V/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KH5V_01_Inline_tabs_in_double_quoted_events() {
    test_events("./yaml-test-suite/KH5V/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KH5V_01_Inline_tabs_in_double_quoted_json() {
    test_json("./yaml-test-suite/KH5V/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KH5V_01_Inline_tabs_in_double_quoted_out() {
    test_out("./yaml-test-suite/KH5V/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KH5V_01_Inline_tabs_in_double_quoted_emit() {
    test_emit("./yaml-test-suite/KH5V/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KH5V_02_Inline_tabs_in_double_quoted_events() {
    test_events("./yaml-test-suite/KH5V/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KH5V_02_Inline_tabs_in_double_quoted_json() {
    test_json("./yaml-test-suite/KH5V/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KH5V_02_Inline_tabs_in_double_quoted_out() {
    test_out("./yaml-test-suite/KH5V/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KH5V_02_Inline_tabs_in_double_quoted_emit() {
    test_emit("./yaml-test-suite/KH5V/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KK5P_Various_combinations_of_explicit_block_mappings_events() {
    test_events("./yaml-test-suite/KK5P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KK5P_Various_combinations_of_explicit_block_mappings_json() {
    test_json("./yaml-test-suite/KK5P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KK5P_Various_combinations_of_explicit_block_mappings_out() {
    test_out("./yaml-test-suite/KK5P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KK5P_Various_combinations_of_explicit_block_mappings_emit() {
    test_emit("./yaml-test-suite/KK5P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KMK3_Block_Submapping_events() {
    test_events("./yaml-test-suite/KMK3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KMK3_Block_Submapping_json() {
    test_json("./yaml-test-suite/KMK3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KMK3_Block_Submapping_out() {
    test_out("./yaml-test-suite/KMK3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KMK3_Block_Submapping_emit() {
    test_emit("./yaml-test-suite/KMK3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KS4U_Invalid_item_after_end_of_flow_sequence_error() {
    test_error("./yaml-test-suite/KS4U");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KS4U_Invalid_item_after_end_of_flow_sequence_events() {
    test_events("./yaml-test-suite/KS4U");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KSS4_Scalars_on_line_events() {
    test_events("./yaml-test-suite/KSS4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KSS4_Scalars_on_line_json() {
    test_json("./yaml-test-suite/KSS4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KSS4_Scalars_on_line_out() {
    test_out("./yaml-test-suite/KSS4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_KSS4_Scalars_on_line_emit() {
    test_emit("./yaml-test-suite/KSS4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L24T_00_Trailing_line_of_spaces_events() {
    test_events("./yaml-test-suite/L24T/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L24T_00_Trailing_line_of_spaces_json() {
    test_json("./yaml-test-suite/L24T/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L24T_00_Trailing_line_of_spaces_out() {
    test_out("./yaml-test-suite/L24T/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L24T_00_Trailing_line_of_spaces_emit() {
    test_emit("./yaml-test-suite/L24T/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L24T_01_Trailing_line_of_spaces_events() {
    test_events("./yaml-test-suite/L24T/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L24T_01_Trailing_line_of_spaces_json() {
    test_json("./yaml-test-suite/L24T/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L24T_01_Trailing_line_of_spaces_out() {
    test_out("./yaml-test-suite/L24T/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L24T_01_Trailing_line_of_spaces_emit() {
    test_emit("./yaml-test-suite/L24T/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L383_Two_scalar_docs_with_trailing_comments_events() {
    test_events("./yaml-test-suite/L383");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L383_Two_scalar_docs_with_trailing_comments_json() {
    test_json("./yaml-test-suite/L383");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L383_Two_scalar_docs_with_trailing_comments_out() {
    test_out("./yaml-test-suite/L383");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L383_Two_scalar_docs_with_trailing_comments_emit() {
    test_emit("./yaml-test-suite/L383");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L94M_Tags_in_Explicit_Mapping_events() {
    test_events("./yaml-test-suite/L94M");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L94M_Tags_in_Explicit_Mapping_json() {
    test_json("./yaml-test-suite/L94M");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L94M_Tags_in_Explicit_Mapping_out() {
    test_out("./yaml-test-suite/L94M");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L94M_Tags_in_Explicit_Mapping_emit() {
    test_emit("./yaml-test-suite/L94M");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L9U5_Spec_Example_7_11_Plain_Implicit_Keys_events() {
    test_events("./yaml-test-suite/L9U5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L9U5_Spec_Example_7_11_Plain_Implicit_Keys_json() {
    test_json("./yaml-test-suite/L9U5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L9U5_Spec_Example_7_11_Plain_Implicit_Keys_out() {
    test_out("./yaml-test-suite/L9U5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_L9U5_Spec_Example_7_11_Plain_Implicit_Keys_emit() {
    test_emit("./yaml-test-suite/L9U5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LE5A_Spec_Example_7_24_Flow_Nodes_events() {
    test_events("./yaml-test-suite/LE5A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LE5A_Spec_Example_7_24_Flow_Nodes_json() {
    test_json("./yaml-test-suite/LE5A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LE5A_Spec_Example_7_24_Flow_Nodes_out() {
    test_out("./yaml-test-suite/LE5A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LE5A_Spec_Example_7_24_Flow_Nodes_emit() {
    test_emit("./yaml-test-suite/LE5A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LHL4_Invalid_tag_error() {
    test_error("./yaml-test-suite/LHL4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LHL4_Invalid_tag_events() {
    test_events("./yaml-test-suite/LHL4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LP6E_Whitespace_After_Scalars_in_Flow_events() {
    test_events("./yaml-test-suite/LP6E");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LP6E_Whitespace_After_Scalars_in_Flow_json() {
    test_json("./yaml-test-suite/LP6E");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LP6E_Whitespace_After_Scalars_in_Flow_out() {
    test_out("./yaml-test-suite/LP6E");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LP6E_Whitespace_After_Scalars_in_Flow_emit() {
    test_emit("./yaml-test-suite/LP6E");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LQZ7_Spec_Example_7_4_Double_Quoted_Implicit_Keys_events() {
    test_events("./yaml-test-suite/LQZ7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LQZ7_Spec_Example_7_4_Double_Quoted_Implicit_Keys_json() {
    test_json("./yaml-test-suite/LQZ7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LQZ7_Spec_Example_7_4_Double_Quoted_Implicit_Keys_out() {
    test_out("./yaml-test-suite/LQZ7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LQZ7_Spec_Example_7_4_Double_Quoted_Implicit_Keys_emit() {
    test_emit("./yaml-test-suite/LQZ7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LX3P_Implicit_Flow_Mapping_Key_on_one_line_events() {
    test_events("./yaml-test-suite/LX3P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LX3P_Implicit_Flow_Mapping_Key_on_one_line_json() {
    test_json("./yaml-test-suite/LX3P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LX3P_Implicit_Flow_Mapping_Key_on_one_line_out() {
    test_out("./yaml-test-suite/LX3P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_LX3P_Implicit_Flow_Mapping_Key_on_one_line_emit() {
    test_emit("./yaml-test-suite/LX3P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M29M_Literal_Block_Scalar_events() {
    test_events("./yaml-test-suite/M29M");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M29M_Literal_Block_Scalar_json() {
    test_json("./yaml-test-suite/M29M");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M29M_Literal_Block_Scalar_out() {
    test_out("./yaml-test-suite/M29M");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M29M_Literal_Block_Scalar_emit() {
    test_emit("./yaml-test-suite/M29M");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M2N8_00_Question_mark_edge_cases_events() {
    test_events("./yaml-test-suite/M2N8/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M2N8_00_Question_mark_edge_cases_json() {
    test_json("./yaml-test-suite/M2N8/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M2N8_00_Question_mark_edge_cases_out() {
    test_out("./yaml-test-suite/M2N8/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M2N8_00_Question_mark_edge_cases_emit() {
    test_emit("./yaml-test-suite/M2N8/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M2N8_01_Question_mark_edge_cases_events() {
    test_events("./yaml-test-suite/M2N8/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M2N8_01_Question_mark_edge_cases_json() {
    test_json("./yaml-test-suite/M2N8/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M2N8_01_Question_mark_edge_cases_out() {
    test_out("./yaml-test-suite/M2N8/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M2N8_01_Question_mark_edge_cases_emit() {
    test_emit("./yaml-test-suite/M2N8/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M5C3_Spec_Example_8_21_Block_Scalar_Nodes_events() {
    test_events("./yaml-test-suite/M5C3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M5C3_Spec_Example_8_21_Block_Scalar_Nodes_json() {
    test_json("./yaml-test-suite/M5C3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M5C3_Spec_Example_8_21_Block_Scalar_Nodes_out() {
    test_out("./yaml-test-suite/M5C3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M5C3_Spec_Example_8_21_Block_Scalar_Nodes_emit() {
    test_emit("./yaml-test-suite/M5C3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M5DY_Spec_Example_2_11_Mapping_between_Sequences_events() {
    test_events("./yaml-test-suite/M5DY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M5DY_Spec_Example_2_11_Mapping_between_Sequences_json() {
    test_json("./yaml-test-suite/M5DY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M5DY_Spec_Example_2_11_Mapping_between_Sequences_out() {
    test_out("./yaml-test-suite/M5DY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M5DY_Spec_Example_2_11_Mapping_between_Sequences_emit() {
    test_emit("./yaml-test-suite/M5DY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M6YH_Block_sequence_indentation_events() {
    test_events("./yaml-test-suite/M6YH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M6YH_Block_sequence_indentation_json() {
    test_json("./yaml-test-suite/M6YH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M6YH_Block_sequence_indentation_out() {
    test_out("./yaml-test-suite/M6YH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M6YH_Block_sequence_indentation_emit() {
    test_emit("./yaml-test-suite/M6YH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M7A3_Spec_Example_9_3_Bare_Documents_events() {
    test_events("./yaml-test-suite/M7A3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M7A3_Spec_Example_9_3_Bare_Documents_json() {
    test_json("./yaml-test-suite/M7A3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M7A3_Spec_Example_9_3_Bare_Documents_out() {
    test_out("./yaml-test-suite/M7A3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M7A3_Spec_Example_9_3_Bare_Documents_emit() {
    test_emit("./yaml-test-suite/M7A3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M7NX_Nested_flow_collections_events() {
    test_events("./yaml-test-suite/M7NX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M7NX_Nested_flow_collections_json() {
    test_json("./yaml-test-suite/M7NX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M7NX_Nested_flow_collections_out() {
    test_out("./yaml-test-suite/M7NX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M7NX_Nested_flow_collections_emit() {
    test_emit("./yaml-test-suite/M7NX");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M9B4_Spec_Example_8_7_Literal_Scalar_events() {
    test_events("./yaml-test-suite/M9B4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M9B4_Spec_Example_8_7_Literal_Scalar_json() {
    test_json("./yaml-test-suite/M9B4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M9B4_Spec_Example_8_7_Literal_Scalar_out() {
    test_out("./yaml-test-suite/M9B4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_M9B4_Spec_Example_8_7_Literal_Scalar_emit() {
    test_emit("./yaml-test-suite/M9B4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MJS9_Spec_Example_6_7_Block_Folding_events() {
    test_events("./yaml-test-suite/MJS9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MJS9_Spec_Example_6_7_Block_Folding_json() {
    test_json("./yaml-test-suite/MJS9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MJS9_Spec_Example_6_7_Block_Folding_out() {
    test_out("./yaml-test-suite/MJS9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MJS9_Spec_Example_6_7_Block_Folding_emit() {
    test_emit("./yaml-test-suite/MJS9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_00_Directive_variants_error() {
    test_error("./yaml-test-suite/MUS6/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_00_Directive_variants_events() {
    test_events("./yaml-test-suite/MUS6/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_01_Directive_variants_error() {
    test_error("./yaml-test-suite/MUS6/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_01_Directive_variants_events() {
    test_events("./yaml-test-suite/MUS6/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_02_Directive_variants_events() {
    test_events("./yaml-test-suite/MUS6/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_02_Directive_variants_json() {
    test_json("./yaml-test-suite/MUS6/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_02_Directive_variants_out() {
    test_out("./yaml-test-suite/MUS6/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_02_Directive_variants_emit() {
    test_emit("./yaml-test-suite/MUS6/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_03_Directive_variants_events() {
    test_events("./yaml-test-suite/MUS6/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_03_Directive_variants_json() {
    test_json("./yaml-test-suite/MUS6/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_03_Directive_variants_out() {
    test_out("./yaml-test-suite/MUS6/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_03_Directive_variants_emit() {
    test_emit("./yaml-test-suite/MUS6/03");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_04_Directive_variants_events() {
    test_events("./yaml-test-suite/MUS6/04");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_04_Directive_variants_json() {
    test_json("./yaml-test-suite/MUS6/04");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_04_Directive_variants_out() {
    test_out("./yaml-test-suite/MUS6/04");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_04_Directive_variants_emit() {
    test_emit("./yaml-test-suite/MUS6/04");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_05_Directive_variants_events() {
    test_events("./yaml-test-suite/MUS6/05");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_05_Directive_variants_json() {
    test_json("./yaml-test-suite/MUS6/05");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_05_Directive_variants_out() {
    test_out("./yaml-test-suite/MUS6/05");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_05_Directive_variants_emit() {
    test_emit("./yaml-test-suite/MUS6/05");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_06_Directive_variants_events() {
    test_events("./yaml-test-suite/MUS6/06");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_06_Directive_variants_json() {
    test_json("./yaml-test-suite/MUS6/06");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_06_Directive_variants_out() {
    test_out("./yaml-test-suite/MUS6/06");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MUS6_06_Directive_variants_emit() {
    test_emit("./yaml-test-suite/MUS6/06");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MXS3_Flow_Mapping_in_Block_Sequence_events() {
    test_events("./yaml-test-suite/MXS3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MXS3_Flow_Mapping_in_Block_Sequence_json() {
    test_json("./yaml-test-suite/MXS3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MXS3_Flow_Mapping_in_Block_Sequence_out() {
    test_out("./yaml-test-suite/MXS3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MXS3_Flow_Mapping_in_Block_Sequence_emit() {
    test_emit("./yaml-test-suite/MXS3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MYW6_Block_Scalar_Strip_events() {
    test_events("./yaml-test-suite/MYW6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MYW6_Block_Scalar_Strip_json() {
    test_json("./yaml-test-suite/MYW6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MYW6_Block_Scalar_Strip_out() {
    test_out("./yaml-test-suite/MYW6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MYW6_Block_Scalar_Strip_emit() {
    test_emit("./yaml-test-suite/MYW6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MZX3_Non_Specific_Tags_on_Scalars_events() {
    test_events("./yaml-test-suite/MZX3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MZX3_Non_Specific_Tags_on_Scalars_json() {
    test_json("./yaml-test-suite/MZX3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MZX3_Non_Specific_Tags_on_Scalars_out() {
    test_out("./yaml-test-suite/MZX3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_MZX3_Non_Specific_Tags_on_Scalars_emit() {
    test_emit("./yaml-test-suite/MZX3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_N4JP_Bad_indentation_in_mapping_error() {
    test_error("./yaml-test-suite/N4JP");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_N4JP_Bad_indentation_in_mapping_events() {
    test_events("./yaml-test-suite/N4JP");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_N782_Invalid_document_markers_in_flow_style_error() {
    test_error("./yaml-test-suite/N782");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_N782_Invalid_document_markers_in_flow_style_events() {
    test_events("./yaml-test-suite/N782");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NAT4_Various_empty_or_newline_only_quoted_strings_events() {
    test_events("./yaml-test-suite/NAT4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NAT4_Various_empty_or_newline_only_quoted_strings_json() {
    test_json("./yaml-test-suite/NAT4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NAT4_Various_empty_or_newline_only_quoted_strings_out() {
    test_out("./yaml-test-suite/NAT4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NAT4_Various_empty_or_newline_only_quoted_strings_emit() {
    test_emit("./yaml-test-suite/NAT4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NB6Z_Multiline_plain_value_with_tabs_on_empty_lines_events() {
    test_events("./yaml-test-suite/NB6Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NB6Z_Multiline_plain_value_with_tabs_on_empty_lines_json() {
    test_json("./yaml-test-suite/NB6Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NB6Z_Multiline_plain_value_with_tabs_on_empty_lines_out() {
    test_out("./yaml-test-suite/NB6Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NB6Z_Multiline_plain_value_with_tabs_on_empty_lines_emit() {
    test_emit("./yaml-test-suite/NB6Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NHX8_Empty_Lines_at_End_of_Document_events() {
    test_events("./yaml-test-suite/NHX8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NHX8_Empty_Lines_at_End_of_Document_json() {
    test_json("./yaml-test-suite/NHX8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NHX8_Empty_Lines_at_End_of_Document_out() {
    test_out("./yaml-test-suite/NHX8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NHX8_Empty_Lines_at_End_of_Document_emit() {
    test_emit("./yaml-test-suite/NHX8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NJ66_Multiline_plain_flow_mapping_key_events() {
    test_events("./yaml-test-suite/NJ66");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NJ66_Multiline_plain_flow_mapping_key_json() {
    test_json("./yaml-test-suite/NJ66");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NJ66_Multiline_plain_flow_mapping_key_out() {
    test_out("./yaml-test-suite/NJ66");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NJ66_Multiline_plain_flow_mapping_key_emit() {
    test_emit("./yaml-test-suite/NJ66");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NKF9_Empty_keys_in_block_and_flow_mapping_events() {
    test_events("./yaml-test-suite/NKF9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NKF9_Empty_keys_in_block_and_flow_mapping_json() {
    test_json("./yaml-test-suite/NKF9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NKF9_Empty_keys_in_block_and_flow_mapping_out() {
    test_out("./yaml-test-suite/NKF9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NKF9_Empty_keys_in_block_and_flow_mapping_emit() {
    test_emit("./yaml-test-suite/NKF9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NP9H_Spec_Example_7_5_Double_Quoted_Line_Breaks_events() {
    test_events("./yaml-test-suite/NP9H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NP9H_Spec_Example_7_5_Double_Quoted_Line_Breaks_json() {
    test_json("./yaml-test-suite/NP9H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NP9H_Spec_Example_7_5_Double_Quoted_Line_Breaks_out() {
    test_out("./yaml-test-suite/NP9H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_NP9H_Spec_Example_7_5_Double_Quoted_Line_Breaks_emit() {
    test_emit("./yaml-test-suite/NP9H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_P2AD_Spec_Example_8_1_Block_Scalar_Header_events() {
    test_events("./yaml-test-suite/P2AD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_P2AD_Spec_Example_8_1_Block_Scalar_Header_json() {
    test_json("./yaml-test-suite/P2AD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_P2AD_Spec_Example_8_1_Block_Scalar_Header_out() {
    test_out("./yaml-test-suite/P2AD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_P2AD_Spec_Example_8_1_Block_Scalar_Header_emit() {
    test_emit("./yaml-test-suite/P2AD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_P2EQ_Invalid_sequene_item_on_same_line_as_previous_item_error() {
    test_error("./yaml-test-suite/P2EQ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_P2EQ_Invalid_sequene_item_on_same_line_as_previous_item_events() {
    test_events("./yaml-test-suite/P2EQ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_P76L_Spec_Example_6_19_Secondary_Tag_Handle_events() {
    test_events("./yaml-test-suite/P76L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_P76L_Spec_Example_6_19_Secondary_Tag_Handle_json() {
    test_json("./yaml-test-suite/P76L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_P76L_Spec_Example_6_19_Secondary_Tag_Handle_out() {
    test_out("./yaml-test-suite/P76L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_P76L_Spec_Example_6_19_Secondary_Tag_Handle_emit() {
    test_emit("./yaml-test-suite/P76L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_P94K_Spec_Example_6_11_Multi_Line_Comments_events() {
    test_events("./yaml-test-suite/P94K");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_P94K_Spec_Example_6_11_Multi_Line_Comments_json() {
    test_json("./yaml-test-suite/P94K");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_P94K_Spec_Example_6_11_Multi_Line_Comments_out() {
    test_out("./yaml-test-suite/P94K");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_P94K_Spec_Example_6_11_Multi_Line_Comments_emit() {
    test_emit("./yaml-test-suite/P94K");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_PBJ2_Spec_Example_2_3_Mapping_Scalars_to_Sequences_events() {
    test_events("./yaml-test-suite/PBJ2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_PBJ2_Spec_Example_2_3_Mapping_Scalars_to_Sequences_json() {
    test_json("./yaml-test-suite/PBJ2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_PBJ2_Spec_Example_2_3_Mapping_Scalars_to_Sequences_out() {
    test_out("./yaml-test-suite/PBJ2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_PBJ2_Spec_Example_2_3_Mapping_Scalars_to_Sequences_emit() {
    test_emit("./yaml-test-suite/PBJ2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_PRH3_Spec_Example_7_9_Single_Quoted_Lines_events() {
    test_events("./yaml-test-suite/PRH3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_PRH3_Spec_Example_7_9_Single_Quoted_Lines_json() {
    test_json("./yaml-test-suite/PRH3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_PRH3_Spec_Example_7_9_Single_Quoted_Lines_out() {
    test_out("./yaml-test-suite/PRH3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_PRH3_Spec_Example_7_9_Single_Quoted_Lines_emit() {
    test_emit("./yaml-test-suite/PRH3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_PUW8_Document_start_on_last_line_events() {
    test_events("./yaml-test-suite/PUW8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_PUW8_Document_start_on_last_line_json() {
    test_json("./yaml-test-suite/PUW8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_PUW8_Document_start_on_last_line_out() {
    test_out("./yaml-test-suite/PUW8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_PUW8_Document_start_on_last_line_emit() {
    test_emit("./yaml-test-suite/PUW8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_PW8X_Anchors_on_Empty_Scalars_events() {
    test_events("./yaml-test-suite/PW8X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_PW8X_Anchors_on_Empty_Scalars_json() {
    test_json("./yaml-test-suite/PW8X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_PW8X_Anchors_on_Empty_Scalars_out() {
    test_out("./yaml-test-suite/PW8X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_PW8X_Anchors_on_Empty_Scalars_emit() {
    test_emit("./yaml-test-suite/PW8X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q4CL_Trailing_content_after_quoted_value_error() {
    test_error("./yaml-test-suite/Q4CL");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q4CL_Trailing_content_after_quoted_value_events() {
    test_events("./yaml-test-suite/Q4CL");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q5MG_Tab_at_beginning_of_line_followed_by_a_flow_mapping_events() {
    test_events("./yaml-test-suite/Q5MG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q5MG_Tab_at_beginning_of_line_followed_by_a_flow_mapping_json() {
    test_json("./yaml-test-suite/Q5MG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q5MG_Tab_at_beginning_of_line_followed_by_a_flow_mapping_out() {
    test_out("./yaml-test-suite/Q5MG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q5MG_Tab_at_beginning_of_line_followed_by_a_flow_mapping_emit() {
    test_emit("./yaml-test-suite/Q5MG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q88A_Spec_Example_7_23_Flow_Content_events() {
    test_events("./yaml-test-suite/Q88A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q88A_Spec_Example_7_23_Flow_Content_json() {
    test_json("./yaml-test-suite/Q88A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q88A_Spec_Example_7_23_Flow_Content_out() {
    test_out("./yaml-test-suite/Q88A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q88A_Spec_Example_7_23_Flow_Content_emit() {
    test_emit("./yaml-test-suite/Q88A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q8AD_Spec_Example_7_5_Double_Quoted_Line_Breaks_1_3_events() {
    test_events("./yaml-test-suite/Q8AD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q8AD_Spec_Example_7_5_Double_Quoted_Line_Breaks_1_3_json() {
    test_json("./yaml-test-suite/Q8AD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q8AD_Spec_Example_7_5_Double_Quoted_Line_Breaks_1_3_out() {
    test_out("./yaml-test-suite/Q8AD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q8AD_Spec_Example_7_5_Double_Quoted_Line_Breaks_1_3_emit() {
    test_emit("./yaml-test-suite/Q8AD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q9WF_Spec_Example_6_12_Separation_Spaces_events() {
    test_events("./yaml-test-suite/Q9WF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q9WF_Spec_Example_6_12_Separation_Spaces_json() {
    test_json("./yaml-test-suite/Q9WF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q9WF_Spec_Example_6_12_Separation_Spaces_out() {
    test_out("./yaml-test-suite/Q9WF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Q9WF_Spec_Example_6_12_Separation_Spaces_emit() {
    test_emit("./yaml-test-suite/Q9WF");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_QB6E_Wrong_indented_multiline_quoted_scalar_error() {
    test_error("./yaml-test-suite/QB6E");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_QB6E_Wrong_indented_multiline_quoted_scalar_events() {
    test_events("./yaml-test-suite/QB6E");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_QF4Y_Spec_Example_7_19_Single_Pair_Flow_Mappings_events() {
    test_events("./yaml-test-suite/QF4Y");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_QF4Y_Spec_Example_7_19_Single_Pair_Flow_Mappings_json() {
    test_json("./yaml-test-suite/QF4Y");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_QF4Y_Spec_Example_7_19_Single_Pair_Flow_Mappings_out() {
    test_out("./yaml-test-suite/QF4Y");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_QF4Y_Spec_Example_7_19_Single_Pair_Flow_Mappings_emit() {
    test_emit("./yaml-test-suite/QF4Y");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_QLJ7_Tag_shorthand_used_in_documents_but_only_defined_in_the_first_error() {
    test_error("./yaml-test-suite/QLJ7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_QLJ7_Tag_shorthand_used_in_documents_but_only_defined_in_the_first_events() {
    test_events("./yaml-test-suite/QLJ7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_QT73_Comment_and_document_end_marker_events() {
    test_events("./yaml-test-suite/QT73");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_QT73_Comment_and_document_end_marker_json() {
    test_json("./yaml-test-suite/QT73");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_QT73_Comment_and_document_end_marker_out() {
    test_out("./yaml-test-suite/QT73");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_QT73_Comment_and_document_end_marker_emit() {
    test_emit("./yaml-test-suite/QT73");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_R4YG_Spec_Example_8_2_Block_Indentation_Indicator_events() {
    test_events("./yaml-test-suite/R4YG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_R4YG_Spec_Example_8_2_Block_Indentation_Indicator_json() {
    test_json("./yaml-test-suite/R4YG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_R4YG_Spec_Example_8_2_Block_Indentation_Indicator_out() {
    test_out("./yaml-test-suite/R4YG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_R4YG_Spec_Example_8_2_Block_Indentation_Indicator_emit() {
    test_emit("./yaml-test-suite/R4YG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_R52L_Nested_flow_mapping_sequence_and_mappings_events() {
    test_events("./yaml-test-suite/R52L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_R52L_Nested_flow_mapping_sequence_and_mappings_json() {
    test_json("./yaml-test-suite/R52L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_R52L_Nested_flow_mapping_sequence_and_mappings_out() {
    test_out("./yaml-test-suite/R52L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_R52L_Nested_flow_mapping_sequence_and_mappings_emit() {
    test_emit("./yaml-test-suite/R52L");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RHX7_YAML_directive_without_document_end_marker_error() {
    test_error("./yaml-test-suite/RHX7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RHX7_YAML_directive_without_document_end_marker_events() {
    test_events("./yaml-test-suite/RHX7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RLU9_Sequence_Indent_events() {
    test_events("./yaml-test-suite/RLU9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RLU9_Sequence_Indent_json() {
    test_json("./yaml-test-suite/RLU9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RLU9_Sequence_Indent_out() {
    test_out("./yaml-test-suite/RLU9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RLU9_Sequence_Indent_emit() {
    test_emit("./yaml-test-suite/RLU9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RR7F_Mixed_Block_Mapping_implicit_to_explicit_events() {
    test_events("./yaml-test-suite/RR7F");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RR7F_Mixed_Block_Mapping_implicit_to_explicit_json() {
    test_json("./yaml-test-suite/RR7F");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RR7F_Mixed_Block_Mapping_implicit_to_explicit_out() {
    test_out("./yaml-test-suite/RR7F");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RR7F_Mixed_Block_Mapping_implicit_to_explicit_emit() {
    test_emit("./yaml-test-suite/RR7F");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RTP8_Spec_Example_9_2_Document_Markers_events() {
    test_events("./yaml-test-suite/RTP8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RTP8_Spec_Example_9_2_Document_Markers_json() {
    test_json("./yaml-test-suite/RTP8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RTP8_Spec_Example_9_2_Document_Markers_out() {
    test_out("./yaml-test-suite/RTP8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RTP8_Spec_Example_9_2_Document_Markers_emit() {
    test_emit("./yaml-test-suite/RTP8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RXY3_Invalid_document_end_marker_in_single_quoted_string_error() {
    test_error("./yaml-test-suite/RXY3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RXY3_Invalid_document_end_marker_in_single_quoted_string_events() {
    test_events("./yaml-test-suite/RXY3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RZP5_Various_Trailing_Comments_1_3_events() {
    test_events("./yaml-test-suite/RZP5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RZP5_Various_Trailing_Comments_1_3_json() {
    test_json("./yaml-test-suite/RZP5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RZP5_Various_Trailing_Comments_1_3_out() {
    test_out("./yaml-test-suite/RZP5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RZP5_Various_Trailing_Comments_1_3_emit() {
    test_emit("./yaml-test-suite/RZP5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RZT7_Spec_Example_2_28_Log_File_events() {
    test_events("./yaml-test-suite/RZT7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RZT7_Spec_Example_2_28_Log_File_json() {
    test_json("./yaml-test-suite/RZT7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RZT7_Spec_Example_2_28_Log_File_out() {
    test_out("./yaml-test-suite/RZT7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_RZT7_Spec_Example_2_28_Log_File_emit() {
    test_emit("./yaml-test-suite/RZT7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S3PD_Spec_Example_8_18_Implicit_Block_Mapping_Entries_events() {
    test_events("./yaml-test-suite/S3PD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S3PD_Spec_Example_8_18_Implicit_Block_Mapping_Entries_json() {
    test_json("./yaml-test-suite/S3PD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S3PD_Spec_Example_8_18_Implicit_Block_Mapping_Entries_out() {
    test_out("./yaml-test-suite/S3PD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S3PD_Spec_Example_8_18_Implicit_Block_Mapping_Entries_emit() {
    test_emit("./yaml-test-suite/S3PD");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S4GJ_Invalid_text_after_block_scalar_indicator_error() {
    test_error("./yaml-test-suite/S4GJ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S4GJ_Invalid_text_after_block_scalar_indicator_events() {
    test_events("./yaml-test-suite/S4GJ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S4JQ_Spec_Example_6_28_Non_Specific_Tags_events() {
    test_events("./yaml-test-suite/S4JQ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S4JQ_Spec_Example_6_28_Non_Specific_Tags_json() {
    test_json("./yaml-test-suite/S4JQ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S4JQ_Spec_Example_6_28_Non_Specific_Tags_out() {
    test_out("./yaml-test-suite/S4JQ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S4JQ_Spec_Example_6_28_Non_Specific_Tags_emit() {
    test_emit("./yaml-test-suite/S4JQ");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S4T7_Document_with_footer_events() {
    test_events("./yaml-test-suite/S4T7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S4T7_Document_with_footer_json() {
    test_json("./yaml-test-suite/S4T7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S4T7_Document_with_footer_out() {
    test_out("./yaml-test-suite/S4T7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S4T7_Document_with_footer_emit() {
    test_emit("./yaml-test-suite/S4T7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S7BG_Colon_followed_by_comma_events() {
    test_events("./yaml-test-suite/S7BG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S7BG_Colon_followed_by_comma_json() {
    test_json("./yaml-test-suite/S7BG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S7BG_Colon_followed_by_comma_out() {
    test_out("./yaml-test-suite/S7BG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S7BG_Colon_followed_by_comma_emit() {
    test_emit("./yaml-test-suite/S7BG");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S98Z_Block_scalar_with_more_spaces_than_first_content_line_error() {
    test_error("./yaml-test-suite/S98Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S98Z_Block_scalar_with_more_spaces_than_first_content_line_events() {
    test_events("./yaml-test-suite/S98Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S9E8_Spec_Example_5_3_Block_Structure_Indicators_events() {
    test_events("./yaml-test-suite/S9E8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S9E8_Spec_Example_5_3_Block_Structure_Indicators_json() {
    test_json("./yaml-test-suite/S9E8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S9E8_Spec_Example_5_3_Block_Structure_Indicators_out() {
    test_out("./yaml-test-suite/S9E8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_S9E8_Spec_Example_5_3_Block_Structure_Indicators_emit() {
    test_emit("./yaml-test-suite/S9E8");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SBG9_Flow_Sequence_in_Flow_Mapping_events() {
    test_events("./yaml-test-suite/SBG9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SBG9_Flow_Sequence_in_Flow_Mapping_json() {
    test_json("./yaml-test-suite/SBG9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SBG9_Flow_Sequence_in_Flow_Mapping_out() {
    test_out("./yaml-test-suite/SBG9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SBG9_Flow_Sequence_in_Flow_Mapping_emit() {
    test_emit("./yaml-test-suite/SBG9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SF5V_Duplicate_YAML_directive_error() {
    test_error("./yaml-test-suite/SF5V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SF5V_Duplicate_YAML_directive_events() {
    test_events("./yaml-test-suite/SF5V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SKE5_Anchor_before_zero_indented_sequence_events() {
    test_events("./yaml-test-suite/SKE5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SKE5_Anchor_before_zero_indented_sequence_json() {
    test_json("./yaml-test-suite/SKE5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SKE5_Anchor_before_zero_indented_sequence_out() {
    test_out("./yaml-test-suite/SKE5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SKE5_Anchor_before_zero_indented_sequence_emit() {
    test_emit("./yaml-test-suite/SKE5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SM9W_00_Single_character_streams_events() {
    test_events("./yaml-test-suite/SM9W/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SM9W_00_Single_character_streams_json() {
    test_json("./yaml-test-suite/SM9W/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SM9W_00_Single_character_streams_out() {
    test_out("./yaml-test-suite/SM9W/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SM9W_00_Single_character_streams_emit() {
    test_emit("./yaml-test-suite/SM9W/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SM9W_01_Single_character_streams_events() {
    test_events("./yaml-test-suite/SM9W/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SM9W_01_Single_character_streams_json() {
    test_json("./yaml-test-suite/SM9W/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SM9W_01_Single_character_streams_out() {
    test_out("./yaml-test-suite/SM9W/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SM9W_01_Single_character_streams_emit() {
    test_emit("./yaml-test-suite/SM9W/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SR86_Anchor_plus_Alias_error() {
    test_error("./yaml-test-suite/SR86");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SR86_Anchor_plus_Alias_events() {
    test_events("./yaml-test-suite/SR86");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SSW6_Spec_Example_7_7_Single_Quoted_Characters_1_3_events() {
    test_events("./yaml-test-suite/SSW6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SSW6_Spec_Example_7_7_Single_Quoted_Characters_1_3_json() {
    test_json("./yaml-test-suite/SSW6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SSW6_Spec_Example_7_7_Single_Quoted_Characters_1_3_out() {
    test_out("./yaml-test-suite/SSW6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SSW6_Spec_Example_7_7_Single_Quoted_Characters_1_3_emit() {
    test_emit("./yaml-test-suite/SSW6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SU5Z_Comment_without_whitespace_after_doublequoted_scalar_error() {
    test_error("./yaml-test-suite/SU5Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SU5Z_Comment_without_whitespace_after_doublequoted_scalar_events() {
    test_events("./yaml-test-suite/SU5Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SU74_Anchor_and_alias_as_mapping_key_error() {
    test_error("./yaml-test-suite/SU74");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SU74_Anchor_and_alias_as_mapping_key_events() {
    test_events("./yaml-test-suite/SU74");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SY6V_Anchor_before_sequence_entry_on_same_line_error() {
    test_error("./yaml-test-suite/SY6V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SY6V_Anchor_before_sequence_entry_on_same_line_events() {
    test_events("./yaml-test-suite/SY6V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SYW4_Spec_Example_2_2_Mapping_Scalars_to_Scalars_events() {
    test_events("./yaml-test-suite/SYW4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SYW4_Spec_Example_2_2_Mapping_Scalars_to_Scalars_json() {
    test_json("./yaml-test-suite/SYW4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SYW4_Spec_Example_2_2_Mapping_Scalars_to_Scalars_out() {
    test_out("./yaml-test-suite/SYW4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_SYW4_Spec_Example_2_2_Mapping_Scalars_to_Scalars_emit() {
    test_emit("./yaml-test-suite/SYW4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_T26H_Spec_Example_8_8_Literal_Content_1_3_events() {
    test_events("./yaml-test-suite/T26H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_T26H_Spec_Example_8_8_Literal_Content_1_3_json() {
    test_json("./yaml-test-suite/T26H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_T26H_Spec_Example_8_8_Literal_Content_1_3_out() {
    test_out("./yaml-test-suite/T26H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_T26H_Spec_Example_8_8_Literal_Content_1_3_emit() {
    test_emit("./yaml-test-suite/T26H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_T4YY_Spec_Example_7_9_Single_Quoted_Lines_1_3_events() {
    test_events("./yaml-test-suite/T4YY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_T4YY_Spec_Example_7_9_Single_Quoted_Lines_1_3_json() {
    test_json("./yaml-test-suite/T4YY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_T4YY_Spec_Example_7_9_Single_Quoted_Lines_1_3_out() {
    test_out("./yaml-test-suite/T4YY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_T4YY_Spec_Example_7_9_Single_Quoted_Lines_1_3_emit() {
    test_emit("./yaml-test-suite/T4YY");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_T5N4_Spec_Example_8_7_Literal_Scalar_1_3_events() {
    test_events("./yaml-test-suite/T5N4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_T5N4_Spec_Example_8_7_Literal_Scalar_1_3_json() {
    test_json("./yaml-test-suite/T5N4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_T5N4_Spec_Example_8_7_Literal_Scalar_1_3_out() {
    test_out("./yaml-test-suite/T5N4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_T5N4_Spec_Example_8_7_Literal_Scalar_1_3_emit() {
    test_emit("./yaml-test-suite/T5N4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_T833_Flow_mapping_missing_a_separating_comma_error() {
    test_error("./yaml-test-suite/T833");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_T833_Flow_mapping_missing_a_separating_comma_events() {
    test_events("./yaml-test-suite/T833");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_TD5N_Invalid_scalar_after_sequence_error() {
    test_error("./yaml-test-suite/TD5N");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_TD5N_Invalid_scalar_after_sequence_events() {
    test_events("./yaml-test-suite/TD5N");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_TE2A_Spec_Example_8_16_Block_Mappings_events() {
    test_events("./yaml-test-suite/TE2A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_TE2A_Spec_Example_8_16_Block_Mappings_json() {
    test_json("./yaml-test-suite/TE2A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_TE2A_Spec_Example_8_16_Block_Mappings_out() {
    test_out("./yaml-test-suite/TE2A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_TE2A_Spec_Example_8_16_Block_Mappings_emit() {
    test_emit("./yaml-test-suite/TE2A");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_TL85_Spec_Example_6_8_Flow_Folding_events() {
    test_events("./yaml-test-suite/TL85");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_TL85_Spec_Example_6_8_Flow_Folding_json() {
    test_json("./yaml-test-suite/TL85");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_TL85_Spec_Example_6_8_Flow_Folding_out() {
    test_out("./yaml-test-suite/TL85");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_TL85_Spec_Example_6_8_Flow_Folding_emit() {
    test_emit("./yaml-test-suite/TL85");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_TS54_Folded_Block_Scalar_events() {
    test_events("./yaml-test-suite/TS54");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_TS54_Folded_Block_Scalar_json() {
    test_json("./yaml-test-suite/TS54");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_TS54_Folded_Block_Scalar_out() {
    test_out("./yaml-test-suite/TS54");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_TS54_Folded_Block_Scalar_emit() {
    test_emit("./yaml-test-suite/TS54");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_U3C3_Spec_Example_6_16_TAG_directive_events() {
    test_events("./yaml-test-suite/U3C3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_U3C3_Spec_Example_6_16_TAG_directive_json() {
    test_json("./yaml-test-suite/U3C3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_U3C3_Spec_Example_6_16_TAG_directive_out() {
    test_out("./yaml-test-suite/U3C3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_U3C3_Spec_Example_6_16_TAG_directive_emit() {
    test_emit("./yaml-test-suite/U3C3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_U3XV_Node_and_Mapping_Key_Anchors_events() {
    test_events("./yaml-test-suite/U3XV");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_U3XV_Node_and_Mapping_Key_Anchors_json() {
    test_json("./yaml-test-suite/U3XV");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_U3XV_Node_and_Mapping_Key_Anchors_out() {
    test_out("./yaml-test-suite/U3XV");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_U3XV_Node_and_Mapping_Key_Anchors_emit() {
    test_emit("./yaml-test-suite/U3XV");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_U44R_Bad_indentation_in_mapping_2_error() {
    test_error("./yaml-test-suite/U44R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_U44R_Bad_indentation_in_mapping_2_events() {
    test_events("./yaml-test-suite/U44R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_U99R_Invalid_comma_in_tag_error() {
    test_error("./yaml-test-suite/U99R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_U99R_Invalid_comma_in_tag_events() {
    test_events("./yaml-test-suite/U99R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_U9NS_Spec_Example_2_8_Play_by_Play_Feed_from_a_Game_events() {
    test_events("./yaml-test-suite/U9NS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_U9NS_Spec_Example_2_8_Play_by_Play_Feed_from_a_Game_json() {
    test_json("./yaml-test-suite/U9NS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_U9NS_Spec_Example_2_8_Play_by_Play_Feed_from_a_Game_out() {
    test_out("./yaml-test-suite/U9NS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_U9NS_Spec_Example_2_8_Play_by_Play_Feed_from_a_Game_emit() {
    test_emit("./yaml-test-suite/U9NS");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UDM2_Plain_URL_in_flow_mapping_events() {
    test_events("./yaml-test-suite/UDM2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UDM2_Plain_URL_in_flow_mapping_json() {
    test_json("./yaml-test-suite/UDM2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UDM2_Plain_URL_in_flow_mapping_out() {
    test_out("./yaml-test-suite/UDM2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UDM2_Plain_URL_in_flow_mapping_emit() {
    test_emit("./yaml-test-suite/UDM2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UDR7_Spec_Example_5_4_Flow_Collection_Indicators_events() {
    test_events("./yaml-test-suite/UDR7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UDR7_Spec_Example_5_4_Flow_Collection_Indicators_json() {
    test_json("./yaml-test-suite/UDR7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UDR7_Spec_Example_5_4_Flow_Collection_Indicators_out() {
    test_out("./yaml-test-suite/UDR7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UDR7_Spec_Example_5_4_Flow_Collection_Indicators_emit() {
    test_emit("./yaml-test-suite/UDR7");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UGM3_Spec_Example_2_27_Invoice_events() {
    test_events("./yaml-test-suite/UGM3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UGM3_Spec_Example_2_27_Invoice_json() {
    test_json("./yaml-test-suite/UGM3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UGM3_Spec_Example_2_27_Invoice_out() {
    test_out("./yaml-test-suite/UGM3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UGM3_Spec_Example_2_27_Invoice_emit() {
    test_emit("./yaml-test-suite/UGM3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UKK6_00_Syntax_character_edge_cases_events() {
    test_events("./yaml-test-suite/UKK6/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UKK6_00_Syntax_character_edge_cases_json() {
    test_json("./yaml-test-suite/UKK6/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UKK6_00_Syntax_character_edge_cases_out() {
    test_out("./yaml-test-suite/UKK6/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UKK6_00_Syntax_character_edge_cases_emit() {
    test_emit("./yaml-test-suite/UKK6/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UKK6_01_Syntax_character_edge_cases_events() {
    test_events("./yaml-test-suite/UKK6/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UKK6_01_Syntax_character_edge_cases_json() {
    test_json("./yaml-test-suite/UKK6/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UKK6_01_Syntax_character_edge_cases_out() {
    test_out("./yaml-test-suite/UKK6/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UKK6_01_Syntax_character_edge_cases_emit() {
    test_emit("./yaml-test-suite/UKK6/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UKK6_02_Syntax_character_edge_cases_events() {
    test_events("./yaml-test-suite/UKK6/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UKK6_02_Syntax_character_edge_cases_json() {
    test_json("./yaml-test-suite/UKK6/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UKK6_02_Syntax_character_edge_cases_out() {
    test_out("./yaml-test-suite/UKK6/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UKK6_02_Syntax_character_edge_cases_emit() {
    test_emit("./yaml-test-suite/UKK6/02");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UT92_Spec_Example_9_4_Explicit_Documents_events() {
    test_events("./yaml-test-suite/UT92");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UT92_Spec_Example_9_4_Explicit_Documents_json() {
    test_json("./yaml-test-suite/UT92");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UT92_Spec_Example_9_4_Explicit_Documents_out() {
    test_out("./yaml-test-suite/UT92");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UT92_Spec_Example_9_4_Explicit_Documents_emit() {
    test_emit("./yaml-test-suite/UT92");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UV7Q_Legal_tab_after_indentation_events() {
    test_events("./yaml-test-suite/UV7Q");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UV7Q_Legal_tab_after_indentation_json() {
    test_json("./yaml-test-suite/UV7Q");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UV7Q_Legal_tab_after_indentation_out() {
    test_out("./yaml-test-suite/UV7Q");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_UV7Q_Legal_tab_after_indentation_emit() {
    test_emit("./yaml-test-suite/UV7Q");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_V55R_Aliases_in_Block_Sequence_events() {
    test_events("./yaml-test-suite/V55R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_V55R_Aliases_in_Block_Sequence_json() {
    test_json("./yaml-test-suite/V55R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_V55R_Aliases_in_Block_Sequence_out() {
    test_out("./yaml-test-suite/V55R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_V55R_Aliases_in_Block_Sequence_emit() {
    test_emit("./yaml-test-suite/V55R");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_V9D5_Spec_Example_8_19_Compact_Block_Mappings_events() {
    test_events("./yaml-test-suite/V9D5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_V9D5_Spec_Example_8_19_Compact_Block_Mappings_json() {
    test_json("./yaml-test-suite/V9D5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_V9D5_Spec_Example_8_19_Compact_Block_Mappings_out() {
    test_out("./yaml-test-suite/V9D5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_V9D5_Spec_Example_8_19_Compact_Block_Mappings_emit() {
    test_emit("./yaml-test-suite/V9D5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_VJP3_00_Flow_collections_over_many_lines_error() {
    test_error("./yaml-test-suite/VJP3/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_VJP3_00_Flow_collections_over_many_lines_events() {
    test_events("./yaml-test-suite/VJP3/00");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_VJP3_01_Flow_collections_over_many_lines_events() {
    test_events("./yaml-test-suite/VJP3/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_VJP3_01_Flow_collections_over_many_lines_json() {
    test_json("./yaml-test-suite/VJP3/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_VJP3_01_Flow_collections_over_many_lines_out() {
    test_out("./yaml-test-suite/VJP3/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_VJP3_01_Flow_collections_over_many_lines_emit() {
    test_emit("./yaml-test-suite/VJP3/01");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_W42U_Spec_Example_8_15_Block_Sequence_Entry_Types_events() {
    test_events("./yaml-test-suite/W42U");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_W42U_Spec_Example_8_15_Block_Sequence_Entry_Types_json() {
    test_json("./yaml-test-suite/W42U");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_W42U_Spec_Example_8_15_Block_Sequence_Entry_Types_out() {
    test_out("./yaml-test-suite/W42U");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_W42U_Spec_Example_8_15_Block_Sequence_Entry_Types_emit() {
    test_emit("./yaml-test-suite/W42U");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_W4TN_Spec_Example_9_5_Directives_Documents_events() {
    test_events("./yaml-test-suite/W4TN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_W4TN_Spec_Example_9_5_Directives_Documents_json() {
    test_json("./yaml-test-suite/W4TN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_W4TN_Spec_Example_9_5_Directives_Documents_out() {
    test_out("./yaml-test-suite/W4TN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_W4TN_Spec_Example_9_5_Directives_Documents_emit() {
    test_emit("./yaml-test-suite/W4TN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_W5VH_Allowed_characters_in_alias_events() {
    test_events("./yaml-test-suite/W5VH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_W5VH_Allowed_characters_in_alias_json() {
    test_json("./yaml-test-suite/W5VH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_W5VH_Allowed_characters_in_alias_out() {
    test_out("./yaml-test-suite/W5VH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_W5VH_Allowed_characters_in_alias_emit() {
    test_emit("./yaml-test-suite/W5VH");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_W9L4_Literal_block_scalar_with_more_spaces_in_first_line_error() {
    test_error("./yaml-test-suite/W9L4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_W9L4_Literal_block_scalar_with_more_spaces_in_first_line_events() {
    test_events("./yaml-test-suite/W9L4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_WZ62_Spec_Example_7_2_Empty_Content_events() {
    test_events("./yaml-test-suite/WZ62");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_WZ62_Spec_Example_7_2_Empty_Content_json() {
    test_json("./yaml-test-suite/WZ62");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_WZ62_Spec_Example_7_2_Empty_Content_out() {
    test_out("./yaml-test-suite/WZ62");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_WZ62_Spec_Example_7_2_Empty_Content_emit() {
    test_emit("./yaml-test-suite/WZ62");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_X38W_Aliases_in_Flow_Objects_events() {
    test_events("./yaml-test-suite/X38W");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_X38W_Aliases_in_Flow_Objects_json() {
    test_json("./yaml-test-suite/X38W");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_X38W_Aliases_in_Flow_Objects_out() {
    test_out("./yaml-test-suite/X38W");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_X38W_Aliases_in_Flow_Objects_emit() {
    test_emit("./yaml-test-suite/X38W");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_X4QW_Comment_without_whitespace_after_block_scalar_indicator_error() {
    test_error("./yaml-test-suite/X4QW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_X4QW_Comment_without_whitespace_after_block_scalar_indicator_events() {
    test_events("./yaml-test-suite/X4QW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_X8DW_Explicit_key_and_value_seperated_by_comment_events() {
    test_events("./yaml-test-suite/X8DW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_X8DW_Explicit_key_and_value_seperated_by_comment_json() {
    test_json("./yaml-test-suite/X8DW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_X8DW_Explicit_key_and_value_seperated_by_comment_out() {
    test_out("./yaml-test-suite/X8DW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_X8DW_Explicit_key_and_value_seperated_by_comment_emit() {
    test_emit("./yaml-test-suite/X8DW");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_XLQ9_Multiline_scalar_that_looks_like_a_YAML_directive_events() {
    test_events("./yaml-test-suite/XLQ9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_XLQ9_Multiline_scalar_that_looks_like_a_YAML_directive_json() {
    test_json("./yaml-test-suite/XLQ9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_XLQ9_Multiline_scalar_that_looks_like_a_YAML_directive_out() {
    test_out("./yaml-test-suite/XLQ9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_XLQ9_Multiline_scalar_that_looks_like_a_YAML_directive_emit() {
    test_emit("./yaml-test-suite/XLQ9");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_XV9V_Spec_Example_6_5_Empty_Lines_1_3_events() {
    test_events("./yaml-test-suite/XV9V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_XV9V_Spec_Example_6_5_Empty_Lines_1_3_json() {
    test_json("./yaml-test-suite/XV9V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_XV9V_Spec_Example_6_5_Empty_Lines_1_3_out() {
    test_out("./yaml-test-suite/XV9V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_XV9V_Spec_Example_6_5_Empty_Lines_1_3_emit() {
    test_emit("./yaml-test-suite/XV9V");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_XW4D_Various_Trailing_Comments_events() {
    test_events("./yaml-test-suite/XW4D");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_XW4D_Various_Trailing_Comments_json() {
    test_json("./yaml-test-suite/XW4D");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_XW4D_Various_Trailing_Comments_out() {
    test_out("./yaml-test-suite/XW4D");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_XW4D_Various_Trailing_Comments_emit() {
    test_emit("./yaml-test-suite/XW4D");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y2GN_Anchor_with_colon_in_the_middle_events() {
    test_events("./yaml-test-suite/Y2GN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y2GN_Anchor_with_colon_in_the_middle_json() {
    test_json("./yaml-test-suite/Y2GN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y2GN_Anchor_with_colon_in_the_middle_out() {
    test_out("./yaml-test-suite/Y2GN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y2GN_Anchor_with_colon_in_the_middle_emit() {
    test_emit("./yaml-test-suite/Y2GN");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_000_Tabs_in_various_contexts_error() {
    test_error("./yaml-test-suite/Y79Y/000");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_000_Tabs_in_various_contexts_events() {
    test_events("./yaml-test-suite/Y79Y/000");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_001_Tabs_in_various_contexts_events() {
    test_events("./yaml-test-suite/Y79Y/001");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_001_Tabs_in_various_contexts_json() {
    test_json("./yaml-test-suite/Y79Y/001");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_001_Tabs_in_various_contexts_out() {
    test_out("./yaml-test-suite/Y79Y/001");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_001_Tabs_in_various_contexts_emit() {
    test_emit("./yaml-test-suite/Y79Y/001");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_002_Tabs_in_various_contexts_events() {
    test_events("./yaml-test-suite/Y79Y/002");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_002_Tabs_in_various_contexts_json() {
    test_json("./yaml-test-suite/Y79Y/002");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_002_Tabs_in_various_contexts_out() {
    test_out("./yaml-test-suite/Y79Y/002");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_002_Tabs_in_various_contexts_emit() {
    test_emit("./yaml-test-suite/Y79Y/002");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_003_Tabs_in_various_contexts_error() {
    test_error("./yaml-test-suite/Y79Y/003");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_003_Tabs_in_various_contexts_events() {
    test_events("./yaml-test-suite/Y79Y/003");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_004_Tabs_in_various_contexts_error() {
    test_error("./yaml-test-suite/Y79Y/004");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_004_Tabs_in_various_contexts_events() {
    test_events("./yaml-test-suite/Y79Y/004");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_005_Tabs_in_various_contexts_error() {
    test_error("./yaml-test-suite/Y79Y/005");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_005_Tabs_in_various_contexts_events() {
    test_events("./yaml-test-suite/Y79Y/005");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_006_Tabs_in_various_contexts_error() {
    test_error("./yaml-test-suite/Y79Y/006");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_006_Tabs_in_various_contexts_events() {
    test_events("./yaml-test-suite/Y79Y/006");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_007_Tabs_in_various_contexts_error() {
    test_error("./yaml-test-suite/Y79Y/007");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_007_Tabs_in_various_contexts_events() {
    test_events("./yaml-test-suite/Y79Y/007");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_008_Tabs_in_various_contexts_error() {
    test_error("./yaml-test-suite/Y79Y/008");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_008_Tabs_in_various_contexts_events() {
    test_events("./yaml-test-suite/Y79Y/008");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_009_Tabs_in_various_contexts_error() {
    test_error("./yaml-test-suite/Y79Y/009");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_009_Tabs_in_various_contexts_events() {
    test_events("./yaml-test-suite/Y79Y/009");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_010_Tabs_in_various_contexts_events() {
    test_events("./yaml-test-suite/Y79Y/010");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_010_Tabs_in_various_contexts_json() {
    test_json("./yaml-test-suite/Y79Y/010");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_010_Tabs_in_various_contexts_out() {
    test_out("./yaml-test-suite/Y79Y/010");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Y79Y_010_Tabs_in_various_contexts_emit() {
    test_emit("./yaml-test-suite/Y79Y/010");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_YD5X_Spec_Example_2_5_Sequence_of_Sequences_events() {
    test_events("./yaml-test-suite/YD5X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_YD5X_Spec_Example_2_5_Sequence_of_Sequences_json() {
    test_json("./yaml-test-suite/YD5X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_YD5X_Spec_Example_2_5_Sequence_of_Sequences_out() {
    test_out("./yaml-test-suite/YD5X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_YD5X_Spec_Example_2_5_Sequence_of_Sequences_emit() {
    test_emit("./yaml-test-suite/YD5X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_YJV2_Dash_in_flow_sequence_error() {
    test_error("./yaml-test-suite/YJV2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_YJV2_Dash_in_flow_sequence_events() {
    test_events("./yaml-test-suite/YJV2");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Z67P_Spec_Example_8_21_Block_Scalar_Nodes_1_3_events() {
    test_events("./yaml-test-suite/Z67P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Z67P_Spec_Example_8_21_Block_Scalar_Nodes_1_3_json() {
    test_json("./yaml-test-suite/Z67P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Z67P_Spec_Example_8_21_Block_Scalar_Nodes_1_3_out() {
    test_out("./yaml-test-suite/Z67P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Z67P_Spec_Example_8_21_Block_Scalar_Nodes_1_3_emit() {
    test_emit("./yaml-test-suite/Z67P");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Z9M4_Spec_Example_6_22_Global_Tag_Prefix_events() {
    test_events("./yaml-test-suite/Z9M4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Z9M4_Spec_Example_6_22_Global_Tag_Prefix_json() {
    test_json("./yaml-test-suite/Z9M4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Z9M4_Spec_Example_6_22_Global_Tag_Prefix_out() {
    test_out("./yaml-test-suite/Z9M4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_Z9M4_Spec_Example_6_22_Global_Tag_Prefix_emit() {
    test_emit("./yaml-test-suite/Z9M4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZCZ6_Invalid_mapping_in_plain_single_line_value_error() {
    test_error("./yaml-test-suite/ZCZ6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZCZ6_Invalid_mapping_in_plain_single_line_value_events() {
    test_events("./yaml-test-suite/ZCZ6");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZF4X_Spec_Example_2_6_Mapping_of_Mappings_events() {
    test_events("./yaml-test-suite/ZF4X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZF4X_Spec_Example_2_6_Mapping_of_Mappings_json() {
    test_json("./yaml-test-suite/ZF4X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZF4X_Spec_Example_2_6_Mapping_of_Mappings_out() {
    test_out("./yaml-test-suite/ZF4X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZF4X_Spec_Example_2_6_Mapping_of_Mappings_emit() {
    test_emit("./yaml-test-suite/ZF4X");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZH7C_Anchors_in_Mapping_events() {
    test_events("./yaml-test-suite/ZH7C");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZH7C_Anchors_in_Mapping_json() {
    test_json("./yaml-test-suite/ZH7C");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZH7C_Anchors_in_Mapping_out() {
    test_out("./yaml-test-suite/ZH7C");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZH7C_Anchors_in_Mapping_emit() {
    test_emit("./yaml-test-suite/ZH7C");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZK9H_Nested_top_level_flow_mapping_events() {
    test_events("./yaml-test-suite/ZK9H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZK9H_Nested_top_level_flow_mapping_json() {
    test_json("./yaml-test-suite/ZK9H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZK9H_Nested_top_level_flow_mapping_out() {
    test_out("./yaml-test-suite/ZK9H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZK9H_Nested_top_level_flow_mapping_emit() {
    test_emit("./yaml-test-suite/ZK9H");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZL4Z_Invalid_nested_mapping_error() {
    test_error("./yaml-test-suite/ZL4Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZL4Z_Invalid_nested_mapping_events() {
    test_events("./yaml-test-suite/ZL4Z");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZVH3_Wrong_indented_sequence_item_error() {
    test_error("./yaml-test-suite/ZVH3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZVH3_Wrong_indented_sequence_item_events() {
    test_events("./yaml-test-suite/ZVH3");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZWK4_Key_with_anchor_after_missing_explicit_mapping_value_events() {
    test_events("./yaml-test-suite/ZWK4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZWK4_Key_with_anchor_after_missing_explicit_mapping_value_json() {
    test_json("./yaml-test-suite/ZWK4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZWK4_Key_with_anchor_after_missing_explicit_mapping_value_out() {
    test_out("./yaml-test-suite/ZWK4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZWK4_Key_with_anchor_after_missing_explicit_mapping_value_emit() {
    test_emit("./yaml-test-suite/ZWK4");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZXT5_Implicit_key_followed_by_newline_and_adjacent_value_error() {
    test_error("./yaml-test-suite/ZXT5");
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test_ZXT5_Implicit_key_followed_by_newline_and_adjacent_value_events() {
    test_events("./yaml-test-suite/ZXT5");
}
