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
