use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

struct Grid {
    points: Vec<Vec<char>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn dist_sq(&self, other: &Pos) -> u32 {
        (self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y))
            as u32
    }
}

impl Grid {
    fn at(&self, pos: Pos) -> Option<char> {
        match self.points.get(pos.y) {
            Some(row) => row.get(pos.x).map(|&c| c),
            None => None,
        }
    }

    fn neighbors(&self, pos: Pos) -> Vec<Pos> {
        let mut poses = Vec::new();
        if pos.x >= 1 {
            let end = Pos {
                x: pos.x - 1,
                y: pos.y,
            };
            if self.can_visit(pos, end) {
                poses.push(end);
            }
        }
        if pos.y >= 1 {
            let end = Pos {
                x: pos.x,
                y: pos.y - 1,
            };
            if self.can_visit(pos, end) {
                poses.push(end);
            }
        }
        if pos.x < self.points[0].len() - 1 {
            let end = Pos {
                x: pos.x + 1,
                y: pos.y,
            };
            if self.can_visit(pos, end) {
                poses.push(end);
            }
        }
        if pos.y < self.points.len() - 1 {
            let end = Pos {
                x: pos.x,
                y: pos.y + 1,
            };
            if self.can_visit(pos, end) {
                poses.push(end);
            }
        }

        poses
    }

    fn can_visit(&self, start: Pos, end: Pos) -> bool {
        height(self.at(end).unwrap()) <= height(self.at(start).unwrap()) + 1
    }

    fn start(&self) -> Pos {
        for y in 0..self.points.len() {
            for x in 0..self.points[0].len() {
                if self.points[y][x] == 'S' {
                    return Pos { x, y };
                }
            }
        }
        panic!("Can't find start");
    }

    fn end(&self) -> Pos {
        for y in 0..self.points.len() {
            for x in 0..self.points[0].len() {
                if self.points[y][x] == 'E' {
                    return Pos { x, y };
                }
            }
        }
        panic!("Can't find End");
    }
}

fn height(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8,
        'S' => 'a' as u8,
        'E' => 'z' as u8,
        _ => 0,
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let k = s
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Ok(Grid { points: k })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    pos: Pos,
    dist_to_end: u32,
}

fn a_star(grid: &Grid, start: Pos, goal: Pos) -> u32 {
    println!("start is {:?}", grid.at(start));
    println!("end is {:?}", grid.at(goal));
    let mut known = vec![start];
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut path_length: HashMap<Pos, u32> = HashMap::new();
    let mut path_to: HashMap<Pos, Pos> = HashMap::new();
    path_length.insert(start, 0);
    while known[0] != goal {
        let leader = known[0].clone();
        known = known[1..].to_vec();
        visited.insert(leader);
        let k = path_length.clone();
        let current_path_length = k.get(&leader).unwrap();
        for neighbor in grid.neighbors(leader) {
            if !visited.contains(&neighbor) {
                if known.contains(&neighbor) {
                    let prev_path_length = k.get(&neighbor).unwrap();
                    if *prev_path_length > *current_path_length + 1 {
                        path_length.insert(neighbor, *current_path_length + 1);
                        path_to.insert(neighbor, leader);
                    }
                } else {
                    known.push(neighbor);
                    path_length.insert(neighbor, current_path_length + 1);
                    path_to.insert(neighbor, leader);
                }
            }
        }
        known.sort_by(|a, b| {
            (a.dist_sq(&goal) + path_length.get(&a).unwrap())
                .cmp(&(&b.dist_sq(&goal) + path_length.get(&b).unwrap()))
        });
    }
    let mut end = goal;
    while end != start {
        println!("{end:?}");
        end = *path_to.get(&end).unwrap();
    }

    *path_length.get(&known[0]).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.parse::<Grid>().unwrap();
    let k = a_star(&grid, grid.start(), grid.end());
    println!("{k:?}");
    Some(k)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
