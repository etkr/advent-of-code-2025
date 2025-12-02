use crate::day2::database::Database;
use std::error::Error;

#[test]
fn day2_part1_example() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test.txt");
    let database: Database = input.parse()?;
    let invalid_ids = database.find_naive_invalid_product_ids();
    let result: u64 = invalid_ids.iter().map(u64::from).sum();
    assert_eq!(1227775554, result);
    Ok(())
}

#[test]
fn day2_part1_solution() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");
    let database: Database = input.parse()?;
    let invalid_ids = database.find_naive_invalid_product_ids();
    let result: u64 = invalid_ids.iter().map(u64::from).sum();
    assert_eq!(30323879646, result);
    Ok(())
}

#[test]
fn day2_part2_example() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test.txt");
    let database: Database = input.parse()?;
    let invalid_ids = database.find_invalid_product_id();
    let result: u64 = invalid_ids.iter().map(u64::from).sum();
    assert_eq!(4174379265, result);
    Ok(())
}

#[test]
fn day2_part2_solution() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");
    let database: Database = input.parse()?;
    let invalid_ids = database.find_invalid_product_id();
    let result: u64 = invalid_ids.iter().map(u64::from).sum();
    assert_eq!(43872163557, result);
    Ok(())
}
