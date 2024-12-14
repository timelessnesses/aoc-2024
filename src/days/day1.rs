use super::Solve;

pub struct Day1;
impl Solve<i128> for Day1 {
    fn solve_part_1(&mut self, input: &str) -> i128 {
        let mut left = vec![];
        let mut right = vec![];
        Self::parsery(input, &mut left, &mut right);
        left.iter()
            .zip(right.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum::<u128>() as i128
    }

    fn new() -> Self {
        Self
    }

    fn solve_part_2(&mut self, input: &str) -> i128 {
        let mut left = vec![];
        let mut right = vec![];
        Self::parsery(input, &mut left, &mut right);
        left.iter()
            .map(|i| i * right.iter().filter(|j| *i == **j).count() as i128)
            .sum()
    }
}

impl Day1 {
    fn parsery(input: &str, left: &mut Vec<i128>, right: &mut Vec<i128>) {
        input
            .lines()
            .map(|i| i.split_whitespace())
            .for_each(|mut i| {
                left.push(i.next().unwrap().parse::<i128>().unwrap());
                right.push(i.next().unwrap().parse::<i128>().unwrap());
            });
        left.sort();
        right.sort();
    }
}