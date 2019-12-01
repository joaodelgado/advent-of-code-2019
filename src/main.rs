use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

mod day01;

pub trait Day<T1, T2>
where
    T1: Display,
    T2: Display,
{
    fn run(part: usize, input: &str) {
        let start = Instant::now();
        let result = match part {
            1 => format!("{}", Self::run1(input)),
            2 => format!("{}", Self::run2(input)),
            _ => panic!("Unsupported part {}", part),
        };
        let duration = Instant::now() - start;

        println!("Result: {}", result);
        println!("Time:   {:?}", duration);
    }

    fn run1(input: &str) -> T1;

    fn run2(input: &str) -> T2;
}

#[allow(clippy::zero_prefixed_literal)]
fn main() {
    let day = env::args()
        .nth(1)
        .expect("Expected at least one argument representing the day")
        .parse()
        .expect("This first argument must be a number representing the day");

    let part = env::args()
        .nth(2)
        .expect("Expected a second argument representing the part of the puzzle")
        .parse()
        .expect("This second argument must be a number representing the part of the puzzle");

    let input = env::args()
        .nth(3)
        .unwrap_or_else(|| format!("./data/day{:02}", day));

    let input = match File::open(&input) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Error reading from input file");

            contents
        }
        Err(_) => input,
    };

    match day {
        01 => day01::Day01::run(part, &input),
        _ => panic!("Unsupported day {}", day),
    };
}
