use crate::day1::dial::Dial;
use crate::day1::rotation_direction::RotationDirection;
use std::error::Error;
use std::str::FromStr;

mod dial;
mod rotation_direction;

pub fn parse_input(input: &str) -> Result<Vec<RotationDirection>, Box<dyn Error>> {
    let input = input
        .lines()
        .map(RotationDirection::from_str)
        .collect::<Result<Vec<RotationDirection>, _>>()?;
    Ok(input)
}

pub fn find_rotations_where_zero(rotations: &[RotationDirection]) -> usize {
    let mut poisitions: Vec<Dial> = vec![Dial::new(50)];

    for rotation in rotations {
        if let Some(previous) = poisitions.last() {
            let rotation = previous.rotate(rotation);
            poisitions.push(rotation);
        }
    }

    poisitions.into_iter().filter(Dial::is_zero).count()
}

pub fn find_rotations_where_zer0_has_passed(rotations: &[RotationDirection]) -> i32 {
    let mut poisitions: Vec<Dial> = vec![Dial::new(50)];
    for rotation in rotations {
        if let Some(previous) = poisitions.last() {
            let rotation = previous.rotate(rotation);
            poisitions.push(rotation);
        }
    }
    poisitions.iter().map(Dial::times_passed_zero).sum()
}

#[cfg(test)]
mod tests;
