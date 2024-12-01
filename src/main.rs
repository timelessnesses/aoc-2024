mod days;

use crate::days::Solve;

fn main() {
    let day1 = include_str!("../input/day1.txt");
    let mut solver = days::Day1::new();
    let result = solver.solve_part_1(&day1);
    println!("{}", result);
    let result = solver.solve_part_2(&day1);
    println!("{}", result);
}