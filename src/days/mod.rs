mod day1;
pub use day1::Day1;

pub trait Solve<T> {
    fn solve_part_1(&mut self, input: &str) -> T;
    fn solve_part_2(&mut self, input: &str) -> T;
    fn new() -> Self;
}