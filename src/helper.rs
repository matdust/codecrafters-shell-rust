pub struct CustomHelper {
    candidates: Vec<String>,
}

impl CustomHelper {
    fn candidates(&self) -> &[String] {
        &self.candidates
    }
}
impl Default for CustomHelper {
    fn default() -> Self {
        Self {
            candidates: [
                vec![
                    "echo".to_string(),
                    "ls".to_string(),
                    "cat".to_string(),
                    "cd".to_string(),
                    "exit".to_string(),
                ],
                crate::utils::files::get_path_exec(),
            ]
            .concat(),
        }
    }
}

impl rustyline::Helper for CustomHelper {}

impl rustyline::completion::Completer for CustomHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let start_position = line[..pos].rfind(' ').map_or(0, |i| i + 1);
        let word = &line[start_position..pos];
        if word.is_empty() {
            return Ok((start_position, vec![]));
        }
        let r = self
            .candidates()
            .iter()
            .filter(|cmd| cmd.starts_with(word))
            .map(|c| format!("{} ", c))
            .collect();

        Ok((start_position, r))
    }
}

impl rustyline::hint::Hinter for CustomHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        None
    }
    // for hints (shadow text)
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

impl rustyline::highlight::Highlighter for CustomHelper {}
impl rustyline::validate::Validator for CustomHelper {}
