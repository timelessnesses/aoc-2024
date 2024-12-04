pub struct RegexDay3;
use super::Solve;

impl Solve<i32> for RegexDay3 {
    fn solve_part_1(&mut self, input: &str) -> i32 {
        let parsed = self.parsery(input);
        parsed.iter().map(|i| {
            i.0 * i.1
        }).sum()
    }
    fn solve_part_2(&mut self, input: &str) -> i32 {
        let parsed = self.do_dont_parsery(input);
        let mut do_thing = true; // applies for any first mul without do/don't
        parsed.iter().map(move |(do_dont, a, b)| {
            dbg!(&do_dont, &a, &b);
            match do_dont {
                Some(c) => {
                    do_thing = *c;
                    0
                },
                None => {
                    if do_thing {
                        println!("do calc");
                        a * b
                    } else {
                        println!("no calc");
                        0
                    }
                }
            }
        }).sum()
    }
    fn new() -> Self {
        Self
    }
}

impl RegexDay3 {
    fn parsery(&self, input: &str) -> Vec<(i32, i32)> {
        let regexed = regex::Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
        let result = regexed.captures_iter(input).map(|i| {
            let a = i.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let b = i.get(2).unwrap().as_str().parse::<i32>().unwrap();
            (a, b)
        }).collect::<Vec<(i32, i32)>>();
        result
    }

    fn do_dont_parsery(&self, input: &str) -> Vec<(Option<bool>, i32, i32)> {
        // stolen from https://github.com/SkyfallWasTaken/aoc2024/blob/master/src/day3.rs :3
        let regexed = regex::Regex::new(r"(do|don't|mul)?\((?:(\d+),(\d+)?)?\)").unwrap();
        let result = regexed.captures_iter(input).map(|i| {
            let a = i.get(1).map_or("default", |i| i.as_str());
            let b = i.get(2).map_or("0", |i| {
                i.as_str()
            });
            let c = i.get(3).map_or("0", |i| {
                i.as_str()
            });
            match a {
                "do" => (Some(true), b.parse::<i32>().unwrap(), c.parse::<i32>().unwrap()),
                "don't" => (Some(false), b.parse::<i32>().unwrap(), c.parse::<i32>().unwrap()),
                "mul" => (None, b.parse::<i32>().unwrap(), c.parse::<i32>().unwrap()),
                _ => (None, 0,0)
            }
        }).collect();
        result
    }
}

