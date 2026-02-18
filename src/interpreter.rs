use crate::{lexer::OperatorType, parser::Node};

pub fn interpret(ast: &Node) {
    match ast {
        Node::Leaf(command) => command.command_type.execute(),
        Node::Operator {
            operation,
            left,
            right,
        } => match operation {
            OperatorType::Pipe => todo!(),
            OperatorType::RedirectStdout => todo!(),
        },
    }
}
