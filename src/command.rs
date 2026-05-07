use std::io::{Read, Write};

use crate::{
    commands::{self},
    lexer::Token,
};

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

    pub fn execute(&self, stdout: &mut dyn Write) {
        self.command_type.execute(stdout);
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum CommandType {
    Exit,
    NotFound { cmd: String },
    Echo { args: String },
    Type { args: String },
    Exec { cmd: String, args: String },
    Pwd,
    Cd { args: String },
}

impl CommandType {
    fn parse_command_and_args(command: &str) -> (String, String) {
        let mut single_quotes = false;
        let mut double_quotes = false;
        let mut buf = String::new();
        let mut r = Vec::<String>::new();

        for ch in command.trim().chars() {
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
            match ch {
                ' ' => {
                    if single_quotes || double_quotes {
                    } else {
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
        (r.first().unwrap().to_string(), r[1..].join(" "))
    }

    fn from_string(command: &str) -> Self {
        let (cmd, args) = Self::parse_command_and_args(command);

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

    pub fn execute(&self, stdout: &mut dyn Write) {
        match self {
            CommandType::Exit => commands::exit::execute(),
            CommandType::NotFound { cmd } => commands::notfound::execute(cmd, stdout),
            CommandType::Exec { cmd, args } => {
                commands::exec::execute(cmd, args.split_whitespace(), stdout)
            }
            CommandType::Type { args } => commands::r#type::execute(args, stdout),
            CommandType::Echo { args } => commands::echo::execute(args, stdout),
            CommandType::Pwd => commands::pwd::execute(stdout),
            CommandType::Cd { args } => commands::cd::execute(args),
        }
    }
}
