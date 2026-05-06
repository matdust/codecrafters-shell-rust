use std::env;
use std::io::Write;

pub fn execute(stdout: &mut dyn Write) {
    writeln!(stdout, "{}", env::current_dir().unwrap_or_default().display()).unwrap();
}