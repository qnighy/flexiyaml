use crate::document::{Document, Stream};
use crate::node::Node;

pub fn parse(buf: &[u8]) -> Stream {
    // TODO: handle BOM and encoding
    let mut parser = Parser::new(buf);
    parser.start_line();
    parser.parse_stream()
}

#[derive(Debug)]
struct Parser<'a> {
    buf: &'a [u8],
    pos: usize,
    line_pos: usize,
    line_indent: usize,
}

impl<'a> Parser<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self {
            buf,
            pos: 0,
            line_pos: 0,
            line_indent: 0,
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
        Node::default()
    }

    fn start_line(&mut self) {
        self.line_pos = self.pos;
        self.line_indent = indent(&self.buf[self.pos..]);
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.buf.len()
    }
}

fn indent(buf: &[u8]) -> usize {
    buf.iter().position(|&ch| ch != b' ').unwrap_or(buf.len())
}
