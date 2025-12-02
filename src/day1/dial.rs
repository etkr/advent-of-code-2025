use crate::day1::rotation_direction::RotationDirection;

#[derive(Debug, PartialEq)]
pub struct Dial {
    position: i32,
    times_passed_zero: i32,
}

impl Dial {
    pub fn new(position: i32) -> Dial {
        Dial {
            position,
            times_passed_zero: 0,
        }
    }

    pub fn rotate(&self, direction: &RotationDirection) -> Dial {
        let temp = match direction {
            RotationDirection::Left(direction) => self.position - direction,
            RotationDirection::Right(direction) => self.position + direction,
        };

        let position = temp % 100;
        let revolutions = temp / 100;

        let result = match position {
            p if p < 0 => Dial {
                position: position + 100,
                times_passed_zero: if self.position != 0 {
                    revolutions + 1
                } else {
                    revolutions
                },
            },
            p => {
                Dial {
                    position: p,
                    times_passed_zero: revolutions, //|| position == 0,
                }
            }
        };

        if result.position == 0 && (temp == 0 || temp % 100 != 0) {
            Dial {
                position: result.position,
                times_passed_zero: result.times_passed_zero + 1,
            }
        } else {
            result
        }
    }

    pub fn is_zero(&self) -> bool {
        self.position == 0
    }

    pub fn times_passed_zero(&self) -> i32 {
        self.times_passed_zero
    }
}

#[cfg(test)]
mod tests {
    use crate::day1::dial::Dial;
    use crate::day1::rotation_direction::RotationDirection;

    #[test]
    fn rotating_left_1_from_0_results_in_99() {
        let result = Dial::new(0).rotate(&RotationDirection::Left(1));
        let expected = Dial {
            position: 99,
            times_passed_zero: 0,
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn rotating_right_1_from_0_results_in_0() {
        let result = Dial::new(99).rotate(&RotationDirection::Right(1));
        let expected = Dial {
            position: 0,
            times_passed_zero: 1,
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn rotating_left_5_from_0_does_not_pass_zero() {
        let result = Dial::new(0).rotate(&RotationDirection::Left(5));
        assert_eq!(0, result.times_passed_zero());
    }

    #[test]
    fn rotating_right_60_from_95_does_pass_zero() {
        let result = Dial::new(95).rotate(&RotationDirection::Right(60));
        assert_eq!(1, result.times_passed_zero());
    }

    #[test]
    fn rotating_left_55_from_55_does_pass_zero() {
        let result = Dial::new(55).rotate(&RotationDirection::Left(55));
        let expected = Dial {
            position: 0,
            times_passed_zero: 1,
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn rotating_right_1000_from_50_passes_zero_ten_times() {
        let result = Dial::new(50).rotate(&RotationDirection::Right(1000));
        let expected = Dial {
            position: 50,
            times_passed_zero: 10,
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn rotating_left_68_from_50_passes_zero() {
        let result = Dial::new(50).rotate(&RotationDirection::Left(68));
        let expected = Dial {
            position: 82,
            times_passed_zero: 1,
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn rotating_right_48_from_52_passes_zero_once() {
        let result = Dial::new(52).rotate(&RotationDirection::Right(48));
        let expected = Dial {
            position: 0,
            times_passed_zero: 1,
        };
        assert_eq!(expected, result);
    }
}
