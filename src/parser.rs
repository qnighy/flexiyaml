use crate::document::{Document, Stream};
use crate::node::{Mapping, MappingStyle, Node, NodeContent, Sequence, SequenceStyle};

pub fn parse(buf: &[u8]) -> Stream {
    // TODO: handle BOM and encoding
    let mut parser = Parser::new(buf);
    parser.parse_stream()
}

#[derive(Debug)]
struct Parser<'a> {
    buf: &'a [u8],
    pos: usize,
    ws_pos: usize,
    line_pos: usize,
}

impl<'a> Parser<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self {
            buf,
            pos: 0,
            ws_pos: 0,
            line_pos: 0,
        }
    }

    fn parse_stream(&mut self) -> Stream {
        let mut documents = Vec::new();
        while let Some(doc) = self.parse_document() {
            documents.push(doc);
        }
        assert!(self.is_eof());
        Stream { documents }
    }

    fn parse_document(&mut self) -> Option<Document> {
        if self.is_eof() {
            return None;
        }
        let root = self.parse_node();
        self.pos = self.buf.len();
        Some(Document { root })
    }

    fn parse_node(&mut self) -> Node {
        self.skip_whitespace();
        match self.next_char() {
            Some(b'-') => Node::Owned(NodeContent::Sequence(Sequence {
                elements: vec![],
                style: Some(SequenceStyle::Block),
            })),
            Some(b'?') => Node::Owned(NodeContent::Mapping(Mapping {
                entries: vec![],
                style: Some(MappingStyle::Block),
            })),
            // Some(b':') => {}
            // Some(b',') => {}
            Some(b'[') => Node::Owned(NodeContent::Sequence(Sequence {
                elements: vec![],
                style: Some(SequenceStyle::Flow),
            })),
            // Some(b']') => {}
            Some(b'{') => Node::Owned(NodeContent::Mapping(Mapping {
                entries: vec![],
                style: Some(MappingStyle::Flow),
            })),
            // Some(b'}') => {}
            Some(b'#') => unreachable!(),

            // Some(b'&') => {}
            // Some(b'*') => {}
            // Some(b'!') => {}
            // Some(b'|') => {}
            // Some(b'>') => {}
            // Some(b'\'') => {}
            // Some(b'"') => {}
            // Some(b'%') => {}
            // Some(b'@') | Some(b'`') => {}
            _ => Node::Owned(NodeContent::default()),
        }
    }

    fn skip_whitespace(&mut self) {
        self.ws_pos = self.pos;
        loop {
            let last_pos = self.pos;
            self.ws_in_line();
            self.ws_comment();
            if last_pos == self.pos {
                break;
            }
        }
    }

    fn ws_in_line(&mut self) {
        while let Some(&ch) = self.buf.get(self.pos) {
            if ch == b' ' || ch == b'\t' {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn ws_comment(&mut self) {
        if self.is_spaced() && self.next_char() == Some(b'#') {
            while !self.is_eol() {
                self.pos += 1;
            }
            self.ws_nl();
        }
    }

    fn ws_nl(&mut self) {
        match self.next_char() {
            Some(b'\n') => {
                self.pos += 1;
            }
            Some(b'\r') => {
                self.pos += 1;
                if self.next_char() == Some(b'\n') {
                    self.pos += 1;
                }
            }
            None => {}
            _ => unreachable!(),
        }
        self.line_pos = self.pos;
    }

    fn next_char(&self) -> Option<u8> {
        if self.pos < self.buf.len() {
            Some(self.buf[self.pos])
        } else {
            None
        }
    }

    fn is_spaced(&self) -> bool {
        self.pos == 0 || {
            let ch = self.buf[self.pos - 1];
            ch == b' ' || ch == b'\t' || ch == b'\r' || ch == b'\n'
        }
    }

    fn is_eol(&self) -> bool {
        if let Some(ch) = self.next_char() {
            ch == b'\n' || ch == b'\r'
        } else {
            true
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.buf.len()
    }
}

fn indent(buf: &[u8]) -> usize {
    buf.iter().position(|&ch| ch != b' ').unwrap_or(buf.len())
}
