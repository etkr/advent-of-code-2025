use crate::day5::database::Database;
use std::error::Error;

#[test]
fn day5_part1_example() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test.txt");
    let database: Database = input.parse()?;
    let fresh_ingredients = database.find_fresh_ingredients();
    assert_eq!(3, fresh_ingredients.len());
    Ok(())
}

#[test]
fn day5_part1_solution() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");
    let database: Database = input.parse()?;
    let fresh_ingredients = database.find_fresh_ingredients();
    assert_eq!(615, fresh_ingredients.len());
    Ok(())
}

#[test]
fn day5_part2_example() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test.txt");
    let database: Database = input.parse()?;
    let fresh_ingredients = database.n_fresh_ingredients_in_index();
    assert_eq!(14, fresh_ingredients);
    Ok(())
}

#[test]
fn day5_part2_solution() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");
    let database: Database = input.parse()?;
    let fresh_ingredients = database.n_fresh_ingredients_in_index();
    assert_eq!(353716783056994, fresh_ingredients);
    Ok(())
}
