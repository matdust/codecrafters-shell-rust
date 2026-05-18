use std::cell::Cell;

use rustyline::completion::Pair;

pub struct CustomHelper {
    candidates: Vec<String>,
    last_completion_pos: Cell<Option<usize>>,
}

impl CustomHelper {
    fn candidates(&self) -> &[String] {
        &self.candidates
    }

    pub(crate) fn longest_common_prefix(candidates: &[Pair]) -> String {
        if candidates.is_empty() {
            return String::new();
        }
        let first = &candidates[0].replacement;
        let mut len = first.len();
        for c in &candidates[1..] {
            len = len.min(
                first
                    .chars()
                    .zip(c.replacement.chars())
                    .take_while(|(a, b)| a == b)
                    .count(),
            );
        }
        first[..len].to_string()
    }
}
impl Default for CustomHelper {
    fn default() -> Self {
        let mut path_candidates = crate::utils::files::get_path_exec();
        path_candidates.append(&mut vec![
            "echo".to_string(),
            "ls".to_string(),
            "cat".to_string(),
            "cd".to_string(),
            "exit".to_string(),
        ]);
        path_candidates.sort_by_key(|a| a.to_lowercase());
        path_candidates.dedup_by(|a, b| a.to_lowercase() == b.to_lowercase());

        Self {
            candidates: path_candidates,
            last_completion_pos: Cell::new(None),
        }
    }
}

impl rustyline::Helper for CustomHelper {}

impl rustyline::completion::Completer for CustomHelper {
    type Candidate = Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let start_position = line[..pos].rfind(' ').map_or(0, |i| i + 1);
        let prefix = &line[start_position..pos];
        if prefix.is_empty() {
            return Ok((start_position, vec![]));
        }
        let r = self
            .candidates()
            .iter()
            .filter(|cmd| cmd.starts_with(prefix))
            .map(|w| Pair {
                display: w.clone(),
                replacement: w.clone(),
            })
            .collect::<Vec<Pair>>();

        if r.is_empty() {
            self.last_completion_pos.set(None);
            return Ok((start_position, vec![]));
        }

        // Compute LCP of all candidates
        let lcp = CustomHelper::longest_common_prefix(&r);

        // If LCP advances beyond the current prefix — complete to LCP only, no list
        if lcp.len() > prefix.len() {
            self.last_completion_pos
                .set(Some(start_position + lcp.len()));
            return Ok((
                start_position,
                vec![Pair {
                    display: lcp.clone(),
                    replacement: lcp,
                }],
            ));
        }

        // LCP didn't advance — show the full list
        self.last_completion_pos.set(None);
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
