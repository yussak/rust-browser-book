use alloc::{string::String, vec::Vec};

#[derive(Debug, Clone, PartialEq)]
pub enum CssToken {
    HashToken(String),
    Delim(char),
    Number(f64),
    Colon,
    SemiColon,
    OpenParenthesis,
    CloseParenthesis,
    OpenCurly,
    CloseCurly,
    Ident(String),
    StringToken(String),
    AtKeyword(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CssTokenizer {
    pos: usize,
    input: Vec<char>,
}

impl Iterator for CssTokenizer {
    type Item = CssToken;

    // CSS文字列を１文字ずつ見ていき、現在の文字によって振る舞いを変える
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.pos >= self.input.len() {
                return None;
            }

            let c = self.input[self.pos];

            let token = match c {
              // 次のトークンを決定
            };

            self.pos += 1;
            return Some(token);
        }
    }
}
