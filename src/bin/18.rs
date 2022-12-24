use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    dbg!(boundries.slots.len());
    let boundries = propegate_unbounded(boundries);
    let cubes = fill_bounded_sections(boundries);

    cubes
}

#[derive(Debug, Clone)]
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
        self.slots.entry(cube).or_insert(block);
    }

    fn get(&self, cube: Cube) -> Block {
        *self.slots.get(&cube).or(Some(&Block::Empty)).unwrap()
    }
}

struct SpaceIterator {
    space: Space,
    i: u32,
}

impl Iterator for SpaceIterator {
    type Item = (Cube, Block);

    fn next(&mut self) -> Option<Self::Item> {
        let dx = self.space.max_x - self.space.min_x;
        let dy = self.space.max_y - self.space.min_y;
        let dz = self.space.max_z - self.space.min_z;
        if self.i >= dx * dy * dz {
            return None;
        }
        let x = self.i % dx + self.space.min_x;
        let y = self.i % (dx * dy) / dx + self.space.min_y;
        let z = self.i / (dx * dy) + self.space.min_z;
        let cube = Cube { x, y, z };
        self.i += 1;
        Some((cube, self.space.get(cube)))
    }
}

impl IntoIterator for Space {
    type Item = (Cube, Block);

    type IntoIter = SpaceIterator;

    fn into_iter(self) -> Self::IntoIter {
        SpaceIterator { i: 0, space: self }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Line {
    XY { x: u32, y: u32 },
    XZ { x: u32, z: u32 },
    YZ { y: u32, z: u32 },
}

fn find_trivial_boundries(cubes: &[Cube]) -> Space {
    let mut data_structure = Space::new();
    let line_sets = create_lines(&cubes);
    for (line, set) in line_sets {
        for i in *set.first().unwrap()..=*set.last().unwrap() {
            match line {
                Line::XY { x, y } => data_structure.insert(Cube { x, y, z: i }, Block::Bounded),
                Line::XZ { x, z } => data_structure.insert(Cube { x, y: i, z }, Block::Bounded),
                Line::YZ { y, z } => data_structure.insert(Cube { x: i, y, z }, Block::Bounded),
            }
        }
    }
    for cube in cubes {
        data_structure.insert(*cube, Block::Boundary)
    }
    data_structure
}

fn propegate_unbounded(mut space: Space) -> Space {
    let mut changed = true;
    for _ in 0..4 {
        println!("Looping");
        let mut processed_space = space.clone();
        changed = false;
        for voxel in space
            .clone()
            .into_iter()
            .filter(|(_, b)| *b == Block::Bounded)
        {
            if neighbors(voxel.0)
                .iter()
                .any(|c| space.get(*c) == Block::Empty)
            {
                processed_space.insert(voxel.0, Block::Empty);
                println!("Found false cavity at {:?}", voxel.0);
                changed = true;
            }
        }
        space = processed_space;
    }
    space
}

fn fill_bounded_sections(space: Space) -> Vec<Cube> {
    space
        .into_iter()
        .filter_map(|(c, b)| if b == Block::Empty { None } else { Some(c) })
        .collect_vec()
}

fn neighbors(cube: Cube) -> Vec<Cube> {
    vec![
        Cube {
            x: cube.x + 1,
            y: cube.y,
            z: cube.z,
        },
        Cube {
            x: cube.x - 1,
            y: cube.y,
            z: cube.z,
        },
        Cube {
            x: cube.x,
            y: cube.y + 1,
            z: cube.z,
        },
        Cube {
            x: cube.x,
            y: cube.y - 1,
            z: cube.z,
        },
        Cube {
            x: cube.x,
            y: cube.y,
            z: cube.z + 1,
        },
        Cube {
            x: cube.x,
            y: cube.y,
            z: cube.z - 1,
        },
    ]
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
        assert_eq!(part_two(&input), Some(58));
    }
}
