use core::iter::Peekable;

use alloc::{rc::Rc, string::String, vec::Vec};

use crate::renderer::js::token::{JsLexer, Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    ExpressionStatement(Option<Rc<Node>>),
    AdditiveExpression {
        operator: char,
        left: Option<Rc<Node>>,
        right: Option<Rc<Node>>,
    },
    AssignmentExpression {
        operator: char,
        left: Option<Rc<Node>>,
        right: Option<Rc<Node>>,
    },
    MemberExpression {
        object: Option<Rc<Node>>,
        property: Option<Rc<Node>>,
    },
    NumericLiteral(u64),
    VariableDeclaration {
        declarations: Vec<Option<Rc<Node>>>,
    },
    VariableDeclarator {
        id: Option<Rc<Node>>,
        init: Option<Rc<Node>>,
    },
    Identifier(String),
    StringLiteral(String),
    // {}で囲まれるブロックを表す
    BlockStatement {
        body: Vec<Option<Rc<Node>>>,
    },
    // returnの予約語から始まる文を表す
    ReturnStatement {
        argument: Option<Rc<Node>>,
    },
    // functionの予約語から始まる文を表す
    FunctionDeclaration {
        id: Option<Rc<Node>>,
        params: Vec<Option<Rc<Node>>>,
        body: Option<Rc<Node>>,
    },
    // 関数呼び出しを表す
    CallExpression {
        callee: Option<Rc<Node>>,
        arguments: Vec<Option<Rc<Node>>>,
    },
}

impl Node {
    pub fn new_expression_statement(expression: Option<Rc<Self>>) -> Option<Rc<Self>> {
        Some(Rc::new(Node::ExpressionStatement(expression)))
    }

    pub fn new_additive_expression(
        operator: char,
        left: Option<Rc<Node>>,
        right: Option<Rc<Node>>,
    ) -> Option<Rc<Self>> {
        Some(Rc::new(Node::AdditiveExpression {
            operator,
            left,
            right,
        }))
    }

    pub fn new_assignment_expression(
        operator: char,
        left: Option<Rc<Node>>,
        right: Option<Rc<Node>>,
    ) -> Option<Rc<Self>> {
        Some(Rc::new(Node::AssignmentExpression {
            operator,
            left,
            right,
        }))
    }

    pub fn new_member_expression(
        object: Option<Rc<Self>>,
        property: Option<Rc<Self>>,
    ) -> Option<Rc<Self>> {
        Some(Rc::new(Node::MemberExpression { object, property }))
    }

    pub fn new_numeric_literal(value: u64) -> Option<Rc<Self>> {
        Some(Rc::new(Node::NumericLiteral(value)))
    }

    pub fn new_variable_declarator(
        id: Option<Rc<Self>>,
        init: Option<Rc<Self>>,
    ) -> Option<Rc<Self>> {
        Some(Rc::new(Node::VariableDeclarator { id, init }))
    }

    pub fn new_variable_declaration(declarations: Vec<Option<Rc<Self>>>) -> Option<Rc<Self>> {
        Some(Rc::new(Node::VariableDeclaration { declarations }))
    }

    pub fn new_identifier(name: String) -> Option<Rc<Self>> {
        Some(Rc::new(Node::Identifier(name)))
    }

    pub fn new_string_literal(value: String) -> Option<Rc<Self>> {
        Some(Rc::new(Node::StringLiteral(value)))
    }

    pub fn new_block_statement(body: Vec<Option<Rc<Self>>>) -> Option<Rc<Self>> {
        Some(Rc::new(Node::BlockStatement { body }))
    }

    pub fn new_return_statement(argument: Option<Rc<Self>>) -> Option<Rc<Self>> {
        Some(Rc::new(Node::ReturnStatement { argument }))
    }

    pub fn new_function_declaration(
        id: Option<Rc<Self>>,
        params: Vec<Option<Rc<Self>>>,
        body: Option<Rc<Self>>,
    ) -> Option<Rc<Self>> {
        Some(Rc::new(Node::FunctionDeclaration { id, params, body }))
    }

    pub fn new_call_expression(
        callee: Option<Rc<Self>>,
        arguments: Vec<Option<Rc<Self>>>,
    ) -> Option<Rc<Self>> {
        Some(Rc::new(Node::CallExpression { callee, arguments }))
    }
}

pub struct JsParser {
    t: Peekable<JsLexer>,
}

impl JsParser {
    pub fn new(t: JsLexer) -> Self {
        Self { t: t.peekable() }
    }

    pub fn parse_ast(&mut self) -> Program {
        let mut program = Program::new();

        let mut body = Vec::new();

        loop {
            let node = self.source_element();

            match node {
                Some(n) => body.push(n),
                None => {
                    program.set_body(body);
                    return program;
                }
            }
        }
    }

    // SourceElementはStatementで構成されることを表現
    fn source_element(&mut self) -> Option<Rc<Node>> {
        match self.t.peek() {
            Some(t) => t,
            None => return None,
        };

        self.statement()
    }

    fn statement(&mut self) -> Option<Rc<Node>> {
        let t = match self.t.peek() {
            Some(t) => t,
            None => return None,
        };

        let node = match t {
            Token::Keyword(keyword) => {
                if keyword == "var" {
                    // varの予約語を消費
                    assert!(self.t.next().is_some());

                    self.variable_declaration()
                } else {
                    None
                }
            }
            _ => Node::new_expression_statement(self.assignment_expression()),
        };

        if let Some(t) = self.t.peek() {
            if let Token::Punctuator(c) = t {
                // ';'を消費する
                if c == &';' {
                    assert!(self.t.next().is_some());
                }
            }
        }
        node
    }

