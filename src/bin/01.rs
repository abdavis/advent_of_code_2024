advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for l in input.lines() {
        let mut l = l.split_whitespace();
        left.push(l.next().unwrap().parse().unwrap());
        right.push(l.next().unwrap().parse().unwrap());
    }
    left.sort();
    right.sort();
    Some(
        left.iter()
            .zip(right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for l in input.lines() {
        let mut l = l.split_whitespace();
        left.push(l.next().unwrap().parse().unwrap());
        right.push(l.next().unwrap().parse().unwrap());
    }
    left.sort();
    right.sort();

    Some(
        left.iter()
            .map(|l| l * right.iter().filter(|r| *r == l).count() as u32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
