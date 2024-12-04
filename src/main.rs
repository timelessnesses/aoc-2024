mod days;

use crate::days::Solve;

fn main() {
    better_panic::debug_install();
    let day3 = include_str!("../input/day3.txt");
    let mut solver = days::Day3::new();
    let result = solver.solve_part_1(&day3);
    println!("{}", result);
    let result = solver.solve_part_2(&day3);
    println!("{}", result);
}
