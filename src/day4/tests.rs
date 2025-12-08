use crate::day4::paper_storage::PaperStorage;
use std::error::Error;
use std::str::FromStr;

#[test]
fn day4_part1_example1() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test.txt");
    let paper_storage: PaperStorage = input.parse()?;
    let result = paper_storage.n_forklift_accessible_paper_rolls();
    assert_eq!(13, result);
    Ok(())
}

#[test]
fn day4_part1_solution() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");
    let paper_storage = PaperStorage::from_str(input)?;
    let result = paper_storage.n_forklift_accessible_paper_rolls();
    assert_eq!(1523, result);
    Ok(())
}
