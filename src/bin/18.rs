advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<usize> {
    let map: ByteMap<71> = input.into();

    map.path(1024)
}

pub fn part_two(input: &str) -> Option<String> {
    let map: ByteMap<71> = input.into();

    Some(map.last_byte(1024))
}

struct ByteMap<const N: usize> {
    map: [[u16; N]; N],
}

impl<const N: usize> ByteMap<N> {
    fn last_byte(&self, start: u16) -> String {
        let mut last = 0;
        for n in start..(N * N) as u16 {
            if let None = self.path(n) {
                last = n - 1;
                break;
            }
        }

        let (x, y, _) = self
            .map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, n)| (x, y, n)))
            .find(|(_, _, n)| **n == last)
            .unwrap();
        format!("{x},{y}")
    }
    fn path(&self, fallen_bytes: u16) -> Option<usize> {
        use std::collections::{BinaryHeap, HashSet};
        let mut queue = BinaryHeap::new();
        let mut visited = HashSet::<Pos>::new();

        struct Node {
            pos: Pos,
            cost: usize,
            heuristic: usize,
        }
        impl Ord for Node {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                (other.cost + other.heuristic).cmp(&(self.cost + &self.heuristic))
            }
        }
        impl PartialOrd for Node {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Eq for Node {}
        impl PartialEq for Node {
            fn eq(&self, other: &Self) -> bool {
                (self.cost + self.heuristic) == (other.cost + other.heuristic)
            }
        }
        #[derive(Hash, PartialEq, Eq, Copy, Clone)]
        struct Pos {
            x: usize,
            y: usize,
        }

        queue.push(Node {
            pos: Pos { x: 0, y: 0 },
            cost: 0,
            heuristic: (N - 1) * 2,
        });
        while let Some(n) = queue.pop() {
            if n.pos.x == N - 1 && n.pos.y == N - 1 {
                return Some(n.cost);
            }
            if !visited.contains(&n.pos) {
                visited.insert(n.pos);
                if n.pos.x > 0 && self.map[n.pos.y][n.pos.x - 1] >= fallen_bytes {
                    queue.push(Node {
                        pos: Pos {
                            x: n.pos.x - 1,
                            ..n.pos
                        },
                        cost: n.cost + 1,
                        heuristic: (N - 1 - (n.pos.x - 1)) + (N - 1 - n.pos.y),
                    });
                }
                if n.pos.x < N - 1 && self.map[n.pos.y][n.pos.x + 1] >= fallen_bytes {
                    queue.push(Node {
                        pos: Pos {
                            x: n.pos.x + 1,
                            ..n.pos
                        },
                        cost: n.cost + 1,
                        heuristic: (N - 1 - (n.pos.x + 1)) + (N - 1 - n.pos.y),
                    })
                }
                if n.pos.y > 0 && self.map[n.pos.y - 1][n.pos.x] >= fallen_bytes {
                    queue.push(Node {
                        pos: Pos {
                            y: n.pos.y - 1,
                            ..n.pos
                        },
                        cost: n.cost + 1,
                        heuristic: (N - 1 - (n.pos.x)) + (N - 1 - (n.pos.y - 1)),
                    });
                }
                if n.pos.y < N - 1 && self.map[n.pos.y + 1][n.pos.x] >= fallen_bytes {
                    queue.push(Node {
                        pos: Pos {
                            y: n.pos.y + 1,
                            ..n.pos
                        },
                        cost: n.cost + 1,
                        heuristic: (N - 1 - (n.pos.x)) + (N - 1 - (n.pos.y + 1)),
                    })
                }
            }
        }

        None
    }
}

impl<const N: usize> From<&str> for ByteMap<N> {
    fn from(value: &str) -> Self {
        let mut map = ByteMap {
            map: [[u16::MAX; N]; N],
        };

        for (n, l) in value.lines().enumerate() {
            let (l, r) = l.split_once(',').unwrap();
            let l: usize = l.parse().unwrap();
            let r: usize = r.parse().unwrap();
            map.map[r][l] = n as u16;
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let map: ByteMap<7> = std::fs::read_to_string("data/examples/18.txt")
            .unwrap()
            .as_str()
            .into();
        assert_eq!(map.path(12), Some(22));
    }

    #[test]
    fn test_part_two() {
        let map: ByteMap<7> = std::fs::read_to_string("data/examples/18.txt")
            .unwrap()
            .as_str()
            .into();
        assert_eq!(Some(map.last_byte(12)), Some("6,1".into()));
    }
}
