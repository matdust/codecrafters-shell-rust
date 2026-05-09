pub const ESCAPED_CHARS_BY_BACKSLASH_IN_DOUBLE_QUOTES: [char; 5] = ['"', '\\', '$', '`', 'n'];

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Token {
    Node(crate::command::Command),
    Path(String),
    Pipe,
    RedirectStdout,
    RedirectStderr,
    AppendStdout,
}

pub fn tokenize(args: &str) -> Vec<Token> {
    let mut result = Vec::new();

    let mut new_token = String::new();

    let mut single_quotes = false;
    let mut double_quotes = false;
    let mut literal_mode = false;

    let mut chars = args.trim().chars().peekable();

    while let Some(ch) = chars.next() {
        if literal_mode {
            literal_mode = false;

            if double_quotes && !ESCAPED_CHARS_BY_BACKSLASH_IN_DOUBLE_QUOTES.contains(&ch) {
                new_token.push('\\');
            }

            new_token.push(ch);
            continue;
        }

        // check if we can enter literal mode -- cannot be entered when single_quotes_mode is on
        if ch == '\\' && !single_quotes {
            new_token.push(ch);
            literal_mode = true;
            continue;
        }

        // handle single quotes mode
        if ch == '\'' && !double_quotes {
            single_quotes = !single_quotes;
            new_token.push(ch);
            continue;
        }

        // handle double quotes mode
        if ch == '"' && !single_quotes {
            double_quotes = !double_quotes;
            new_token.push(ch);
            continue;
        }

        match ch {
            '|' => {
                if !(single_quotes || double_quotes) {
                    if new_token.is_empty() {
                        result.push(Token::Pipe);
                        continue;
                    }

                    result.push(Token::Node(crate::command::Command::from_string(
                        new_token.trim(),
                    )));
                    result.push(Token::Pipe);
                    new_token.clear();
                } else {
                    new_token.push(ch);
                }
            }
            '>' => {
                if !(single_quotes || double_quotes) {
                    if new_token.is_empty() {
                        result.push(Token::RedirectStdout);
                        continue;
                    }
                    let redirect_token = if let Some(next_char) = chars.peek() {
                        if *next_char == '>' {
                            chars.next();
                            Token::AppendStdout
                        } else {
                            Token::RedirectStdout
                        }
                    } else {
                        Token::RedirectStdout
                    };

                    result.push(Token::Node(crate::command::Command::from_string(
                        new_token.trim(),
                    )));

                    result.push(redirect_token);

                    new_token.clear()
                } else {
                    new_token.push(ch);
                }
            }
            '1' => {
                if !(single_quotes || double_quotes) {
                    if new_token.is_empty() {
                        result.push(Token::RedirectStdout);
                        continue;
                    }

                    // standard '1' no redirection, just part of args
                    let Some(next_char) = chars.peek() else {
                        new_token.push(ch);
                        continue;
                    };

                    // standard '1' no redirection, just part of args
                    if next_char != &'>' {
                        new_token.push(ch);
                        continue;
                    }

                    chars.next();

                    let Some(next_next_char) = chars.peek() else {
                        continue;
                    };

                    let redirect_token = match next_next_char {
                        '>' => {
                            chars.next();
                            Token::AppendStdout
                        }
                        _ => Token::RedirectStdout,
                    };

                    result.push(Token::Node(crate::command::Command::from_string(
                        new_token.trim(),
                    )));

                    result.push(redirect_token);

                    new_token.clear()
                } else {
                    new_token.push(ch);
                }
            }
            '2' => {
                if !(single_quotes || double_quotes) {
                    if new_token.is_empty() {
                        result.push(Token::RedirectStderr);
                        continue;
                    }

                    // standard '2' no redirection, just part of args
                    let Some(next_char) = chars.peek() else {
                        new_token.push(ch);
                        continue;
                    };

                    // standard '2' no redirection, just part of args
                    if next_char != &'>' {
                        new_token.push(ch);
                        continue;
                    }

                    chars.next();

                    result.push(Token::Node(crate::command::Command::from_string(
                        new_token.trim(),
                    )));

                    result.push(Token::RedirectStderr);

                    new_token.clear()
                } else {
                    new_token.push(ch);
                }
            }
            _ => new_token.push(ch),
        }
    }

    // check if buf is not empty and push it as a token
    if !new_token.is_empty() {
        let Some(token) = result.last() else {
            result.push(Token::Node(crate::command::Command::from_string(
                new_token.trim(),
            )));
            return result;
        };

        let token_to_add = match token {
            Token::RedirectStdout | Token::RedirectStderr | Token::AppendStdout => {
                Token::Path(new_token.trim().to_string())
            }
            _ => Token::Node(crate::command::Command::from_string(new_token.trim())),
        };
        result.push(token_to_add);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Command;

    fn node(s: &str) -> Token {
        Token::Node(Command::from_string(s))
    }

    fn path(s: &str) -> Token {
        Token::Path(s.to_string())
    }

    // --- Basic tokenization ---

    #[test]
    fn simple_command() {
        let tokens = tokenize("echo hello");
        assert_eq!(tokens, vec![node("echo hello")]);
    }

    #[test]
    fn empty_input() {
        let tokens = tokenize("");
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn whitespace_only() {
        let tokens = tokenize("   ");
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn leading_and_trailing_whitespace() {
        let tokens = tokenize("  echo hello  ");
        assert_eq!(tokens, vec![node("echo hello")]);
    }

    // --- Pipe ---

    #[test]
    fn pipe_between_commands() {
        let tokens = tokenize("echo hello | echo world");
        assert_eq!(
            tokens,
            vec![node("echo hello"), Token::Pipe, node("echo world")]
        );
    }

    #[test]
    fn multiple_pipes() {
        let tokens = tokenize("echo a | echo b | echo c");
        assert_eq!(
            tokens,
            vec![
                node("echo a"),
                Token::Pipe,
                node("echo b"),
                Token::Pipe,
                node("echo c"),
            ]
        );
    }

    #[test]
    fn pipe_no_spaces() {
        let tokens = tokenize("echo hello|echo world");
        assert_eq!(
            tokens,
            vec![node("echo hello"), Token::Pipe, node("echo world")]
        );
    }

    // --- Redirect stdout ---

    #[test]
    fn redirect_stdout() {
        let tokens = tokenize("echo hello > file.txt");
        assert_eq!(
            tokens,
            vec![node("echo hello"), Token::RedirectStdout, path("file.txt")]
        );
    }

    #[test]
    fn redirect_stdout_explicit() {
        let tokens = tokenize("echo hello 1> file.txt");
        assert_eq!(
            tokens,
            vec![node("echo hello"), Token::RedirectStdout, path("file.txt")]
        );
    }

    #[test]
    fn redirect_stdout_no_spaces() {
        let tokens = tokenize("echo hello>file.txt");
        assert_eq!(
            tokens,
            vec![node("echo hello"), Token::RedirectStdout, path("file.txt")]
        );
    }

    #[test]
    fn redirect_stdout_explicit_no_spaces() {
        let tokens = tokenize("echo hello1>file.txt");
        assert_eq!(
            tokens,
            vec![node("echo hello"), Token::RedirectStdout, path("file.txt")]
        );
    }
    #[test]
    fn no_redirect_when_1_not_followed_by_gt() {
        let tokens = tokenize("echo hello 1 file.txt");
        assert_eq!(tokens, vec![node("echo hello 1 file.txt")]);
    }

    #[test]
    fn no_redirect_when_1_at_end() {
        let tokens = tokenize("echo hello 1");
        assert_eq!(tokens, vec![node("echo hello 1")]);
    }

    // --- Redirect stderr ---

    #[test]
    fn redirect_stderr() {
        let tokens = tokenize("echo hello 2> file.txt");
        assert_eq!(
            tokens,
            vec![node("echo hello"), Token::RedirectStderr, path("file.txt")]
        );
    }

    #[test]
    fn redirect_stderr_no_spaces() {
        let tokens = tokenize("echo hello2>file.txt");
        assert_eq!(
            tokens,
            vec![node("echo hello"), Token::RedirectStderr, path("file.txt")]
        );
    }

    #[test]
    fn no_redirect_stderr_at_en() {
        let tokens = tokenize("echo hello 2");
        assert_eq!(tokens, vec![node("echo hello 2")]);
    }

    #[test]
    fn no_redirect_when_2_not_followed_by_gt() {
        let tokens = tokenize("echo hello 2 file.txt");
        assert_eq!(tokens, vec![node("echo hello 2 file.txt")]);
    }

    // --- Append stdout ---
    #[test]
    fn append_stdout() {
        let tokens = tokenize("echo hello >> file.txt");
        assert_eq!(
            tokens,
            vec![node("echo hello"), Token::AppendStdout, path("file.txt")]
        );
    }

    #[test]
    fn append_stdout_no_spaces() {
        let tokens = tokenize("echo hello>>file.txt");
        assert_eq!(
            tokens,
            vec![node("echo hello"), Token::AppendStdout, path("file.txt")]
        );
    }

    #[test]
    fn append_stdout_number() {
        let tokens = tokenize("echo hello 1>> file.txt");
        assert_eq!(
            tokens,
            vec![node("echo hello"), Token::AppendStdout, path("file.txt")]
        );
    }

    #[test]
    fn append_stdout_number_no_spaces() {
        let tokens = tokenize("echo hello1>>file.txt");
        assert_eq!(
            tokens,
            vec![node("echo hello"), Token::AppendStdout, path("file.txt")]
        );
    }

    // --- Single quotes ---

    #[test]
    fn single_quotes_prevent_pipe_split() {
        let tokens = tokenize("echo 'hello|world'");
        assert_eq!(tokens, vec![node("echo 'hello|world'")]);
    }

    #[test]
    fn single_quotes_prevent_redirect() {
        let tokens = tokenize("echo 'hello>world'");
        assert_eq!(tokens, vec![node("echo 'hello>world'")]);
    }

    #[test]
    fn single_quotes_prevent_backslash_escape() {
        let tokens = tokenize(r"echo 'hello\nworld'");
        assert_eq!(tokens, vec![node(r"echo 'hello\nworld'")]);
    }

    // --- Double quotes ---

    #[test]
    fn double_quotes_prevent_pipe_split() {
        let tokens = tokenize(r#"echo "hello|world""#);
        assert_eq!(tokens, vec![node(r#"echo "hello|world""#)]);
    }

    #[test]
    fn double_quotes_prevent_redirect() {
        let tokens = tokenize(r#"echo "hello>world""#);
        assert_eq!(tokens, vec![node(r#"echo "hello>world""#)]);
    }

    // --- Backslash escaping ---

    #[test]
    fn backslash_escapes_pipe() {
        let tokens = tokenize(r"echo hello\|world");
        assert_eq!(tokens, vec![node(r"echo hello\|world")]);
    }

    #[test]
    fn backslash_escapes_redirect() {
        let tokens = tokenize(r"echo hello\>world");
        assert_eq!(tokens, vec![node(r"echo hello\>world")]);
    }

    #[test]
    fn backslash_in_double_quotes_escaped_char() {
        let tokens = tokenize(r#"echo "hello\"world""#);
        assert_eq!(tokens, vec![node(r#"echo "hello\"world""#)]);
    }

    #[test]
    fn backslash_in_double_quotes_non_escaped_char() {
        let tokens = tokenize(r#"echo "hello\aworld""#);
        // The tokenizer pushes both '\' and 'a' when char is not in the escaped set
        assert_eq!(tokens, vec![node(r#"echo "hello\\aworld""#)]);
    }

    // --- Combined operators ---

    #[test]
    fn pipe_then_redirect() {
        let tokens = tokenize("echo hello | echo world > file.txt");
        assert_eq!(
            tokens,
            vec![
                node("echo hello"),
                Token::Pipe,
                node("echo world"),
                Token::RedirectStdout,
                path("file.txt"),
            ]
        );
    }

    #[test]
    fn redirect_stderr_at_end() {
        let tokens = tokenize("echo hello world 2> err.log");
        assert_eq!(
            tokens,
            vec![
                node("echo hello world"),
                Token::RedirectStderr,
                path("err.log"),
            ]
        );
    }

    #[test]
    fn mixed_quotes() {
        let tokens = tokenize("echo 'hello' \"world\"");
        assert_eq!(tokens, vec![node("echo 'hello' \"world\"")]);
    }
}
