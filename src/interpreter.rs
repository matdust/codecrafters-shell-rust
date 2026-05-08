use std::io::Write;

use crate::parser::Node;

pub fn interpret(ast: &Node, stdout: &mut dyn Write, stderr: &mut dyn Write) {
    match ast {
        Node::Leaf(command) => command.execute(stdout, stderr),
        Node::Pipe { left, right } => {
            interpret(left, stdout, stderr);
            interpret(right, stdout, stderr);
        }
        Node::RedirectStdout { source, target } => {
            let path = std::path::Path::new(target);
            let parent_dirs = path.parent().unwrap_or_else(|| std::path::Path::new("."));
            std::fs::create_dir_all(parent_dirs)
                .expect("Failed to create parent directories for redirect target file");

            let mut file =
                std::fs::File::create(path).expect("Failed to create redirect target file");
            interpret(source, &mut file, stderr);
        }
        Node::RedirectStderr { source, target } => {
            let path = std::path::Path::new(target);
            let parent_dirs = path.parent().unwrap_or_else(|| std::path::Path::new("."));
            std::fs::create_dir_all(parent_dirs)
                .expect("Failed to create parent directories for redirect target file");

            let mut file =
                std::fs::File::create(path).expect("Failed to create redirect target file");
            interpret(source, stdout, &mut file);
        }
    }
}
