use std::io::Write;

pub fn execute(args: &[&str], stdout: &mut dyn Write) {
    writeln!(stdout, "{}", args.join(" ")).unwrap();
}

