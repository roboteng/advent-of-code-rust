use std::collections::HashSet;

use nom::sequence::separated_pair;
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{newline, u32},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn delta(&self) -> Pos {
        match self {
            Dir::Up => Pos(0, 1),
            Dir::Down => Pos(0, -1),
            Dir::Left => Pos(-1, 0),
            Dir::Right => Pos(1, 0),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Move {
    dir: Dir,
    dist: u32,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Pos(i32, i32);
impl Pos {
    fn add(&self, other: &Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }
    fn sub(&self, other: &Pos) -> Pos {
        Pos(self.0 - other.0, self.1 - other.1)
    }
}

fn _move(input: &str) -> IResult<&str, Move> {
    let (input, (dir, dist)) = separated_pair(is_a("UDLR"), tag(" "), u32)(input)?;

    Ok((
        input,
        match dir {
            "U" => Move { dir: Dir::Up, dist },
            "D" => Move {
                dir: Dir::Down,
                dist,
            },
            "L" => Move {
                dir: Dir::Left,
                dist,
            },
            "R" => Move {
                dir: Dir::Right,
                dist,
            },
            _ => panic!("unable to parse, {dir}"),
        },
    ))
}

fn moves(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(newline, _move)(input)
}

struct HeadPath {
    moves: Vec<Move>,
    pos: Pos,
}

impl HeadPath {
    fn new(moves: Vec<Move>) -> Self {
        Self {
            moves,
            pos: Pos(0, 0),
        }
    }
}

impl Iterator for HeadPath {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        match self.moves.get_mut(0) {
            Some(mut m) => {
                self.pos = self.pos.add(&m.dir.delta());
                m.dist -= 1;
                if m.dist == 0 {
                    self.moves = self.moves[1..].to_vec();
                }
                Some(self.pos)
            }
            None => None,
        }
    }
}

fn new_tail_pos(head: Pos, old_tail: Pos) -> Pos {
    let delta = head.sub(&old_tail);
    match delta {
        Pos(-2, -2) => old_tail.add(&Pos(-1, -1)),
        Pos(-2, -1) => old_tail.add(&Pos(-1, -1)),
        Pos(-2, 0) => old_tail.add(&Pos(-1, 0)),
        Pos(-2, 1) => old_tail.add(&Pos(-1, 1)),
        Pos(-2, 2) => old_tail.add(&Pos(-1, 1)),

        Pos(-1, -2) => old_tail.add(&Pos(-1, -1)),
        Pos(-1, -1) => old_tail,
        Pos(-1, 0) => old_tail,
        Pos(-1, 1) => old_tail,
        Pos(-1, 2) => old_tail.add(&Pos(-1, 1)),

        Pos(0, -2) => old_tail.add(&Pos(0, -1)),
        Pos(0, -1) => old_tail,
        Pos(0, 0) => old_tail,
        Pos(0, 1) => old_tail,
        Pos(0, 2) => old_tail.add(&Pos(0, 1)),

        Pos(1, -2) => old_tail.add(&Pos(1, -1)),
        Pos(1, -1) => old_tail,
        Pos(1, 0) => old_tail,
        Pos(1, 1) => old_tail,
        Pos(1, 2) => old_tail.add(&Pos(1, 1)),

        Pos(2, -2) => old_tail.add(&Pos(1, -1)),
        Pos(2, -1) => old_tail.add(&Pos(1, -1)),
        Pos(2, 0) => old_tail.add(&Pos(1, 0)),
        Pos(2, 1) => old_tail.add(&Pos(1, 1)),
        Pos(2, 2) => old_tail.add(&Pos(1, 1)),
        _ => panic!("Unknown delta: {:?}", delta),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, moves) = moves(input).unwrap();
    let iter = HeadPath::new(moves);
    let mut tail_pos = Pos(0, 0);
    let mut positions = HashSet::new();
    for head in iter {
        tail_pos = new_tail_pos(head, tail_pos);
        positions.insert(tail_pos);
    }
    Some(positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, moves) = moves(input).unwrap();
    let iter = HeadPath::new(moves);
    let mut rope = [Pos(0, 0); 9];
    let mut positions = HashSet::new();
    for head in iter {
        rope[0] = new_tail_pos(head, rope[0]);
        for i in 0..8 {
            rope[i + 1] = new_tail_pos(rope[i], rope[i + 1]);
        }
        positions.insert(rope[8]);
    }
    Some(positions.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
