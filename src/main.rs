mod days;

use std::str::FromStr;
use clap::Parser;
use crate::days::Solve;

#[derive(clap::Parser)]
struct Cli {
    day: Days,
}

fn main() {
    let cli = Cli::parse();
    let mut solver = cli.day.get_solver();
    better_panic::debug_install();
    let input = cli.day.get_input();
    let result = solver.solve_part_1(&input);
    println!("{result}");
    let result = solver.solve_part_2(&input);
    println!("{result}");
}

#[derive(clap::Parser, Clone, Copy)]
enum Days {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6
}

impl FromStr for Days {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::Day1),
            "2" => Ok(Self::Day2),
            "3" => Ok(Self::Day3),
            "4" => Ok(Self::Day4),
            "5" => Ok(Self::Day5),
            "6" => Ok(Self::Day6),
            _ => Err(format!("Invalid day: {}", s)),
        }
    }
}

impl Days {
    fn get_solver(&self) -> Box<dyn days::Solve<i128>> {
        match self {
            Self::Day1 => Box::new(days::Day1),
            Self::Day2 => Box::new(days::Day2),
            Self::Day3 => Box::new(days::Day3),
            Self::Day4 => Box::new(days::Day4),
            Self::Day5 => Box::new(days::Day5),
            Self::Day6 => Box::new(days::Day6),
        }
    }
    fn get_input(&self) -> String {
        match self {
            Self::Day1 => include_str!("../input/day1.txt").to_string(),
            Self::Day2 => include_str!("../input/day2.txt").to_string(),
            Self::Day3 => include_str!("../input/day3.txt").to_string(),
            Self::Day4 => include_str!("../input/day4.txt").to_string(),
            Self::Day5 => include_str!("../input/test_day5.txt").to_string(),
            Self::Day6 => include_str!("../input/test_day6.txt").to_string(),
        }
    }
}
