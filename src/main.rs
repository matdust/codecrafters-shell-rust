mod command;
mod commands;
mod helper;
mod interpreter;
mod lexer;
mod parser;
mod utils;

use rustyline::config::Configurer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("{:?}", utils::files::get_path_exec());
    // return Ok(());
    let cfg = rustyline::Config::builder()
        .history_ignore_space(true)
        .history_ignore_dups(true)?
        .completion_type(rustyline::CompletionType::Circular)
        .auto_add_history(true)
        .color_mode(rustyline::ColorMode::Forced)
        .build();

    let mut rl = rustyline::Editor::<crate::helper::MyHelper, _>::with_config(cfg)?;

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    rl.set_completion_type(rustyline::CompletionType::Circular);
    rl.set_auto_add_history(true);
    rl.set_completion_show_all_if_ambiguous(true);
    rl.set_history_ignore_dups(true)?;
    rl.set_helper(Some(crate::helper::MyHelper));
    loop {
        let readline = rl.readline("$ ");
        match readline {
            Ok(line) => {
                let tokens = lexer::tokenize(&line);
                // println!("TOKENS: {:#?}", &tokens);

                let ast = parser::build_ast(tokens).expect("Failed to build AST");
                // println!("AST: {:#?}", &ast);

                interpreter::interpret(&ast, &mut std::io::stdout(), &mut std::io::stderr());
            }
            Err(_) => {
                break;
            }
        }
    }

    rl.save_history("history.txt")?;
    Ok(())
}
