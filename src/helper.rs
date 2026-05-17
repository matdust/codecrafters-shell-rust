use rustyline::completion::Candidate;

pub struct MyHelper;

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
        let mut candidates = vec![
            "echo".to_string(),
            "ls".to_string(),
            "cat".to_string(),
            "cd".to_string(),
            "exit".to_string(),
        ];
        let path_candidates = crate::utils::files::get_path_exec();
        candidates.extend(path_candidates);

        let r = candidates
            .into_iter()
            .filter(|cmd| cmd.starts_with(word))
            .map(|c| format!("{} ", c))
            .collect();

        Ok((start, r))
    }
}

impl rustyline::hint::Hinter for MyHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        None
    }
    // fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
    //     if let Ok((start, c)) = self.complete(line, pos, ctx)
    //         && let Some(e) = c.first()
    //     {
    //         let typed_len = pos - start;
    //         Some(e[typed_len..].to_string())
    //     } else {
    //         None
    //     }
    // }
}

impl rustyline::highlight::Highlighter for MyHelper {}
impl rustyline::validate::Validator for MyHelper {}
