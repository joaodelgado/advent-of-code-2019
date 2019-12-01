use super::Day;

pub struct Day01;

impl Day<i32, i32> for Day01 {
    fn run1(input: &str) -> i32 {
        input
            .lines()
            .filter_map(|m| m.parse::<i32>().ok())
            .map(|m| m / 3)
            .map(|m| m - 2)
            .sum()
    }

    fn run2(input: &str) -> i32 {
        input
            .lines()
            .filter_map(|m| m.parse::<i32>().ok())
            .map(|m| calculate_fuel(m))
            .sum()
    }
}

fn calculate_fuel(m: i32) -> i32 {
    let required_fuel = m / 3 - 2;
    if required_fuel <= 0 {
        return 0;
    } else {
        return required_fuel + calculate_fuel(required_fuel);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day01::run1(include_str!("../data/day01")), 3305115);
    }

    #[test]
    fn test_run2() {
        assert_eq!(Day01::run2(include_str!("../data/day01")), 4954799);
    }
}
