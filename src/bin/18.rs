use std::collections::{BTreeSet, HashMap};

use nom::{bytes::complete::tag, character::complete, multi::separated_list1, IResult};

pub fn part_one(input: &str) -> Option<u32> {
    let cubes = input
        .lines()
        .map(|line| cube(line).unwrap().1)
        .collect::<Vec<Cube>>();

    let sides = count_sides(cubes);

    Some(sides as u32)
}

fn count_sides(cubes: Vec<Cube>) -> usize {
    let lines = create_lines(&cubes);
    let mut sides = cubes.len() * 6;
    for (_, set) in lines {
        let mut set = set.iter().peekable();
        while let Some(point) = set.next() {
            match (point, set.peek()) {
                (&p, Some(&&n)) => {
                    if n - p == 1 {
                        sides -= 2;
                    }
                }
                _ => {}
            }
        }
    }
    sides
}

enum Block {
    Empty,
    Bounded,
    Boundary,
}

pub fn part_two(input: &str) -> Option<u32> {
    let cubes = input
        .lines()
        .map(|line| cube(line).unwrap().1)
        .collect::<Vec<Cube>>();
    let cubes = fill_cavities(cubes);
    let sides = count_sides(cubes);
    Some(sides as u32)
}

fn fill_cavities(cubes: Vec<Cube>) -> Vec<Cube> {
    let boundries = find_trivial_boundries(&cubes);
    let boundries = propegate_unbounded(boundries);
    let cubes = fill_bounded_sections(boundries);

    cubes
}

struct Space {
    min_x: u32,
    max_x: u32,
    min_y: u32,
    max_y: u32,
    min_z: u32,
    max_z: u32,
    slots: HashMap<Cube, Block>,
}

impl Space {
    fn new() -> Self {
        Self {
            min_x: u32::MAX,
            max_x: u32::MIN,
            min_y: u32::MAX,
            max_y: u32::MIN,
            min_z: u32::MAX,
            max_z: u32::MIN,
            slots: HashMap::new(),
        }
    }
    fn insert(&mut self, cube: Cube, block: Block) {
        self.min_x = self.min_x.min(cube.x);
        self.max_x = self.max_x.max(cube.x);
        self.min_y = self.min_y.min(cube.y);
        self.max_y = self.max_y.max(cube.y);
        self.min_z = self.min_z.min(cube.y);
        self.max_z = self.max_z.max(cube.y);
        self.slots.entry(cube);
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Line {
    XY { x: u32, y: u32 },
    XZ { x: u32, z: u32 },
    YZ { y: u32, z: u32 },
}

fn find_trivial_boundries(cubes: &[Cube]) -> Space {
    let min_x = 0;
    let max_x = cubes.iter().map(|c| c.x).max().unwrap();
    let min_y = 0;
    let max_y = cubes.iter().map(|c| c.y).max().unwrap();
    let min_z = 0;
    let max_z = cubes.iter().map(|c| c.z).max().unwrap();

    let mut data_structure = Space::new();
    let line_sets = create_lines(&cubes);
    for (line, set) in line_sets {
        for i in *set.first().unwrap()..=*set.last().unwrap() {
            match line {
                Line::XY { x, y } => {}
                Line::XZ { x, z } => todo!(),
                Line::YZ { y, z } => todo!(),
            }
        }
    }
    Space::new()
}

fn propegate_unbounded(space: Space) -> Space {
    Space::new()
}

fn fill_bounded_sections(space: Space) -> Vec<Cube> {
    vec![]
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Cube {
    x: u32,
    y: u32,
    z: u32,
}

fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, pos) = separated_list1(tag(","), complete::u32)(input)?;
    Ok((
        input,
        Cube {
            x: pos[0],
            y: pos[1],
            z: pos[2],
        },
    ))
}

type LineSegment = HashMap<Line, BTreeSet<u32>>;

fn create_lines(cubes: &[Cube]) -> LineSegment {
    let mut map: LineSegment = HashMap::new();
    for cube in cubes {
        map.entry(Line::XY {
            x: cube.x,
            y: cube.y,
        })
        .and_modify(|e| {
            e.insert(cube.z);
        })
        .or_insert(BTreeSet::from([cube.z; 1]));
    }

    for cube in cubes {
        map.entry(Line::YZ {
            y: cube.y,
            z: cube.z,
        })
        .and_modify(|e| {
            e.insert(cube.x);
        })
        .or_insert(BTreeSet::from([cube.x; 1]));
    }

    for cube in cubes {
        map.entry(Line::XZ {
            x: cube.x,
            z: cube.z,
        })
        .and_modify(|e| {
            e.insert(cube.y);
        })
        .or_insert(BTreeSet::from([cube.y; 1]));
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), None);
    }
}
