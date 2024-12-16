advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<usize> {
    let maze: Maze<141> = input.into();
    Some(maze.shortest_path())
}

pub fn part_two(input: &str) -> Option<usize> {
    let maze: Maze<141> = input.into();
    Some(maze.count_paths_shortest())
}

struct Maze<const N: usize> {
    maze: [[Tile; N]; N],
    start: Pos,
    end: Pos,
}

impl<const N: usize> Maze<N> {
    fn count_paths_shortest(&self) -> usize {
        use std::collections::{BinaryHeap, HashMap, HashSet};

        #[derive(Eq, PartialEq, Hash, Clone, Copy)]
        enum Heading {
            N,
            E,
            S,
            W,
        }
        #[derive(Eq, PartialEq, Hash)]
        struct VisitedNode {
            pos: Pos,
            heading: Heading,
        }
        struct Node {
            pos: Pos,
            heading: Heading,
            cost: usize,
            parents: Vec<Pos>,
        }
        impl Ord for Node {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                other.cost.cmp(&self.cost)
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
                self.cost == other.cost
            }
        }
        let mut visited = HashMap::new();
        let mut queue = BinaryHeap::new();
        let mut best_path_tiles = HashSet::new();
        queue.push(Node {
            pos: self.start,
            heading: Heading::E,
            cost: 0,
            parents: vec![],
        });
        let mut cost = usize::MAX;
        while let Some(n) = queue.pop() {
            if *visited
                .entry(VisitedNode {
                    pos: n.pos,
                    heading: n.heading,
                })
                .or_insert(n.cost)
                == n.cost
            {
                if n.pos == self.end && n.cost <= cost {
                    cost = n.cost;
                    for v in &n.parents {
                        best_path_tiles.insert(*v);
                    }
                    continue;
                }
                match n.heading {
                    Heading::N => {
                        if n.pos.y > 0 && matches!(self.maze[n.pos.y - 1][n.pos.x], Tile::Path) {
                            let mut parents = n.parents.clone();
                            parents.push(n.pos);
                            queue.push(Node {
                                pos: Pos {
                                    y: n.pos.y - 1,
                                    ..n.pos
                                },
                                cost: n.cost + 1,
                                parents,
                                ..n
                            })
                        }
                        queue.push(Node {
                            heading: Heading::E,
                            cost: n.cost + 1000,
                            parents: n.parents.clone(),
                            ..n
                        });
                        queue.push(Node {
                            heading: Heading::W,
                            cost: n.cost + 1000,
                            ..n
                        });
                    }
                    Heading::E => {
                        if n.pos.x < N - 1 && matches!(self.maze[n.pos.y][n.pos.x + 1], Tile::Path)
                        {
                            let mut parents = n.parents.clone();
                            parents.push(n.pos);
                            queue.push(Node {
                                pos: Pos {
                                    x: n.pos.x + 1,
                                    ..n.pos
                                },
                                cost: n.cost + 1,
                                parents,
                                ..n
                            })
                        }
                        queue.push(Node {
                            heading: Heading::N,
                            cost: n.cost + 1000,
                            parents: n.parents.clone(),
                            ..n
                        });
                        queue.push(Node {
                            heading: Heading::S,
                            cost: n.cost + 1000,
                            ..n
                        });
                    }
                    Heading::S => {
                        if n.pos.y < N - 1 && matches!(self.maze[n.pos.y + 1][n.pos.x], Tile::Path)
                        {
                            let mut parents = n.parents.clone();
                            parents.push(n.pos);
                            queue.push(Node {
                                pos: Pos {
                                    y: n.pos.y + 1,
                                    ..n.pos
                                },
                                cost: n.cost + 1,
                                parents,
                                ..n
                            })
                        }
                        queue.push(Node {
                            heading: Heading::E,
                            cost: n.cost + 1000,
                            parents: n.parents.clone(),
                            ..n
                        });
                        queue.push(Node {
                            heading: Heading::W,
                            cost: n.cost + 1000,
                            ..n
                        });
                    }
                    Heading::W => {
                        if n.pos.x > 0 && matches!(self.maze[n.pos.y][n.pos.x - 1], Tile::Path) {
                            let mut parents = n.parents.clone();
                            parents.push(n.pos);
                            queue.push(Node {
                                pos: Pos {
                                    x: n.pos.x - 1,
                                    ..n.pos
                                },
                                cost: n.cost + 1,
                                parents,
                                ..n
                            })
                        }
                        queue.push(Node {
                            heading: Heading::N,
                            cost: n.cost + 1000,
                            parents: n.parents.clone(),
                            ..n
                        });
                        queue.push(Node {
                            heading: Heading::S,
                            cost: n.cost + 1000,
                            ..n
                        });
                    }
                }
            }
        }

        // use std::io::stdout;
        // use std::io::Write;
        // let mut stdout = stdout().lock();
        // for y in 0..N {
        //     for x in 0..N {
        //         write!(
        //             stdout,
        //             "{}",
        //             if best_path_tiles.contains(&Pos { x, y }) {
        //                 'O'
        //             } else {
        //                 '.'
        //             }
        //         );
        //     }
        //     write!(stdout, "\n");
        // }

