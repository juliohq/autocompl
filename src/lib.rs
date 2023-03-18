use itertools::Itertools;

mod shuffle;

trait StrMatch {
    fn match_score(&self, what: Self) -> usize;
}

impl StrMatch for String {
    #[inline]
    fn match_score(&self, what: String) -> usize {
        let patterns = shuffle::shuffle(self);

        // Iterate over each pattern, compare with `what` and assign a score
        patterns
            .iter()
            .map(|pat| {
                if what == *pat {
                    println!("{}: {}", what, pat.len());
                    pat.len()
                } else {
                    0
                }
            })
            .sum::<usize>()
    }
}

/// Searches the given string `what` on `on` for a sorted vector of best matches.
fn search<'a, T>(what: &str, on: T) -> Vec<String>
where
    T: IntoIterator<Item = &'a str> + Clone + Copy,
{
    // Convert what to lower
    let term = what.to_lowercase();

    // Get a map with all match scores
    let mut sorted = on.into_iter().map(String::from).collect::<Vec<String>>();
    sorted.sort_by_key(|entry| StrMatch::match_score(&entry.to_lowercase(), term.to_owned()));
    sorted.reverse();

    sorted
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
