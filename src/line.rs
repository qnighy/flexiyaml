#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Line {
    pub(crate) indent: usize,
    pub(crate) text: Vec<u8>,
    pub(crate) newline: NewLine,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum NewLine {
    Lf,
    CrLf,
    Cr,
    None,
}

pub(crate) fn split_lines(buf: &[u8]) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut start = 0;
    while start < buf.len() {
        let mut content_start = start;
        while buf.get(content_start) == Some(&b' ') {
            content_start += 1;
        }
        let mut content_end = content_start;
        while content_end < buf.len() && ![b'\n', b'\r'].contains(&buf[content_end]) {
            content_end += 1;
        }
        let (newline, line_end) = if buf[content_end..].starts_with(b"\n") {
            (NewLine::Lf, content_end + 1)
        } else if buf[content_end..].starts_with(b"\r\n") {
            (NewLine::CrLf, content_end + 2)
        } else if buf[content_end..].starts_with(b"\r") {
            (NewLine::Cr, content_end + 1)
        } else {
            (NewLine::None, content_end)
        };
        lines.push(Line {
            indent: content_start - start,
            text: buf[content_start..content_end].into(),
            newline,
        });
        start = line_end;
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_lines() {
        assert_eq!(split_lines(b""), vec![]);
        assert_eq!(
            split_lines(b"foo"),
            vec![Line {
                indent: 0,
                text: b"foo"[..].into(),
                newline: NewLine::None
            }]
        );
        assert_eq!(
            split_lines(b"  foo"),
            vec![Line {
                indent: 2,
                text: b"foo"[..].into(),
                newline: NewLine::None
            }]
        );
        assert_eq!(
            split_lines(b"foo:\n  bar"),
            vec![
                Line {
                    indent: 0,
                    text: b"foo:"[..].into(),
                    newline: NewLine::Lf
                },
                Line {
                    indent: 2,
                    text: b"bar"[..].into(),
                    newline: NewLine::None
                }
            ]
        );
        assert_eq!(
            split_lines(b"foo\r\n"),
            vec![Line {
                indent: 0,
                text: b"foo"[..].into(),
                newline: NewLine::CrLf
            }]
        );
        assert_eq!(
            split_lines(b"foo\r"),
            vec![Line {
                indent: 0,
                text: b"foo"[..].into(),
                newline: NewLine::Cr
            }]
        );
    }
}
