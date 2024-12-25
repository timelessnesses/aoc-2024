use clap::parser;

use super::Solve;

pub struct Day6;

impl Solve<i128> for Day6 {
    fn new() -> Self {
        Self
    }

    fn solve_part_1(&mut self, input: &str) -> i128 {
        let mut parsery = self.parsery(input);
        // if object in front of guard, turn 90 degrees
        // nothing? go forward
        let mut guard_pos = (0, 0);
        for x in 0..parsery.len() {
            for y in 0..parsery[x].len() {
                if parsery[x][y] == Block::Guard(Coordinate::North) {
                    guard_pos = (x, y);
                }
            }
        }
        let mut count_steps = 0;
        loop {
            let next_position = match parsery[guard_pos.0][guard_pos.1] {
                Block::Guard(Coordinate::North) => {
                    (guard_pos.0 - 1, guard_pos.1)
                },
                Block::Guard(Coordinate::South) => {
                    (guard_pos.0 + 1, guard_pos.1)
                },
                Block::Guard(Coordinate::East) => {
                    (guard_pos.0, guard_pos.1 - 1)
                },
                Block::Guard(Coordinate::West) => {
                    (guard_pos.0, guard_pos.1 + 1)
                },
                _ => unreachable!()
            };
            if let None = parsery.get(next_position.0) {
                // break free
                break;
            } else if let None = parsery[next_position.0].get(next_position.1) {
                // break free
                break;
            }
            if parsery[next_position.0][next_position.1] == Block::Empty {
                parsery[next_position.0][next_position.1] = parsery[guard_pos.0][guard_pos.1];
                parsery[guard_pos.0][guard_pos.1] = Block::Empty;
                guard_pos = next_position;
            } else if parsery[next_position.0][next_position.1] == Block::Object {
                parsery[guard_pos.0][guard_pos.1] = Block::Guard(match parsery[guard_pos.0][guard_pos.1] {
                    Block::Guard(Coordinate::North) => Coordinate::East,
                    Block::Guard(Coordinate::South) => Coordinate::West,
                    Block::Guard(Coordinate::East) => Coordinate::North,
                    Block::Guard(Coordinate::West) => Coordinate::South,
                    _ => unreachable!()
                });
            }
            count_steps += 1;
        }
        count_steps
    }
    fn solve_part_2(&mut self, input: &str) -> i128 {
        todo!()
    }
}

impl Day6 {
    fn parsery(&self, input: &str) -> Vec<Vec<Block>> {
        let mut x_vec = Vec::new();
        let mut y_vec = Vec::new();
        for line in input.lines() {
            for c in line.chars() {
                if c == '#' {
                    y_vec.push(Block::Object)
                } else if c == '.' {
                    y_vec.push(Block::Empty)
                } else if c == '^' {
                    y_vec.push(Block::Guard(Coordinate::North))
                }
            }
            x_vec.push(y_vec.clone())
        }
        x_vec
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Block {
    Empty,
    Object,
    Guard(Coordinate)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Coordinate {
    North, South, East, West
}