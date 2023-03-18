mod shuffle;

trait StrMatch {
    /// Iterate over each pattern, compare with `what` and assign a score
    fn match_score(&self, what: Self) -> usize;
}

impl StrMatch for String {
    #[inline]
    fn match_score(&self, what: String) -> usize {
        shuffle::shuffle(self)
            .iter()
            .map(|pat| if what == *pat { pat.len() } else { 0 })
            .sum::<usize>()
    }
}

/// Searches the given string `what` on `on` for a sorted vector of best matches.
pub fn search(what: &str, on: Vec<String>) -> Vec<String> {
    // Convert what to lower
    let term = what.to_lowercase();

    // Get a map with all match scores
    let mut sorted = on.into_iter().map(String::from).collect::<Vec<String>>();
    sorted
        .sort_by_cached_key(|entry| StrMatch::match_score(&entry.to_lowercase(), term.to_owned()));
    sorted.reverse();

    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_score() {
        let x = search("Rust", vec!["Python!".into(), "Rust!".into()]);
        assert_eq!(x, ["Rust!", "Python!"]);
    }
}