        best_path_tiles.len() + 1
    }
    fn shortest_path(&self) -> usize {
        use std::collections::BinaryHeap;
        use std::collections::HashSet;

        #[derive(Eq, PartialEq, Hash, Clone, Copy)]
        enum Heading {
            N,
            E,
            S,
            W,
        }
        #[derive(Eq, PartialEq, Hash)]
        struct VisitedNode {
            pos: Pos,
            heading: Heading,
        }
        struct Node {
            pos: Pos,
            heading: Heading,
            cost: usize,
        }
        impl Ord for Node {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                other.cost.cmp(&self.cost)
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
                self.cost == other.cost
            }
        }
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::new();
        queue.push(Node {
            pos: self.start,
            heading: Heading::E,
            cost: 0,
        });
        while let Some(n) = queue.pop() {
            if n.pos == self.end {
                return n.cost;
            }
            if visited.insert(VisitedNode {
                pos: n.pos,
                heading: n.heading,
            }) {
                match n.heading {
                    Heading::N => {
                        if n.pos.y > 0 && matches!(self.maze[n.pos.y - 1][n.pos.x], Tile::Path) {
                            queue.push(Node {
                                pos: Pos {
                                    y: n.pos.y - 1,
                                    ..n.pos
                                },
                                cost: n.cost + 1,
                                ..n
                            })
                        }
                        queue.push(Node {
                            heading: Heading::E,
                            cost: n.cost + 1000,
                            ..n
                        });
                        queue.push(Node {
                            heading: Heading::W,
                            cost: n.cost + 1000,
                            ..n
                        });
                    }
                    Heading::E => {
                        if n.pos.x < N - 1 && matches!(self.maze[n.pos.y][n.pos.x + 1], Tile::Path)
                        {
                            queue.push(Node {
                                pos: Pos {
                                    x: n.pos.x + 1,
                                    ..n.pos
                                },
                                cost: n.cost + 1,
                                ..n
                            })
                        }
                        queue.push(Node {
                            heading: Heading::N,
                            cost: n.cost + 1000,
                            ..n
                        });
                        queue.push(Node {
                            heading: Heading::S,
                            cost: n.cost + 1000,
                            ..n
                        });
                    }
                    Heading::S => {
                        if n.pos.y < N - 1 && matches!(self.maze[n.pos.y + 1][n.pos.x], Tile::Path)
                        {
                            queue.push(Node {
                                pos: Pos {
                                    y: n.pos.y + 1,
                                    ..n.pos
                                },
                                cost: n.cost + 1,
                                ..n
                            })
                        }
                        queue.push(Node {
                            heading: Heading::E,
                            cost: n.cost + 1000,
                            ..n
                        });
                        queue.push(Node {
                            heading: Heading::W,
                            cost: n.cost + 1000,
                            ..n
                        });
                    }
                    Heading::W => {
                        if n.pos.x > 0 && matches!(self.maze[n.pos.y][n.pos.x - 1], Tile::Path) {
                            queue.push(Node {
                                pos: Pos {
                                    x: n.pos.x - 1,
                                    ..n.pos
                                },
                                cost: n.cost + 1,
                                ..n
                            })
                        }
                        queue.push(Node {
                            heading: Heading::N,
                            cost: n.cost + 1000,
                            ..n
                        });
                        queue.push(Node {
                            heading: Heading::S,
                            cost: n.cost + 1000,
                            ..n
                        });
                    }
                }
            }
        }

        panic!("no path found")
    }
}

impl<const N: usize> From<&str> for Maze<N> {
    fn from(value: &str) -> Self {
        use std::array::from_fn;
        let mut start = Pos { x: 0, y: 0 };
        let mut end = Pos { x: 0, y: 0 };
        let mut lines = value.lines().enumerate();
        let maze = from_fn(|_| {
            let (y, l) = lines.next().unwrap();
            let mut chars = l.chars().enumerate();
            from_fn(|_| {
                let (x, c) = chars.next().unwrap();
                match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Path,
                    'S' => {
                        start = Pos { x, y };
                        Tile::Path
                    }
                    'E' => {
                        end = Pos { x, y };
                        Tile::Path
                    }
                    c => panic!("{c} is an invalid character"),
                }
            })
        });

        Self { start, end, maze }
    }
}

enum Tile {
    Path,
    Wall,
}
#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let maze: Maze<17> = std::fs::read_to_string("data/examples/16.txt")
            .unwrap()
            .as_str()
            .into();
        assert_eq!(Some(maze.shortest_path()), Some(11048))
    }

    #[test]
    fn test_part_two() {
        let maze: Maze<17> = std::fs::read_to_string("data/examples/16.txt")
            .unwrap()
            .as_str()
            .into();
        assert_eq!(Some(maze.count_paths_shortest()), Some(64))
    }
}
