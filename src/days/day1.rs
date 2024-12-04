use super::Solve;

pub struct Day1;
impl Solve<u128> for Day1 {
    fn solve_part_1(&mut self, input: &str) -> u128 {
        let mut left = vec![];
        let mut right = vec![];
        Self::parsery(input, &mut left, &mut right);
        left.iter()
            .zip(right.iter())
            .map(|(a, b)| minus_max(*a, *b))
            .sum()
    }

    fn new() -> Self {
        Self
    }

    fn solve_part_2(&mut self, input: &str) -> u128 {
        let mut left = vec![];
        let mut right = vec![];
        Self::parsery(input, &mut left, &mut right);
        left.iter()
            .map(|i| i * right.iter().filter(|j| *i == **j).count() as u128)
            .sum()
    }
}

impl Day1 {
    fn parsery(input: &str, left: &mut Vec<u128>, right: &mut Vec<u128>) {
        input
            .lines()
            .map(|i| i.split_whitespace())
            .for_each(|mut i| {
                left.sort();
                right.sort();
                left.push(i.next().unwrap().parse::<u128>().unwrap());
                right.push(i.next().unwrap().parse::<u128>().unwrap());
            });
    }
}

fn minus_max(a: u128, b: u128) -> u128 {
    if a > b {
        a - b
    } else {
        b - a
    }
}
