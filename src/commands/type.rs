use std::io::Write;

use crate::{command::COMMANDS, utils::files::find_exe_in_env};

pub fn execute(args: &[&str], stdout: &mut dyn Write) {
    for cmd in args {
        if COMMANDS.contains(cmd) {
            writeln!(stdout, "{} is a shell builtin", cmd).unwrap();
        } else if let Some(path) = find_exe_in_env(cmd) {
            writeln!(stdout, "{} is {}", cmd, path.display()).unwrap();
        } else {
            writeln!(stdout, "{}: not found", cmd).unwrap();
        }
    }
}
