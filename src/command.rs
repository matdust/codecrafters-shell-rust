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
    pub fn from_token(token: &Token) -> Self {
        Command {
            command_type: CommandType::from_token(token),
        }
    }

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
    fn from_token(token: &Token) -> Self {
        match token {
            Token::Word(command) => CommandType::from_string(command),
            Token::Operator(_) => {
                log::error!("Unexpected operator {:?} when parsing command", token);
                unreachable!()
            }
        }
    }

    fn from_string(command: &str) -> Self {
        let (cmd, args) = command
            .split_once(' ')
            .map_or((command.to_owned(), "".to_string()), |(c, a)| {
                (c.to_string(), a.to_string())
            });

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
