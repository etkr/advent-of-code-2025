use std::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum RotationDirection {
    Right(i32),
    Left(i32),
}

impl FromStr for RotationDirection {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s.chars().nth(0);
        let rest = s.chars().skip(1);
        let amount: i32 = rest.collect::<String>().parse()?;
        match first {
            Some('L') => Ok(RotationDirection::Left(amount)),
            Some('R') => Ok(RotationDirection::Right(amount)),
            None | Some(_) => Err("Unknown direction".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day1::rotation_direction::RotationDirection;

    #[test]
    fn test_direction_parsing() {
        let input = "L32";
        let result: RotationDirection = input.parse().unwrap();
        assert_eq!(RotationDirection::Left(32), result);
    }
}
