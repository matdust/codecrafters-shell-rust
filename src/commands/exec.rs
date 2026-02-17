use std::{ffi::OsStr, process::Command};

pub fn execute<I, S>(cmd: &str, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(cmd);
    command.args(args);

    match command.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            print!("{}", stdout);
        }
        Err(err) => eprintln!("error: {}", err),
    }
}