    fn assignment_expression(&mut self) -> Option<Rc<Node>> {
        let expr = self.additive_expression();

        let t = match self.t.peek() {
            Some(token) => token,
            None => return expr,
        };

        match t {
            Token::Punctuator('=') => {
                // = を消費する
                assert!(self.t.next().is_some());
                Node::new_assignment_expression('=', expr, self.assignment_expression())
            }
            _ => expr,
        }
    }

    fn additive_expression(&mut self) -> Option<Rc<Node>> {
        let left = self.left_hand_side_expression();

        let t = match self.t.peek() {
            Some(token) => token.clone(),
            None => return left,
        };

        match t {
            Token::Punctuator(c) => match c {
                '+' | '-' => {
                    // + or -の記号を消費する
                    assert!(self.t.next().is_some());
                    Node::new_additive_expression(c, left, self.assignment_expression())
                }
                _ => left,
            },
            _ => left,
        }
    }

    fn left_hand_side_expression(&mut self) -> Option<Rc<Node>> {
        self.member_expression()
    }

    fn member_expression(&mut self) -> Option<Rc<Node>> {
        self.primary_expression()
    }

    fn primary_expression(&mut self) -> Option<Rc<Node>> {
        let t = match self.t.next() {
            Some(token) => token,
            None => return None,
        };

        match t {
            Token::Identifier(value) => Node::new_identifier(value),
            Token::StringLiteral(value) => Node::new_string_literal(value),
            Token::Number(value) => Node::new_numeric_literal(value),
            _ => None,
        }
    }

    fn variable_declaration(&mut self) -> Option<Rc<Node>> {
        let ident = self.identifier();

        let declarator = Node::new_variable_declarator(ident, self.initializer());

        let mut declarations = Vec::new();
        declarations.push(declarator);

        Node::new_variable_declaration(declarations)
    }

    fn identifier(&mut self) -> Option<Rc<Node>> {
        let t = match self.t.next() {
            Some(token) => token,
            None => return None,
        };

        match t {
            Token::Identifier(name) => Node::new_identifier(name),
            _ => None,
        }
    }

    fn initializer(&mut self) -> Option<Rc<Node>> {
        let t = match self.t.next() {
            Some(token) => token,
            None => return None,
        };

        match t {
            Token::Punctuator(c) => match c {
                '=' => self.assignment_expression(),
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
// ASTのルートノードとなる構造体
pub struct Program {
    body: Vec<Rc<Node>>,
}

impl Program {
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }

    pub fn set_body(&mut self, body: Vec<Rc<Node>>) {
        self.body = body;
    }

    pub fn body(&self) -> &Vec<Rc<Node>> {
        &self.body
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_empty() {
        let input = "".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let expected = Program::new();
        assert_eq!(expected, parser.parse_ast());
    }

    #[test]
    fn test_num() {
        let input = "42".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let mut expected = Program::new();
        let mut body = Vec::new();
        body.push(Rc::new(Node::ExpressionStatement(Some(Rc::new(
            Node::NumericLiteral(42),
        )))));
        expected.set_body(body);
        assert_eq!(expected, parser.parse_ast());
    }

    #[test]
    fn test_add_nums() {
        let input = "1 + 2".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let mut expected = Program::new();
        let mut body = Vec::new();
        body.push(Rc::new(Node::ExpressionStatement(Some(Rc::new(
            Node::AdditiveExpression {
                operator: '+',
                left: Some(Rc::new(Node::NumericLiteral(1))),
                right: Some(Rc::new(Node::NumericLiteral(2))),
            },
        )))));
        expected.set_body(body);
        assert_eq!(expected, parser.parse_ast());
    }

    #[test]
    fn test_assignment_variable() {
        let input = "var foo=\"bar\";".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let mut expected = Program::new();
        let mut body = Vec::new();
        body.push(Rc::new(Node::VariableDeclaration {
            declarations: [Some(Rc::new(Node::VariableDeclarator {
                id: Some(Rc::new(Node::Identifier("foo".to_string()))),
                init: Some(Rc::new(Node::StringLiteral("bar".to_string()))),
            }))]
            .to_vec(),
        }));
        expected.set_body(body);
        assert_eq!(expected, parser.parse_ast());
    }

    #[test]
    fn test_add_variable_and_num() {
        let input = "var foo=42; var result=foo+1".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let mut expected = Program::new();
        let mut body = Vec::new();
        body.push(Rc::new(Node::VariableDeclaration {
            declarations: [Some(Rc::new(Node::VariableDeclarator {
                id: Some(Rc::new(Node::Identifier("foo".to_string()))),
                init: Some(Rc::new(Node::NumericLiteral(42))),
            }))]
            .to_vec(),
        }));
        body.push(Rc::new(Node::VariableDeclaration {
            declarations: [Some(Rc::new(Node::VariableDeclarator {
                id: Some(Rc::new(Node::Identifier("result".to_string()))),
                init: Some(Rc::new(Node::AdditiveExpression {
                    operator: '+',
                    left: Some(Rc::new(Node::Identifier("foo".to_string()))),
                    right: Some(Rc::new(Node::NumericLiteral(1))),
                })),
            }))]
            .to_vec(),
        }));
        expected.set_body(body);
        assert_eq!(expected, parser.parse_ast());
    }
}
