use std::io::Write;

pub fn execute(command: &str, stdout: &mut dyn Write) {
    writeln!(stdout, "{}: command not found", command).unwrap();
}