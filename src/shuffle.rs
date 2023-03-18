use itertools::Itertools;

/// Collects a vector.
#[inline]
pub fn shuffle<T: Into<String>>(word: T) -> Vec<String> {
    let word: String = word.into();

    // Collects all combinations
    let mut combs = vec![];

    for char in 1..=word.len() {
        combs.extend(word.chars().combinations(char).collect_vec());
    }

    // Collect into strings and reverse for extact match first
    combs
        .iter()
        .map(String::from_iter)
        .rev()
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let word = "abc";
        assert_eq!(shuffle(word), ["abc", "bc", "ac", "ab", "c", "b", "a"]);
    }
}
