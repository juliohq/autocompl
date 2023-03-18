use crate::shuffle;

pub trait StringMatch {
    /// Iterate over each pattern, compare with `what` and assign a score
    fn match_score(&self, what: Self) -> usize;
}

impl StringMatch for String {
    #[inline]
    fn match_score(&self, what: String) -> usize {
        shuffle::shuffle(self)
            .iter()
            .map(|pat| if what == *pat { pat.len() } else { 0 })
            .sum::<usize>()
    }
}
