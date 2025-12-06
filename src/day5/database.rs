use crate::day5::ingredient_id::IngredientId;
use crate::day5::ingredient_id_range::IngredientIdRange;
use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

pub struct Database {
    fresh_ingredients: HashSet<IngredientIdRange>,
    ingredients: Vec<IngredientId>,
}

impl Database {
    pub fn new(fresh_ingredients: Vec<IngredientIdRange>, ingredients: Vec<IngredientId>) -> Self {
        let set = HashSet::from_iter(fresh_ingredients);
        let reduced_fresh_ingredients = Self::reduce_fresh_ingredient_ranges(&set);
        Self {
            fresh_ingredients: reduced_fresh_ingredients,
            ingredients,
        }
    }

    pub fn find_fresh_ingredients(&self) -> Vec<IngredientId> {
        self.ingredients
            .iter()
            .filter(|id| {
                self.fresh_ingredients
                    .iter()
                    .any(|range| range.is_ingredient_fresh(id))
            })
            .copied()
            .collect()
    }

    /// Reduce a set of ingredient id ranges until the minimal set of non overlapping ranges
    ///
    /// # Arguments
    ///
    /// * `fresh_ingredient`: set of id ranges
    ///
    /// returns: HashSet<IngredientIdRange, RandomState>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn reduce_fresh_ingredient_ranges(
        ingredient_ids: &HashSet<IngredientIdRange>,
    ) -> HashSet<IngredientIdRange> {
        let mut new: HashSet<IngredientIdRange> = HashSet::new();
        let mut found_any_overlap = false;
        for range_a in ingredient_ids {
            let mut found_overlap = false;
            for range_b in ingredient_ids {
                if range_a == range_b {
                    continue;
                }
                if range_a.overlaps(range_b) {
                    new.insert(range_a.merge(range_b));
                    found_overlap = true;
                }
            }
            if !found_overlap {
                new.insert(range_a.clone());
            } else {
                found_any_overlap = true;
            }
        }
        if !found_any_overlap {
            return new;
        }

        Self::reduce_fresh_ingredient_ranges(&new)
    }

    pub fn n_fresh_ingredients_in_index(&self) -> u64 {
        self.fresh_ingredients
            .iter()
            .map(IngredientIdRange::size)
            .sum()
    }
}

impl FromStr for Database {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ingredients: Vec<IngredientId> = Vec::new();
        let mut fresh_ingredients: Vec<IngredientIdRange> = Vec::new();

        for line in s.lines() {
            if line.trim().is_empty() {
                continue;
            }
            if line.contains('-') {
                let range: IngredientIdRange = line.parse()?;
                fresh_ingredients.push(range);
            } else {
                let id: IngredientId = line.parse()?;
                ingredients.push(id);
            }
        }

        Ok(Database::new(fresh_ingredients, ingredients))
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::database::Database;
    use crate::day5::ingredient_id::IngredientId;
    use crate::day5::ingredient_id_range::IngredientIdRange;
    use std::collections::HashSet;
    use std::error::Error;

    #[test]
    fn test_parse_database() -> Result<(), Box<dyn Error>> {
        let test_input = r#"
        1-2
        3-4

        1
        2
        3
        "#;
        let database: Database = test_input.parse()?;
        let expected_fresh_ingredients = HashSet::from_iter(vec![
            IngredientIdRange::new(1, 2),
            IngredientIdRange::new(3, 4),
        ]);
        assert_eq!(expected_fresh_ingredients, database.fresh_ingredients);
        let expected_ingredients: Vec<IngredientId> = vec![1.into(), 2.into(), 3.into()];
        assert_eq!(expected_ingredients, database.ingredients);
        Ok(())
    }

    #[test]
    fn test_reduce_fresh_ingredients() {
        let origin: HashSet<IngredientIdRange> = HashSet::from_iter(vec![
            IngredientIdRange::new(3, 5),
            IngredientIdRange::new(10, 14),
            IngredientIdRange::new(16, 20),
            IngredientIdRange::new(12, 18),
        ]);
        let result = Database::reduce_fresh_ingredient_ranges(&origin);
        let expected = HashSet::from_iter(vec![
            IngredientIdRange::new(3, 5),
            IngredientIdRange::new(10, 20),
        ]);
        assert_eq!(expected, result)
    }
}
