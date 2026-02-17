use std::{
    env,
    fs::{self},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
};

const KEY: &str = "PATH";

pub fn find_exe_in_env(cmd: &str) -> Option<PathBuf> {
    let paths = env::var(KEY).ok()?;

    env::split_paths(&paths)
        .map(|f| f.join(cmd))
        .find(|cmd_path| {
            fs::metadata(cmd_path)
                .map(|md| md.is_file() && md.permissions().mode() & 0o111 != 0)
                .unwrap_or(false)
        })
}

pub fn is_dir_exists(path: &str) -> bool {
    PathBuf::from(&path).is_dir()
}

pub fn redirect_to_file(path: &str, output: &str) {
    let file = std::fs::File::create(path).unwrap();
    let stdio = std::process::Stdio::from(file);

    let command = std::process::Command::new("foo").stdout(stdio);
}

// pub fn get_redirect_path(args: &[&str]) -> Option<String> {
//     let mut iter = args.iter();
//     while let Some(arg) = iter.next() {
//         if REDIRECT_OPERATORS.contains(arg) {
//             return iter.next().map(|x| x.to_string());
//         }
//     }
//
//     None
// }
