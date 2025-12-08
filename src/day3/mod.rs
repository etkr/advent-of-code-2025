mod monotonic_stack;

use crate::day3::monotonic_stack::MonotonicStack;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

pub struct Bank {
    input: String,
    batteries: Vec<char>,
    index: HashMap<char, Vec<usize>>,
}

impl Bank {}

impl Bank {
    fn generate_index(battery: &[char]) -> HashMap<char, Vec<usize>> {
        let mut index: HashMap<char, Vec<usize>> = HashMap::new();
        for i in 0..10 {
            let char = i.to_string().chars().nth(0);
            if let Some(v) = char {
                index.insert(v, Vec::new());
            }
        }
        for (i, battery) in battery.iter().enumerate() {
            match index.get_mut(battery) {
                None => panic!("hashmap not initialized"),
                Some(vec) => vec.push(i),
            }
        }
        for list in index.values_mut() {
            list.sort();
            list.reverse()
        }

        index
    }

    pub fn new(batteries: &[char]) -> Self {
        let index = Self::generate_index(&batteries);
        let input: String = batteries.into_iter().collect();
        let batteries = batteries.to_owned();
        Self {
            batteries,
            index,
            input,
        }
    }

    fn joltage_exists(first: &[usize], second: &[usize]) -> bool {
        for a in first {
            for b in second {
                if a < b {
                    return true;
                }
            }
        }
        false
    }

    fn find_digits(str: &str) -> Option<(char, char)> {
        if let Some(first_digit) = str.chars().nth(0)
            && let Some(second_digit) = str.chars().nth(1)
        {
            Some((first_digit, second_digit))
        } else {
            None
        }
    }

    fn where_joltage_exists(&self, joltage: i32) -> bool {
        if let Some((first_digit, second_digit)) = Bank::find_digits(&joltage.to_string())
            && let Some(fist_digit_positions) = self.index.get(&first_digit)
            && let Some(second_digit_positions) = self.index.get(&second_digit)
            && Bank::joltage_exists(fist_digit_positions, second_digit_positions)
        {
            true
        } else {
            false
        }
    }

    pub fn find_highest_joltage(&self) -> Result<i32, Box<dyn Error>> {
        (10..100)
            .rev()
            .find(|i| self.where_joltage_exists(*i))
            .ok_or(Box::from("No solution found"))
    }

    pub fn find_highest_joltage_v2(&self) -> Result<u64, Box<dyn Error>> {
        // let mut result: Vec<(usize, char)> = Vec::new();
        // for i in (0..10).rev() {
        //     let char = i.to_string().chars().nth(0).unwrap();
        //     for j in self.index.get(&char).unwrap() {
        //         result.push((*j, char));
        //         if result.len() == 12 {
        //             break;
        //         }
        //     }
        // }
        //
        // result.sort_by_key(|x2| x2.0);
        // let string: String = result.into_iter().map(|x1| x1.1).collect();
        // let u: BigInt = string.parse()?;
        // Ok(u)

        let mut stack: MonotonicStack<char> = MonotonicStack::new(12, self.batteries.len());
        for char in &self.batteries {
            stack.push(*char)
        }
        assert_eq!(stack.values().len(), 12);
        let string: String = stack.values().iter().collect();
        let u: u64 = string.parse()?;
        Ok(u)
    }
}

impl FromStr for Bank {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<char>>();
        Ok(Bank::new(&chars))
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::Bank;
    use std::error::Error;
    use std::str::FromStr;

    #[test]
    fn test_811111111111119_has_highest_89() -> Result<(), Box<dyn Error>> {
        let bank = Bank::from_str("811111111111119")?;
        let highest_result = bank.find_highest_joltage()?;
        assert_eq!(89, highest_result);
        Ok(())
    }

    #[test]
    fn part_2_example_1() -> Result<(), Box<dyn Error>> {
        let bank = Bank::from_str("987654321111111")?;
        let highest_result = bank.find_highest_joltage_v2()?;
        let expected: u64 = 987654321111;
        assert_eq!(expected, highest_result);
        Ok(())
    }

    #[test]
    fn part_2_example_2() -> Result<(), Box<dyn Error>> {
        let bank = Bank::from_str("811111111111119")?;
        let highest_result = bank.find_highest_joltage_v2()?;
        let expected: u64 = 811111111119;
        assert_eq!(expected, highest_result);
        Ok(())
    }

    #[test]
    fn part_2_example_3() -> Result<(), Box<dyn Error>> {
        let bank = Bank::from_str("234234234234278")?;
        let highest_result = bank.find_highest_joltage_v2()?;
        let expected: u64 = 434234234278;
        assert_eq!(expected, highest_result);
        Ok(())
    }

    #[test]
    fn part_2_example_4() -> Result<(), Box<dyn Error>> {
        let bank = Bank::from_str("818181911112111")?;
        let highest_result = bank.find_highest_joltage_v2()?;
        let expected: u64 = 888911112111;
        assert_eq!(expected, highest_result);
        Ok(())
    }

    #[test]
    fn day3_part1_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("test.txt");
        let banks = input
            .lines()
            .map(Bank::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        let sum: i32 = banks
            .iter()
            .map(|x1| x1.find_highest_joltage())
            .sum::<Result<_, _>>()?;
        assert_eq!(357, sum);
        Ok(())
    }

    #[test]
    fn day3_part1_solution() -> Result<(), Box<dyn Error>> {
        let input = include_str!("input.txt");
        let banks = input
            .lines()
            .map(Bank::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        let sum: i32 = banks
            .iter()
            .map(|x1| x1.find_highest_joltage())
            .sum::<Result<_, _>>()?;
        assert_eq!(17109, sum);
        Ok(())
    }

    #[test]
    fn day3_part2_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("test.txt");
        let banks = input
            .lines()
            .map(Bank::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        let sum = banks
            .iter()
            .map(|x1| x1.find_highest_joltage_v2())
            .sum::<Result<_, _>>()?;
        let expected: u64 = 3121910778619;
        assert_eq!(expected, sum);
        Ok(())
    }

    #[test]
    fn day3_part2_solution() -> Result<(), Box<dyn Error>> {
        let input = include_str!("input.txt");
        let banks = input
            .lines()
            .map(Bank::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        let sum = banks
            .iter()
            .map(|x1| x1.find_highest_joltage_v2())
            .sum::<Result<_, _>>()?;
        let expected: u64 = 96107789497890;

        assert_eq!(expected, sum);
        Ok(())
    }
}
