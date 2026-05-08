use std::ffi::OsStr;
use std::io::Write;
use std::process::Command;

pub fn execute<I, S>(cmd: &str, args: I, stdout: &mut dyn Write, stderr: &mut dyn Write)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(cmd);
    command.args(args);
    command.stderr(std::process::Stdio::piped());

    match command.stdout(std::process::Stdio::piped()).spawn() {
        Ok(child) => {
            let output = child.wait_with_output().unwrap();
            stderr.write_all(&output.stderr).unwrap();
            stdout.write_all(&output.stdout).unwrap();
        }
        Err(err) => {
            println!("here 2");
            stderr.write_all(&format!("{}", err).into_bytes()).unwrap();
        }
    }
}
