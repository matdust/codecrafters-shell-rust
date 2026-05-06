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
