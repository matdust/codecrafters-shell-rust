use std::io::Write;

use crate::{lexer::OperatorType, parser::Node};

pub fn interpret(ast: &Node, stdout: &mut dyn Write) {
    match ast {
        Node::Leaf(command) => command.execute(stdout),
        Node::Operator {
            operation,
            left,
            right,
        } => match operation {
            OperatorType::Pipe => {
                interpret(left, stdout);
                interpret(right, stdout);
                // let mut buffer = Vec::new();
                // interpret(left, &mut buffer);
                // pipe output becomes part of the right command's input
                // for now, just write it to the right command's stdout
                // stdout.write_all(&buffer).unwrap();
                // interpret(right, stdout);
            }
            OperatorType::RedirectStdout => {
                unreachable!("RedirectStdout should be parsed as Node::RedirectStdout");
            }
        },
        Node::RedirectStdout { source, target } => {
            let path = std::path::Path::new(target);
            let parent_dirs = path.parent().unwrap_or_else(|| std::path::Path::new("."));
            std::fs::create_dir_all(parent_dirs)
                .expect("Failed to create parent directories for redirect target file");

            let mut file =
                std::fs::File::create(path).expect("Failed to create redirect target file");
            interpret(source, &mut file);
        }
    }
}
