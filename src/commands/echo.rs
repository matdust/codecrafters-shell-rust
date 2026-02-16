pub fn execute(args: &str) {
    match crate::utils::string::get_redirect_path(&args.split_whitespace().collect::<Vec<&str>>()) {
        Some(path) => {
            crate::utils::files::redirect_to_file(&path, args);
        }
        None => println!("{}", args),
    }
}
