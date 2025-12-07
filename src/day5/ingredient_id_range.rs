use crate::day5::ingredient_id::IngredientId;
use std::cmp::{max, min};
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct IngredientIdRange {
    start: IngredientId,
    end: IngredientId,
}

impl IngredientIdRange {
    pub fn new(start: u64, end: u64) -> Self {
        IngredientIdRange {
            start: start.into(),
            end: end.into(),
        }
    }
    pub fn is_ingredient_fresh(&self, ingredient_id: &IngredientId) -> bool {
        ingredient_id >= &self.start && ingredient_id <= &self.end
    }

    pub fn overlaps(&self, other: &IngredientIdRange) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    pub fn overlaps_pair(pair: &[IngredientIdRange]) -> bool {
        assert_eq!(2, pair.len());
        pair[0].start <= pair[1].end && pair[1].start <= pair[0].end
    }

    pub fn merge(&self, other: &IngredientIdRange) -> Self {
        let start = min(self.start, other.start);
        let end = max(self.end, other.end);
        IngredientIdRange { start, end }
    }

    pub fn merge_pair(pair: &[IngredientIdRange]) -> Vec<IngredientIdRange> {
        if pair.len() < 2 {
            return pair.to_vec();
        }
        let first = &pair[0];
        let second = &pair[1];
        if first.overlaps(second) {
            let merged = first.merge(second);
            vec![merged]
        } else {
            vec![first.clone(), second.clone()]
        }
    }

    pub fn start(&self) -> IngredientId {
        self.start
    }
    pub fn end(&self) -> IngredientId {
        self.end
    }

    pub fn size(&self) -> u64 {
        self.end - self.start + 1
    }
}

impl FromStr for IngredientIdRange {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split_once('-');
        if let Some((start, end)) = parts {
            return Ok(Self {
                start: start.parse()?,
                end: end.parse()?,
            });
        }
        Err("Could not parse ingredient range".into())
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::ingredient_id::IngredientId;
    use crate::day5::ingredient_id_range::IngredientIdRange;

    #[test]
    fn test_ingredient_id_range() {
        let range = IngredientIdRange::new(45, 55);
        let id = IngredientId::from(45);
        assert!(range.is_ingredient_fresh(&id));
    }

    #[test]
    fn test_ingredient_id_not_in_range() {
        let range = IngredientIdRange::new(45, 55);
        let id = IngredientId::from(100);
        assert!(!range.is_ingredient_fresh(&id));
    }

    #[test]
    fn test_ingredient_id_range_overlaps() {
        let range = IngredientIdRange::new(45, 55);
        let other = IngredientIdRange::new(50, 100);
        assert!(range.overlaps(&other));
    }

    #[test]
    fn test_ingredient_id_range_overlaps_at_end() {
        let range = IngredientIdRange::new(45, 55);
        let other = IngredientIdRange::new(55, 100);
        assert!(range.overlaps(&other));
    }

    #[test]
    fn test_ingredient_id_range_does_not_overlap() {
        let range = IngredientIdRange::new(45, 55);
        let other = IngredientIdRange::new(60, 100);
        assert!(!range.overlaps(&other));
    }

    #[test]
    fn test_ingredient_id_range_merge() {
        let range = IngredientIdRange::new(45, 55);
        let other = IngredientIdRange::new(50, 100);
        let merged = range.merge(&other);
        let expected = IngredientIdRange::new(45, 100);
        assert_eq!(expected, merged);
    }

    #[test]
    fn test_ingredient_id_size() {
        assert_eq!(3, IngredientIdRange::new(3, 5).size());
    }
}
