use alloc::{string::String, vec::Vec};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Punctuator(char),
    Number(u64),
}

// レキサー：トークナイザー＋α
pub struct JsLexer {
    pos: usize,
    input: Vec<char>,
}

impl JsLexer {
    pub fn new(js: String) -> Self {
        Self {
            pos: 0,
            input: js.chars().collect(),
        }
    }
}
