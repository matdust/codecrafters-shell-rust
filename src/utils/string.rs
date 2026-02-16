use log::info;

const ESCAPED_CHARS_BY_BACKSLASH_IN_DOUBLE_QUOTES: [char; 5] = ['"', '\\', '$', '`', 'n'];

const REDIRECT_OPERATORS: [&str; 2] = [">", "1>"];

/// Splits [&str] at the first " "
/// # Returns
/// Tuple of 2 with optional second value
///
/// # Safety
/// Input cannot be empty
pub fn get_cmd_and_args(input: &str) -> (String, Vec<String>) {
    let v = get_parsed_input(input);
    info!("After parsed: {:?}", v);
    (v[0].clone(), v[1..].to_vec())
}

pub fn get_parsed_input(args: &str) -> Vec<String> {
    let mut r = Vec::new();
    let mut word_to_append = String::new();

    let mut single_quotes = false;
    let mut double_quotes = false;
    let mut literal_mode = false;

    for ch in args.trim().chars() {
        if literal_mode {
            literal_mode = false;

            if double_quotes && !ESCAPED_CHARS_BY_BACKSLASH_IN_DOUBLE_QUOTES.contains(&ch) {
                word_to_append.push('\\');
            }

            word_to_append.push(ch);
            continue;
        }

        // check if we can enter literal mode -- cannot be entered when single_quotes_mode is on
        if ch == '\\' && !single_quotes {
            literal_mode = true;
            continue;
        }

        // handle single quotes
        if ch == '\'' && !double_quotes {
            single_quotes = !single_quotes;
            continue;
        }

        // handle single quotes
        if ch == '"' && !single_quotes {
            double_quotes = !double_quotes;
            continue;
        }

        // handle char
        if ch == ' ' {
            if !(single_quotes || double_quotes) {
                // prevent appending empty String to result
                if word_to_append.is_empty() {
                    continue;
                }

                // we are not in any quotes mode so ' ' means next word
                r.push(word_to_append.clone());
                word_to_append.clear();
            } else {
                word_to_append.push(ch);
            }
        } else {
            word_to_append.push(ch);
        }
    }

    r.push(word_to_append);

    r
}

// pub fn should_redirect<S, I>(args: I)
// where
//     I: IntoIterator<Item = S>,
//     S: AsRef<str>,
// {
//     args.into_iter()
// }
//
/// Checks if redirect should happens and returns path if so
///
/// # Returns
/// Returns [Option<String>]
pub fn get_redirect_path(args: &[&str]) -> Option<String> {
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if REDIRECT_OPERATORS.contains(arg) {
            return iter.next().map(|x| x.to_string());
        }
    }

    None
}
