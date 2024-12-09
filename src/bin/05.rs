advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules: PrintRules = rules.into();
    let updates: PageUpdates = updates.into();
    Some(updates.sum_valid(&rules))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules: PrintRules = rules.into();
    let mut updates: PageUpdates = updates.into();
    Some(updates.sum_corrected(&rules))
}

use std::collections::{HashMap, HashSet};
struct PrintRules(HashMap<u32, HashSet<u32>>);
struct PageUpdates(Vec<Vec<u32>>);

impl PageUpdates {
    fn sum_valid(&self, rules: &PrintRules) -> u32 {
        self.0
            .iter()
            .map(|v| {
                if valid_update(v, rules) {
                    v[v.len() / 2]
                } else {
                    0
                }
            })
            .sum()
    }
    fn sum_corrected(&mut self, rules: &PrintRules) -> u32 {
        let mut temp: Vec<_> = self
            .0
            .iter()
            .filter(|v| !valid_update(v, rules))
            .cloned()
            .collect();

        for mut v in &mut temp {
            sort_update(v, rules);
        }

        temp.iter().map(|v| v[v.len() / 2]).sum()
    }
}

fn sort_update(update: &mut [u32], rules: &PrintRules) {
    if update.len() == 1 {
        return;
    }
    'outer: while let Some(v) = rules.0.get(&update[0]) {
        for n in 1..update.len() {
            if v.contains(&update[n]) {
                let temp = update[n];
                update[n] = update[0];
                update[0] = temp;
                continue 'outer;
            }
        }
        break;
    }
    sort_update(&mut update[1..], rules);
}

fn valid_update(update: &Vec<u32>, rules: &PrintRules) -> bool {
    let mut disallowed_pages = HashSet::new();
    for v in update {
        if let Some(v) = rules.0.get(&v) {
            disallowed_pages = disallowed_pages.union(v).cloned().collect();
        }
        if disallowed_pages.contains(&v) {
            return false;
        }
    }

    true
}

impl From<&str> for PageUpdates {
    fn from(value: &str) -> Self {
        Self(
            value
                .lines()
                .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
                .collect(),
        )
    }
}

impl From<&str> for PrintRules {
    fn from(value: &str) -> Self {
        let mut map = HashMap::<u32, HashSet<u32>>::new();
        for l in value.lines() {
            let (left, right) = l.split_once('|').unwrap();
            let left: u32 = left.parse().unwrap();
            let right: u32 = right.parse().unwrap();
            map.entry(right)
                .and_modify(|v| {
                    v.insert(left);
                })
                .or_insert([left].into());
        }

        Self(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
