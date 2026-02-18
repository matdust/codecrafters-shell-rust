use log::info;

const ESCAPED_CHARS_BY_BACKSLASH_IN_DOUBLE_QUOTES: [char; 5] = ['"', '\\', '$', '`', 'n'];

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Word(String),
    Operator(OperatorType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorType {
    Pipe,
    RedirectStdout,
}

pub fn tokenize(args: &str) -> Vec<Token> {
    let mut result = Vec::new();

    let mut command_to_append = String::new();

    let mut single_quotes = false;
    let mut double_quotes = false;
    let mut literal_mode = false;

    for ch in args.trim().chars() {
        if literal_mode {
            literal_mode = false;

            if double_quotes && !ESCAPED_CHARS_BY_BACKSLASH_IN_DOUBLE_QUOTES.contains(&ch) {
                command_to_append.push('\\');
            }

            command_to_append.push(ch);
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
            continue;
        }

        // handle single quotes mode
        if ch == '"' && !single_quotes {
            double_quotes = !double_quotes;
            continue;
        }

        // handle whitespace and operators
        match ch {
            // ' ' => {
            //     // check if we are in the any queotes mode if not then ' ' means next word
            //     if !(single_quotes || double_quotes) {
            //         // prevent appending empty String to result
            //         if command_to_append.is_empty() {
            //             continue;
            //         }
            //
            //         result.push(Token::Word(command_to_append.clone()));
            //         command_to_append.clear();
            //     } else {
            //         command_to_append.push(ch);
            //     }
            // }
            '|' => {
                if !(single_quotes || double_quotes) {
                    if command_to_append.is_empty() {
                        result.push(Token::Operator(OperatorType::Pipe));
                        continue;
                    }

                    result.push(Token::Word(command_to_append.trim().into()));
                    result.push(Token::Operator(OperatorType::Pipe));
                    command_to_append.clear();
                } else {
                    command_to_append.push(ch);
                }
            }
            '>' => {
                if !(single_quotes || double_quotes) {
                    if command_to_append.is_empty() {
                        result.push(Token::Operator(OperatorType::RedirectStdout));
                        continue;
                    }

                    result.push(Token::Word(command_to_append.trim().into()));
                    result.push(Token::Operator(OperatorType::RedirectStdout));
                    command_to_append.clear();
                } else {
                    command_to_append.push(ch);
                }
            }

            _ => command_to_append.push(ch),
        }
    }

    if !command_to_append.is_empty() {
        result.push(Token::Word(command_to_append.trim().into()));
    }

    // println!("Tokens: {:?}", result);
    result
}

// pub fn get_parsed_input(args: &str) -> Vec<String> {
//     let mut r = Vec::new();
//     let mut word_to_append = String::new();
//
//     let mut single_quotes = false;
//     let mut double_quotes = false;
//     let mut literal_mode = false;
//
//     for ch in args.trim().chars() {
//         if literal_mode {
//             literal_mode = false;
//
//             if double_quotes && !ESCAPED_CHARS_BY_BACKSLASH_IN_DOUBLE_QUOTES.contains(&ch) {
//                 word_to_append.push('\\');
//             }
//
//             word_to_append.push(ch);
//             continue;
//         }
//
//         // check if we can enter literal mode -- cannot be entered when single_quotes_mode is on
//         if ch == '\\' && !single_quotes {
//             literal_mode = true;
//             continue;
//         }
//
//         // handle single quotes
//         if ch == '\'' && !double_quotes {
//             single_quotes = !single_quotes;
//             continue;
//         }
//
//         // handle single quotes
//         if ch == '"' && !single_quotes {
//             double_quotes = !double_quotes;
//             continue;
//         }
//
//         // handle whitespace
//         if ch == ' ' {
//             if !(single_quotes || double_quotes) {
//                 // prevent appending empty String to result
//                 if word_to_append.is_empty() {
//                     continue;
//                 }
//
//                 // we are not in any quotes mode so ' ' means next word
//                 r.push(word_to_append.clone());
//                 word_to_append.clear();
//             } else {
//                 word_to_append.push(ch);
//             }
//         } else {
//             word_to_append.push(ch);
//         }
//     }
//
//     r.push(word_to_append);
//
//     r
// }
