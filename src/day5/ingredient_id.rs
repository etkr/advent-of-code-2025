use std::num::ParseIntError;
use std::ops::Sub;
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub struct IngredientId(u64);

impl FromStr for IngredientId {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.trim().parse()?))
    }
}

impl Sub for IngredientId {
    type Output = u64;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0.sub(rhs.0)
    }
}

impl From<u64> for IngredientId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}
