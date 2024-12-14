use super::Solve;

pub struct Day4;

impl Solve<i128> for Day4 {
    fn new() -> Self {
        Self
    }

    // stolen from https://discord.com/channels/381880193251409931/1312065700214607882/1313812693357957161
    fn solve_part_1(&mut self, input: &str) -> i128 {
        let parsery = self.parsery(input);
        let parsery = self.parsery(input);
        let womp = ['X', 'M', 'A', 'S'];
        let reversed = womp.iter().copied().rev().collect::<Vec<char>>();
        let mut sum = 0;

        for x in 0..parsery.len() {
            for y in 0..parsery[x].len() {
                if x + 3 < parsery.len() {
                    let slice: Vec<_> = (0..4).map(|i| parsery[x + i][y]).collect();
                    if slice == womp || slice == reversed {
                        sum += 1;
                    }
                }
            }
        }

        sum
    }

    fn solve_part_2(&mut self, input: &str) -> i128 {
        let parsery = self.parsery(input);
        unimplemented!()
    }
}

impl Day4 {
    fn parsery(&self, input: &str) -> Vec<Vec<char>> {
        let mut x_vec = Vec::new();
        let mut y_vec = Vec::new();
        for line in input.lines() {
            for c in line.chars() {
                y_vec.push(c)
            }
            x_vec.push(y_vec.clone())
        }
        x_vec
    }
}
