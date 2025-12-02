use crate::day1::{find_rotations_where_zer0_has_passed, find_rotations_where_zero, parse_input};
use std::error::Error;

#[test]
fn day1_part1_example() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test.txt");
    let rotations = parse_input(input)?;
    let result = find_rotations_where_zero(&rotations);
    assert_eq!(3, result);
    Ok(())
}

#[test]
fn day1_part1_input() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");
    let rotations = parse_input(input)?;
    let result = find_rotations_where_zero(&rotations);
    assert_eq!(1036, result);
    Ok(())
}

#[test]
fn day1_part2_example() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test.txt");
    let rotations = parse_input(input)?;
    let result = find_rotations_where_zer0_has_passed(&rotations);
    assert_eq!(6, result);
    Ok(())
}

#[test]
fn day1_part2_input() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");
    let rotations = parse_input(input)?;
    let result = find_rotations_where_zer0_has_passed(&rotations);
    assert_eq!(2124, result);
    Ok(())
}
