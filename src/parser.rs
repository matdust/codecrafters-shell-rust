use crate::command::Command;
use crate::lexer::{OperatorType, Token};

#[derive(Debug, PartialEq)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn move_next(&mut self) {
        self.position += 1;
    }

    fn get_current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn consume(&mut self) -> Option<Token> {
        match self.tokens.get(self.position) {
            Some(token) => {
                self.position += 1;
                Some(token.to_owned())
            }
            None => None,
        }
    }
}

/// AST Node
#[derive(Debug, PartialEq)]
pub enum Node {
    Leaf(Command),
    Operator {
        operation: OperatorType,
        left: Box<Node>,
        right: Box<Node>,
    },
}

pub fn build_ast(tokens: Vec<Token>) -> Result<Node, String> {
    let mut parser = Parser::new(tokens);
    parse_operator(&mut parser)
}

// handles Node::Operator
fn parse_operator(parser: &mut Parser) -> Result<Node, String> {
    let mut head = parse_command(parser)?;

    while let Some(token) = parser.get_current() {
        match token {
            Token::Operator(operator_type) => match operator_type {
                OperatorType::Pipe => {
                    parser.move_next();
                    let right = parse_command(parser)?;
                    let new_head = Node::Operator {
                        operation: OperatorType::Pipe,
                        left: Box::new(head),
                        right: Box::new(right),
                    };
                    head = new_head;
                }
                OperatorType::RedirectStdout => {
                    parser.move_next();
                    let right = parse_command(parser)?;
                    let new_head = Node::Operator {
                        operation: OperatorType::RedirectStdout,
                        left: Box::new(head),
                        right: Box::new(right),
                    };
                    head = new_head;
                }
            },
            _ => break,
        }
    }

    Result::Ok(head)
}

// handles Node::Leaf
fn parse_command(parser: &mut Parser) -> Result<Node, String> {
    if let Some(Token::Word(input)) = parser.consume() {
        return Result::Ok(Node::Leaf(Command::from_string(&input)));
    }

    Result::Err("Unexpected end of input".to_string())
}
