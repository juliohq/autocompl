mod shuffle;
mod string_match;

use itertools::Itertools;
use string_match::StringMatch;

/// Searches the given string `what` on `on` for a sorted vector of best matches.
#[inline]
pub fn search(what: &str, on: Vec<String>) -> Vec<String> {
    // Convert `what` to lower
    let term = what.to_lowercase();

    // Trim whitespace from `what`
    #[cfg(feature = "ignore-whitespace")]
    let term: String = term.chars().filter(|char| !char.is_whitespace()).collect();

    // Extract first character
    let first_char = term.chars().next();

    if first_char.is_none() {
        // `what` is empty
        return vec![];
    }

    // Get a map with all match scores
    let sorted = on
        .into_iter()
        .filter(|word| {
            // Filter words by all containined characters in `what`
            let lower_word = word.to_lowercase();
            term.chars().all(|char| lower_word.contains(char))
        })
        .sorted_by_cached_key(|entry| {
            StringMatch::match_score(entry.to_lowercase(), term.to_owned())
        })
        .rev()
        .collect::<Vec<String>>();

    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_score() {
        let x = search("Rust", vec!["Python!".into(), "Rust!".into()]);
        assert_eq!(x, ["Rust!"]);
    }
}
