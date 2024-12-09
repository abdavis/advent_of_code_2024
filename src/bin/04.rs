advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let wordsearch = WordSearch::<140>::from(input);

    Some(wordsearch.count_matches(&['X', 'M', 'A', 'S']))
}

pub fn part_two(input: &str) -> Option<usize> {
    let wordsearch = WordSearch::<140>::from(input);
    Some(wordsearch.count_x())
}

struct WordSearch<const N: usize>([[char; N]; N]);

impl<const N: usize> WordSearch<N> {
    fn count_x(&self) -> usize {
        let mut count = 0;
        for x in 1..(N - 1) {
            for y in 1..(N - 1) {
                if let (('M', 'A', 'S'), ('M', 'A', 'S'))
                | (('S', 'A', 'M'), ('M', 'A', 'S'))
                | (('M', 'A', 'S'), ('S', 'A', 'M'))
                | (('S', 'A', 'M'), ('S', 'A', 'M')) = (
                    (self.0[y - 1][x - 1], self.0[y][x], self.0[y + 1][x + 1]),
                    (self.0[y + 1][x - 1], self.0[y][x], self.0[y - 1][x + 1]),
                ) {
                    count += 1;
                }
            }
        }
        count
    }
    fn count_matches(&self, pat: &[char]) -> usize {
        let mut count = 0;
        for x in 0..N {
            for y in 0..N {
                let mut word_check = |x_dir, y_dir| {
                    let mut x = x;
                    let mut y = y;
                    for c in pat {
                        if x >= N || y >= N || self.0[y][x] != *c {
                            return;
                        }
                        x = (x as isize + x_dir) as usize;
                        y = (y as isize + y_dir) as usize;
                    }
                    count += 1;
                };

                word_check(1, 0);
                word_check(1, 1);
                word_check(0, 1);
                word_check(-1, 1);
                word_check(-1, 0);
                word_check(-1, -1);
                word_check(0, -1);
                word_check(1, -1);
            }
        }
        count
    }
}

impl<const N: usize> From<&str> for WordSearch<N> {
    fn from(value: &str) -> Self {
        use std::array::from_fn;
        let mut lines = value.lines();
        Self(from_fn(|_| {
            let mut chars = lines.next().unwrap().chars();
            from_fn(|_| chars.next().unwrap())
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_val: WordSearch<10> = std::fs::read_to_string("data/examples/04.txt")
            .unwrap()
            .as_str()
            .into();
        assert_eq!(test_val.count_matches(&['X', 'M', 'A', 'S']), 18);
    }

    #[test]
    fn test_part_two() {
        let test_val: WordSearch<10> = std::fs::read_to_string("data/examples/04.txt")
            .unwrap()
            .as_str()
            .into();
        assert_eq!(test_val.count_x(), 9);
    }
}
