use super::Day;

pub struct Day01;

impl Day<i32, isize> for Day01 {
    fn run1(input: &str) -> i32 {
        input
            .lines()
            .filter_map(|m| m.parse::<i32>().ok())
            .map(|m| m / 3)
            .map(|m| m - 2)
            .sum()
    }

    fn run2(input: &str) -> isize {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day01::run1(include_str!("../data/day01")), 3305115);
    }
}
