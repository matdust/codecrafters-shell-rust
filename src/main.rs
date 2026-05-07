mod command;
mod commands;
mod interpreter;
mod lexer;
mod parser;
mod utils;

use std::{
    fs::OpenOptions,
    io::{self, Write},
};

use log::info;

fn main() {
    setup_logger().unwrap();

    info!("Shell started");

    let mut input = String::new();
    loop {
        print!("$ ");

        if let Err(err) = io::stdout().flush() {
            eprintln!("error: {}", err)
        }

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read command");

        if input.is_empty() {
            continue;
        }

        info!("Raw input: {}", &input);

        let tokens = lexer::tokenize(&input);
        // println!("TOKENS: {:#?}", &tokens);

        let ast = parser::build_ast(tokens).expect("Failed to build AST");
        // println!("AST: {:#?}", &ast);

        interpreter::interpret(&ast, &mut io::stdout());
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open("output.log")?,
        )
        // .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
