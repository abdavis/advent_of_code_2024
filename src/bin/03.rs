advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<usize> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    Some(
        re.captures_iter(input)
            .map(|c| c[1].parse::<usize>().unwrap() * c[2].parse::<usize>().unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    Some(
        re.captures_iter(input)
            .scan(true, |st, c| match (&st, &c[0]) {
                (true, "don't()") => {
                    println!("turned off");
                    *st = false;
                    Some(0)
                }
                (false, "do()") => {
                    println!("turned on");
                    *st = true;
                    Some(0)
                }
                (true, m) if m.starts_with("mul(") => {
                    Some(c[1].parse::<usize>().unwrap() * c[2].parse::<usize>().unwrap())
                }
                _ => Some(0),
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(48));
    // }
}
