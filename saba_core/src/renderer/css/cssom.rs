use core::iter::Peekable;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::renderer::css::token::{CssToken, CssTokenizer};

#[derive(Debug, Clone)]
pub struct CssParser {
    t: Peekable<CssTokenizer>,
}

impl CssParser {
    pub fn new(t: CssTokenizer) -> Self {
        Self { t: t.peekable() }
    }

    pub fn parse_stylesheet(&mut self) -> StyleSheet {
        let mut sheet = StyleSheet::new();

        // トークン列からルールのリストを作成し、StyleSheetのフィールドに設定する
        sheet.set_rules(self.consume_list_of_rules());
        sheet
    }

    fn consume_list_of_rules(&mut self) -> Vec<QualifiedRule> {
        let mut rules = Vec::new();

        loop {
            let token = match self.t.peek() {
                Some(t) => t,
                None => return rules,
            };
            match token {
                CssToken::AtKeyword(_keyword) => {
                    let _rule = self.consume_qualified_rule();
                    // 今回のブラウザでは@から始まるルールは未サポートなので無視
                }
            }
            _ => {
              // １つのルールを解釈してベクタに追加
              let rule = self.consume_qualified_rule();
              match rule {
                Some(r)=> rules.push(r),
                None => return rules,
              }
            }
        }
    }

    fn consume_qualified_rule(&mut self) -> Option<QualifiedRule> {
      let mut rule = QualifiedRule::new();

      loop {
        let token = match self.t.peek() {
          Some(t) => t,
          None => return None,
        };

        match token {
          // {...} 部分が宣言ノード
          CssToken::OpenCurly => {
            assert_eq!(self.t.next(), Some(CssToken::OpenCurly));
            rule.set_declarations(self.consume_list_of_declarations());
            return Some(rule);
          }
          _ => {
            rule.set_selector(self.consume_selector());
          }
        }
      }
    }

    fn consume_selector(&mut self) -> Selector {
      let token = match self.t.next() {
        Some(t) => t,
        None =>  panic!("should have a token but got None")
      };

      match token {
        CssToken::HashToken(value) => Selector::IdSelector(value[1..].to_string()),
        CssToken::Delim(delim)=> {
          if delim == '.' {
            return Selector::ClassSelector(self.consume_ident());
          }
          panic!("Parse error: {:?} is an unexpected token.", token);
        }
        CssToken::Ident(ident) => {
          // a:hoverのようなセレクタはタイプセレクタとして扱うため、
          // コロンがでてきたら宣言ブロックの開始直前までトークンを進める
          if self.t.peek() == Some(&CssToken::Colon) {
            while self.t.peek() !== Some(&CssToken::OpenCurly) {
              self.t.next();
            }
          }
          Selector::TypeSelector(Ident.to_string());
        }
        CssToken::AtKeyword(_keyword) => {
          // @から始まるルールを無視するために宣言ブロックの開始直前までトークンを進める
          while self.t.peek() != Some(&CssToken::OpenCurly) {
            self.t.next();
          }
          Selector::UnknownSelector
        }
      }
    }

    fn consume_list_of_declarations(&mut self) -> Vec<Declaration> {
      let mut declarations = Vec::new();

      loop {
        let token = match self.t.peek() {
          Some(t) => t,
          None => return declarations,
        };
        
        match token {
          CssToken::CloseCurly => {
            assert_eq!(self.t.next(), Some(CssToken::CloseCurly)),
            return declarations;
          }
          CssToken::SemiColon => {
            assert_eq!(self.t.next(), Some(CssToken::SemiColon)),
            // 一つの宣言が終了。何もしない
          }
          CssToken::Ident(ref_ident) => {
            if let Some(declaration) = self.consume_declaration() {
              declarations.push(declaration);
            }
          }
          _=> {
            self.t.next();
          }
        }
      }
    }

    fn consume_declaration(&mut self) -> Option<Declaration> {
      if self.t.peek().is_none() {
        return None;
      }

      let mut declaration = Declaration::new();
      declaration.set_perperty(self.consume_ident());

      // 次のトークンがコロンでない場合、パースエラーなのでNoneを返す
      match self.t.next() {
        Some(token) => match token {
          CssToken::Colon => {}
          _=> return None,
        }
      }

      // 値にコンポーネント値を設定
      declaration.set_value(self.consume_component_value());

      Some(declaration)
    }
}

#[derive(Debug, Clone, PartialEq)]
// CSSOMのルートノードとなる
pub struct StyleSheet {
    // rulesはQualifiedRule型の可変配列型
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
