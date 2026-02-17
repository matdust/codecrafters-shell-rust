use log::info;

use crate::{
    commands::{self},
    lexer::Token,
};

pub const COMMANDS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];

#[derive(Debug, PartialEq)]
pub struct Command {
    command_type: CommandType,
    stdout: Option<Box<Stdio>>,
    stdin: Option<Box<Stdio>>,
    stderr: Option<Box<Stdio>>,
}

impl Command {
    pub fn from_token(token: &Token) -> Self {
        Command {
            command_type: CommandType::from_token(token),
            stdout: None,
            stdin: None,
            stderr: None,
        }
    }

    pub fn from_string(input: &str) -> Self {
        Command {
            command_type: CommandType::from_string(input),
            stdout: None,
            stdin: None,
            stderr: None,
        }
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
}

// Handle output redirection and input redirection
#[derive(Debug, PartialEq)]
pub enum Stdio {
    Console,
    File { path: String },
}

// impl CommandType {
//     pub fn new(input: &str) -> CommandType {
//         let (cmd, args) = crate::utils::string::get_cmd_and_args(input);
//
//         info!("Parsed input\n cmd: {}\n args: {:?}\n", &cmd, &args);
//
//         if COMMANDS.contains(&cmd.as_str()) {
//             match cmd.as_str() {
//                 "exit" => CommandType::Exit,
//                 "echo" => CommandType::Echo {
//                     args: args.join(" "),
//                 },
//                 "type" => CommandType::Type {
//                     args: args.join(" "),
//                 },
//                 "pwd" => CommandType::Pwd,
//                 "cd" => CommandType::Cd {
//                     args: args.first().cloned().unwrap_or(String::from("~")),
//                 },
//                 _ => CommandType::NotFound { cmd },
//             }
//         } else {
//             match crate::utils::files::find_exe_in_env(&cmd) {
//                 Some(_) => CommandType::Exec { cmd, args },
//                 None => CommandType::NotFound { cmd },
//             }
//         }
//     }
//
//     pub fn execute(&self) {
//         match self {
//             CommandType::Exit => commands::exit::execute(),
//             CommandType::NotFound { cmd } => commands::notfound::execute(cmd),
//             CommandType::Exec { cmd, args } => commands::exec::execute(cmd, args),
//             CommandType::Type { args } => commands::r#type::execute(args),
//             CommandType::Echo { args } => commands::echo::execute(args),
//             CommandType::Pwd => commands::pwd::execute(),
//             CommandType::Cd { args } => commands::cd::execute(args),
//         }
//     }
// }
