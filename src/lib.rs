pub mod skim;

#[cfg(not(feature = "compact"))]
type IndexType = usize;
#[cfg(not(feature = "compact"))]
type ScoreType = i64;

#[cfg(feature = "compact")]
type IndexType = u32;
#[cfg(feature = "compact")]
type ScoreType = i32;

pub trait FuzzyMatcher {
    fn fuzzy_indices(&self, choice: &str, pattern: &str) -> Option<(ScoreType, Vec<IndexType>)>;
}
