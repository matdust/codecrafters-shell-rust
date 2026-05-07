use std::{env, path::PathBuf};

pub fn execute(args: &[&str]) {
    let path = if args.is_empty() { "~" } else { args[0] };
    let mut path_chars = path.chars();
    let Some(path_type) = path_chars.next() else {
        eprintln!("cannot get path");
        return;
    };

    let new_path = match path_type {
        '/' => {
            if crate::utils::files::is_dir_exists(path) {
                PathBuf::from(path)
            } else {
                eprintln!("cd: {path}: No such file or directory");
                return;
            }
        }
        '~' => {
            let Some(home_dir) = env::home_dir() else {
                eprintln!("cannot get home dir");
                return;
            };
            path_chars.next();
            home_dir.join(path_chars.as_str())
        }
        _ => {
            let Ok(current_dir) = env::current_dir() else {
                eprintln!("cannot get current dir");
                return;
            };
            current_dir.join(path)
        }
    };

    if !&new_path.is_dir() {
        eprintln!("cd: {path}: No such file or directory");
        return;
    }

    if let Err(err) = env::set_current_dir(new_path) {
        eprintln!("error: {}", err);
    }
}
