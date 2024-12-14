mod day1;
mod day2;
mod day3;
mod day4;

pub use day1::Day1;
pub use day2::Day2;
pub use day3::RegexDay3 as Day3; // non regex soon enough :)
pub use day4::Day4;

pub trait Solve<T> {
    fn solve_part_1(&mut self, input: &str) -> T;
    fn solve_part_2(&mut self, input: &str) -> T;
    fn new() -> Self where Self: Sized;
}
