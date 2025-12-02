use std::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct ProductId(u64);

impl ProductId {
    pub fn naive_is_invalid(&self) -> bool {
        let str = self.0.to_string();
        let first_half = &str[0..str.len() / 2];
        let second_half = &str[str.len() / 2..];
        first_half == second_half
    }

    pub fn is_invalid(&self) -> bool {
        let product_id = self.0.to_string();
        let until_half = 1..product_id.len() / 2 + 1;
        for i in until_half {
            // skip patterns that doesn't fit the string
            if !product_id.len().is_multiple_of(i) {
                continue;
            }
            let pattern = &product_id[0..i];
            let times = product_id.len() / pattern.len();
            let repeated_pattern = pattern.repeat(times);
            if repeated_pattern == product_id {
                return true;
            }
        }
        false
    }
}

impl From<ProductId> for u64 {
    fn from(value: ProductId) -> Self {
        value.0
    }
}

impl From<&ProductId> for u64 {
    fn from(value: &ProductId) -> Self {
        value.0
    }
}

#[derive(Debug, PartialEq)]
pub struct ProductIdRange {
    from: u64,
    to: u64,
}

impl ProductIdRange {
    pub fn new(from: u64, to: u64) -> Self {
        ProductIdRange { from, to }
    }
}

impl FromStr for ProductIdRange {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.trim().split_once('-');
        if let Some((from, to)) = split {
            let product_id_range = ProductIdRange {
                from: from.parse()?,
                to: to.parse()?,
            };
            return Ok(product_id_range);
        };
        Err("Error parsing ProductIdRange".into())
    }
}

impl IntoIterator for &ProductIdRange {
    type Item = ProductId;
    type IntoIter = ProductIdIterator;

    fn into_iter(self) -> Self::IntoIter {
        ProductIdIterator {
            current: self.from,
            to: self.to,
        }
    }
}

pub struct ProductIdIterator {
    current: u64,
    to: u64,
}

impl Iterator for ProductIdIterator {
    type Item = ProductId;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.to {
            return None;
        }
        let next = self.current;
        self.current += 1;
        Some(ProductId(next))
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::product_id::{ProductId, ProductIdRange};
    use std::error::Error;

    #[test]
    fn test_22_is_invalid() {
        assert!(ProductId(22).naive_is_invalid());
    }

    #[test]
    fn test_38593856_is_not_invalid() {
        assert!(!ProductId(38593856).naive_is_invalid());
    }

    #[test]
    fn test_38593859_is_invalid() {
        assert!(ProductId(38593859).naive_is_invalid());
    }

    #[test]
    fn can_parse_product_id_range() -> Result<(), Box<dyn Error>> {
        let input = "1188511880-1188511890";
        let range: ProductIdRange = input.parse()?;
        assert_eq!(ProductIdRange::new(1188511880, 1188511890), range);
        Ok(())
    }

    #[test]
    fn product_id_iterator_returns_to_and_from_iterator() {
        let product_id_range = ProductIdRange::new(12, 15);
        let result: Vec<ProductId> = product_id_range.into_iter().collect();
        let expected = vec![ProductId(12), ProductId(13), ProductId(14), ProductId(15)];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_111_is_invalid() {
        assert!(ProductId(111).is_invalid());
    }

    #[test]
    fn is_1_multiple_of_3() {
        let size: usize = 3;
        assert!(size.is_multiple_of(1));
    }
}
