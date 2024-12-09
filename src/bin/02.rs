advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let nums: Vec<isize> = l.split_whitespace().map(|n| n.parse().unwrap()).collect();
                let mut pairs = nums.iter().zip(nums.iter().skip(1));
                if let Some((l, r)) = pairs.next() {
                    match r - l {
                        -3..=-1 => match pairs.all(|(l, r)| l - r > 0 && l - r < 4) {
                            true => Some(()),
                            false => None,
                        },
                        1..=3 => match pairs.all(|(l, r)| r - l > 0 && r - l < 4) {
                            true => Some(()),
                            false => None,
                        },
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let nums: Vec<isize> = l.split_whitespace().map(|n| n.parse().unwrap()).collect();
                let safety_check = |nums: &Vec<isize>| {
                    let mut pairs = nums.iter().zip(nums.iter().skip(1));
                    if let Some((l, r)) = pairs.next() {
                        match r - l {
                            -3..=-1 => match pairs.all(|(l, r)| l - r > 0 && l - r < 4) {
                                true => Some(()),
                                false => None,
                            },
                            1..=3 => match pairs.all(|(l, r)| r - l > 0 && r - l < 4) {
                                true => Some(()),
                                false => None,
                            },
                            _ => None,
                        }
                    } else {
                        None
                    }
                };
                if let Some(()) = safety_check(&nums) {
                    Some(())
                } else {
                    for n in 0..nums.len() {
                        let mut nums = nums.clone();
                        nums.remove(n);
                        if let Some(()) = safety_check(&nums) {
                            return Some(());
                        }
                    }
                    None
                }
            })
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
