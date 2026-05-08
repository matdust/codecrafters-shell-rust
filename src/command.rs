use std::io::Write;

use crate::commands::{self};

pub const COMMANDS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];
#[derive(PartialEq, Debug, Clone)]
pub struct Command {
    command_type: CommandType,
}

impl Command {
    pub fn from_string(input: &str) -> Self {
        Command {
            command_type: CommandType::from_string(input),
        }
    }

    pub fn execute(&self, stdout: &mut dyn Write, stderr: &mut dyn Write) {
        self.command_type.execute(stdout, stderr);
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum CommandType {
    Exit,
    NotFound { cmd: String },
    Echo { args: Vec<String> },
    Type { args: Vec<String> },
    Exec { cmd: String, args: Vec<String> },
    Pwd,
    Cd { args: Vec<String> },
}

impl CommandType {
    fn parse_command_and_args(command: &str) -> (String, Vec<String>) {
        let mut single_quotes = false;
        let mut double_quotes = false;
        let mut literal_mode = false;
        let mut buf = String::new();
        let mut r = Vec::<String>::new();

        for ch in command.trim().chars() {
            if literal_mode {
                literal_mode = false;

                if double_quotes
                    && !crate::lexer::ESCAPED_CHARS_BY_BACKSLASH_IN_DOUBLE_QUOTES.contains(&ch)
                {
                    buf.push('\\');
                }

                buf.push(ch);
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

            // handle double quotes mode
            if ch == '"' && !single_quotes {
                double_quotes = !double_quotes;
                continue;
            }
            match ch {
                ' ' => {
                    if single_quotes || double_quotes {
                        buf.push(ch);
                    } else {
                        if buf.is_empty() {
                            continue;
                        }
                        r.push(buf.clone());
                        buf.clear();
                    }
                }
                _ => buf.push(ch),
            }
        }

        if !buf.is_empty() {
            r.push(buf);
        }
        (
            r.first().unwrap().to_string(),
            r.iter().skip(1).cloned().collect(),
        )
    }

    fn from_string(command: &str) -> Self {
        // println!("[RAW] CMD: {}", command);
        let (cmd, args) = Self::parse_command_and_args(command);
        // println!("[PARSED] CMD: {}, ARGS: {:?}", cmd, args);

        if COMMANDS.contains(&cmd.as_str()) {
            match cmd.as_str() {
                "exit" => CommandType::Exit,
                "echo" => CommandType::Echo { args },
                "type" => CommandType::Type { args },
                "pwd" => CommandType::Pwd,
                "cd" => CommandType::Cd { args },
                _ => CommandType::NotFound { cmd },
            }
        } else {
            match crate::utils::files::find_exe_in_env(&cmd) {
                Some(_) => CommandType::Exec { cmd, args },
                None => CommandType::NotFound { cmd },
            }
        }
    }

    pub fn execute(&self, stdout: &mut dyn Write, stderr: &mut dyn Write) {
        match self {
            CommandType::Exit => commands::exit::execute(),
            CommandType::NotFound { cmd } => commands::notfound::execute(cmd, stdout),
            CommandType::Exec { cmd, args } => commands::exec::execute(
                cmd,
                args.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                stdout,
                stderr,
            ),
            CommandType::Type { args } => commands::r#type::execute(
                &args.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                stdout,
            ),
            CommandType::Echo { args } => commands::echo::execute(
                &args.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                stdout,
            ),
            CommandType::Pwd => commands::pwd::execute(stdout),
            CommandType::Cd { args } => {
                commands::cd::execute(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>())
            }
        }
    }
}
