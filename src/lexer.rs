const ESCAPED_CHARS_BY_BACKSLASH_IN_DOUBLE_QUOTES: [char; 5] = ['"', '\\', '$', '`', 'n'];

// #[derive(Debug, PartialEq, Clone)]
// pub enum Token {
//     Word(String),
//     Operator(OperatorType),
// }

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Token {
    Node(crate::command::Command),
    Path(String),
    Operator(OperatorType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorType {
    Pipe,
    RedirectStdout,
}

pub fn tokenize(args: &str) -> Vec<Token> {
    let mut result = Vec::new();

    let mut new_token = String::new();

    let mut single_quotes = false;
    let mut double_quotes = false;
    let mut literal_mode = false;

    for ch in args.trim().chars() {
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
            literal_mode = true;
            continue;
        }

        // handle single quotes mode
        if ch == '\'' && !double_quotes {
            single_quotes = !single_quotes;
            new_token.push(ch);
            continue;
        }

        // handle single quotes mode
        if ch == '"' && !single_quotes {
            double_quotes = !double_quotes;
            new_token.push(ch);
            continue;
        }

        match ch {
            '|' => {
                if !(single_quotes || double_quotes) {
                    if new_token.is_empty() {
                        result.push(Token::Operator(OperatorType::Pipe));
                        continue;
                    }

                    result.push(Token::Node(crate::command::Command::from_string(
                        new_token.trim(),
                    )));
                    result.push(Token::Operator(OperatorType::Pipe));
                    new_token.clear();
                } else {
                    new_token.push(ch);
                }
            }

            '>' => {
                if !(single_quotes || double_quotes) {
                    if new_token.is_empty() {
                        result.push(Token::Operator(OperatorType::RedirectStdout));
                        continue;
                    }

                    if new_token.ends_with("1") {
                        new_token.pop();
                    }

                    result.push(Token::Node(crate::command::Command::from_string(
                        new_token.trim(),
                    )));
                    result.push(Token::Operator(OperatorType::RedirectStdout));
                    new_token.clear();
                } else {
                    new_token.push(ch);
                }
            }

            _ => new_token.push(ch),
        }
    }

    if !new_token.is_empty() {
        let Some(token) = result.last() else {
            result.push(Token::Node(crate::command::Command::from_string(
                new_token.trim(),
            )));
            return result;
        };

        println!("new_token: {new_token}");
        let token_to_add = match token {
            Token::Operator(operator_type) => match operator_type {
                OperatorType::RedirectStdout => Token::Path(new_token.trim().to_string()),
                _ => Token::Node(crate::command::Command::from_string(new_token.trim())),
            },
            _ => Token::Node(crate::command::Command::from_string(new_token.trim())),
        };
        result.push(token_to_add);
    }

    result
}
