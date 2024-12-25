// unsolved

use std::{ops::{Deref, DerefMut}, path::Iter};

use super::Solve;

pub struct Day5;

impl Solve<i128> for Day5 {
    fn new() -> Self where Self: Sized {
        Day5
    }

    fn solve_part_1(&mut self, input: &str) -> i128 {
        let (mut rules, mut updates) = Self::parsery(input);
        let mut valid_page_prints = Vec::new();
        for i in updates {
            for j in i.iter() {
                for k in rules.iter_mut() {
                    if k.first == *j {
                        k.first_already_updated = true;
                    } else if k.second == *j && k.first_already_updated {
                        valid_page_prints.push(i.clone());
                        break;
                    }
                }
            }
        }
        assert_eq!(valid_page_prints.len(), 3);
        valid_page_prints.iter().map(|i| {
            i[i.len() / 2] as i128
        }).sum()
    }
    fn solve_part_2(&mut self, input: &str) -> i128 {
        todo!()
    }
}

impl Day5 {
    fn parsery(input: &str) -> (Vec<PageRule>, Vec<PageUpdates>) {
        let splitted = input.split("\n\n").collect::<Vec<&str>>();
        let mut rules = Vec::new();
        let mut updates = Vec::new();
        let rule_str = splitted[0];
        let a = rule_str.lines();
        for line in a {
            let splitted = line.split("|").collect::<Vec<&str>>();
            if splitted.len() != 2 {
                continue;
            }
            rules.push(PageRule{
                first: splitted[0].parse::<u128>().unwrap(),
                second: splitted[1].parse::<u128>().unwrap(),
                first_already_updated: false,
            });
        }
        let update_str = splitted[1];
        for line in update_str.lines() {
            let splitted = line.split(",").map(|i| {
                i.parse::<u128>().unwrap()
            }).collect();
            updates.push(PageUpdates(splitted));
        }
        
        (rules, updates)
    }
}

#[derive(Clone, Copy, Debug)]
struct PageRule{
    first: u128,
    second: u128,
    first_already_updated: bool,
}

#[derive(Clone, Debug)]
struct PageUpdates(Vec<u128>);
impl Deref for PageUpdates {
    type Target = Vec<u128>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for PageUpdates {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}