use std::ffi::OsStr;
use std::io::Write;
use std::process::Command;

pub fn execute<I, S>(cmd: &str, args: I, stdout: &mut dyn Write)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(cmd);
    command.args(args);

    match command.output() {
        Ok(output) => {
            if output.stdout.is_empty() && !output.stderr.is_empty() {
                print!("{}", String::from_utf8_lossy(&output.stderr));
                return;
            }
            stdout.write_all(&output.stdout).unwrap();
        }
        Err(err) => eprintln!("error: {}", err),
    }
}

