use crate::command::Command;
use crate::lexer::Token;

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
    Pipe { left: Box<Node>, right: Box<Node> },
    RedirectStdout { source: Box<Node>, target: String },
    RedirectStderr { source: Box<Node>, target: String },
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
            Token::Pipe => {
                parser.move_next();
                let right = parse_command(parser)?;
                head = Node::Pipe {
                    left: Box::new(head),
                    right: Box::new(right),
                };
            }
            Token::RedirectStdout => {
                parser.move_next();
                if let Some(Token::Path(path)) = parser.consume() {
                    head = Node::RedirectStdout {
                        source: Box::new(head),
                        target: path,
                    };
                }
            }
            Token::RedirectStderr => {
                parser.move_next();
                if let Some(Token::Path(path)) = parser.consume() {
                    head = Node::RedirectStderr {
                        source: Box::new(head),
                        target: path,
                    };
                }
            }
            _ => break,
        }
    }

    Result::Ok(head)
}

// handles Node::Leaf
fn parse_command(parser: &mut Parser) -> Result<Node, String> {
    if let Some(Token::Node(command)) = parser.consume() {
        return Result::Ok(Node::Leaf(command));
    }

    Result::Err("Unexpected end of input".to_string())
}
