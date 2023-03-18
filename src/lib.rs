mod shuffle;

trait StrMatch {
    fn match_score(&self, what: Self) -> u32;
}

impl StrMatch for String {
    #[inline]
    fn match_score(&self, what: String) -> u32 {
        let combs = shuffle::shuffle(self);
        0
    }
}

/// Searches the given string `what` on `on` for a sorted vector of best matches.
fn search<'a, T>(what: &str, on: T) -> Vec<&'a str>
where
    T: IntoIterator<Item = &'a str> + Clone + Copy,
{
    // Convert what to lower
    let term = what.to_lowercase();

    // Get a map with all strings lowercase
    let lower = on.into_iter().map(str::to_lowercase);

    // Get a map with all match scores
    lower
        .into_iter()
        .collect::<Vec<String>>()
        .sort_by_key(|entry| StrMatch::match_score(entry, term.to_owned()));

    on.into_iter().collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_score() {
        let x = search("Rust", ["Python!", "Rust!"]);
        assert_eq!(x, ["Rust!", "Python!"]);
    }
}
