use core::iter::Peekable;

use alloc::{string::String, vec::Vec};

use crate::renderer::css::token::CssTokenizer;

#[derive(Debug, Clone)]
pub struct CssParser {
    t: Peekable<CssTokenizer>,
}

impl CssParser {
    pub fn new(t: CssTokenizer) -> Self {
        Self { t: peekable() }
    }
}

#[derive(Debug, Clone, PartialEq)]
// CSSOMのルートノードとなる
pub struct StyleSheet {
    pub rules: Vec<QualifiedRule>,
}

impl StyleSheet {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn set_rules(&mut self, rules: Vec<QualifiedRule>) {
        self.rules = rules;
    }
}
