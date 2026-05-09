mod command;
mod commands;
mod interpreter;
mod lexer;
mod parser;
mod utils;

use rustyline::{completion::Completer, config::Configurer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = rustyline::Config::builder()
        .history_ignore_space(true)
        .history_ignore_dups(true)?
        .completion_type(rustyline::CompletionType::Circular)
        .auto_add_history(true)
        .color_mode(rustyline::ColorMode::Forced)
        .build();

    let mut rl = rustyline::Editor::<MyHelper, _>::with_config(cfg)?;

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    rl.set_completion_type(rustyline::CompletionType::Circular);
    rl.set_auto_add_history(true);
    rl.set_completion_show_all_if_ambiguous(true);
    rl.set_history_ignore_dups(true)?;
    rl.set_helper(Some(MyHelper));
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
struct MyHelper;

impl rustyline::Helper for MyHelper {}
impl rustyline::completion::Completer for MyHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let start = line[..pos].rfind(' ').map_or(0, |i| i + 1);
        let word = &line[start..pos];
        if word.is_empty() {
            return Ok((start, vec![]));
        }
        let candidates = vec![
            "echo".to_string(),
            "ls".to_string(),
            "cat".to_string(),
            "cd".to_string(),
            "exit".to_string(),
        ]
        .into_iter()
        .filter(|cmd| cmd.starts_with(word))
        .map(|c| format!("{} ", c))
        .collect();

        Ok((start, candidates))
    }
}

impl rustyline::hint::Hinter for MyHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        if let Ok((start, c)) = self.complete(line, pos, ctx)
            && let Some(e) = c.first()
        {
            let typed_len = pos - start;
            Some(e[typed_len..].to_string())
        } else {
            None
        }
    }
}

impl rustyline::highlight::Highlighter for MyHelper {}
impl rustyline::validate::Validator for MyHelper {}
