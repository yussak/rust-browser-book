use core::iter::Peekable;

use alloc::{string::String, vec::Vec};

use crate::renderer::css::token::{CssToken, CssTokenizer};

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

// #[derive(Debug,Clone,PartialEq)]
// ルールを表す（セレクタとその中のプロパティ、値）ノード
pub struct QualifiedRule {
    pub selector: Selector,
    pub declarations: Vec<Declaration>,
}

impl QualifiedRule {
    pub fn new() -> Self {
        Self {
            selector: Selector::TypeSelector("".to_string()),
            declarations: Vec::new(),
        }
    }

    pub fn set_selector(&mut self, selector: Selector) {
        self.selector = selector;
    }

    pub fn set_declarations(&mut self, declarations: Vec<Declaration>) {
        self.declarations = declarations;
    }
}

// TODO: #[derive(Debug, Clone, PartialEq, Eq)]だが必要になったら追加してみたいので一旦コメントアウト
pub enum Selector {
    // タグ名で指定するセレクタ
    TypeSelector(String),
    ClassSelector(String),
    IdSelector(String),
    // パース中にエラーが起こったときに使用されるセレクタ
    UnknownSelector,
}

// TODO: #[derive(Debug, Clone, PartialEq)]だが必要になったら追加してみたいので一旦コメントアウト
// プロパティ、値のセットを表現
pub struct Declaration {
    pub property: String,
    pub value: ComponentValue,
}

impl Declaration {
    pub fn new() -> Self {
        Self {
            property: String::new(),
            value: ComponentValue::Ident(String::new()),
        }
    }

    pub fn set_property(&mut self, property: String) {
        self.property = property;
    }

    pub fn set_value(&mut self, value: ComponentValue) {
        self.value = value;
    }
}

// プロパティの値に対するノード
// #fffや42pxなど
// このブラウザでは保存されたトークンのみを値として扱う
pub type ComponentValue = CssToken;
