use nom::{character::complete::one_of, multi::many1, IResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Blizzard {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    blizzards: Vec<((u32, u32), Blizzard)>,
    max_x: u32,
    max_y: u32,
}

fn map(input: &str) -> IResult<&str, Map> {
    let mut map = Map {
        blizzards: vec![],
        max_x: 0,
        max_y: 0,
    };
    let mut iter = input.lines();
    iter.next();
    let mut iter = iter.enumerate().peekable();
    while let Some((y, line)) = iter.next() {
        if iter.peek().is_some() {}
    }

    let (input, _last_line) = many1(one_of("#."))(input)?;

    Ok((input, map))
}

fn pathfind(map: Map) -> Option<u32> {
    todo!()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (input, map) = map(input).unwrap();
    assert_eq!(input, "");
    let i = pathfind(map).unwrap();
    Some(i)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), None);
    }
}
