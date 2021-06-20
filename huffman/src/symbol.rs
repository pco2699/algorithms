use std::cmp::Ordering;
use std::cmp::Ord;

#[derive(Debug, Clone, Eq)]
pub struct Symbol {
    pub weight: usize,
    pub max_merged_count: usize,
    pub min_merged_count: usize,
}

impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}