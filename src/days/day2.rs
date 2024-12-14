use crate::days::Solve;

pub struct Day2;

impl Solve<i128> for Day2 {
    fn solve_part_1(&mut self, input: &str) -> i128 {
        let parsed = self.parsery(input);
        parsed.iter().map(|i| i.is_safe()).filter(|i| *i).count() as i128
    }
    fn solve_part_2(&mut self, input: &str) -> i128 {
        let parsed = self.parsery(input);
        let mut is_safe = 0;
        for i in parsed {
            if i.is_safe() {
                is_safe += 1;
            } else {
                for j in 0..i.ints.len() {
                    let mut lol = i.clone();
                    lol.ints.remove(j);
                    if lol.is_safe() {
                        is_safe += 1;
                        break;
                    }
                }
            }
        }
        is_safe
    }
    fn new() -> Self {
        Self
    }
}

impl Day2 {
    fn parsery(&self, input: &str) -> Vec<Data> {
        input
            .lines()
            .map(|i| {
                i.split_whitespace()
                    .map(|i| i.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .map(|i| Data::new(i))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Data {
    ints: Vec<i32>,
}

impl Data {
    pub fn new(ints: Vec<i32>) -> Self {
        Self { ints }
    }

    pub fn is_safe(&self) -> bool {
        let check_is_down = self.ints[0] > self.ints[1];
        let mut available_choices = vec![self.ints[0]; 3];
        for i in self.ints.windows(2) {
            if check_is_down {
                available_choices[0] = i[0] - 1;
                available_choices[1] = i[0] - 2;
                available_choices[2] = i[0] - 3;
            } else {
                available_choices[0] = i[0] + 1;
                available_choices[1] = i[0] + 2;
                available_choices[2] = i[0] + 3;
            }
            if !available_choices.contains(&i[1]) {
                return false;
            }
        }
        true
    }
}
