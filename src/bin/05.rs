use std::str::FromStr;

type Stack = Vec<char>;

struct Move {
    origin: usize,
    end: usize,
    count: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_ascii_whitespace();
        words.next();
        let count = words.next().unwrap().parse::<usize>().unwrap();
        words.next();
        let origin = words.next().unwrap().parse::<usize>().unwrap();
        words.next();
        let end = words.next().unwrap().parse::<usize>().unwrap();
        Ok(Self { count, origin, end })
    }
}

fn move_crates(mut stacks: Vec<Stack>, m: Move) -> Vec<Stack> {
    for _ in 0..m.count {
        let crate_ = stacks[m.origin - 1].pop().unwrap();
        stacks[m.end - 1].push(crate_);
    }
    stacks
}

fn move_crates_at_once(mut stacks: Vec<Stack>, m: Move) -> Vec<Stack> {
    let mut temp_stack = Vec::new();
    for _ in 0..m.count {
        let crate_ = stacks[m.origin - 1].pop().unwrap();
        temp_stack.push(crate_);
    }
    for _ in 0..m.count {
        let crate_ = temp_stack.pop().unwrap();
        stacks[m.end - 1].push(crate_);
    }
    stacks
}

fn parse_input(input: &str) -> (Vec<Stack>, Vec<Move>) {
    let mut halves = input.split("\n\n");
    let stacks = halves.next().unwrap();
    let moves = halves.next().unwrap();
    (parse_stacks(stacks), parse_moves(moves))
}

fn parse_stacks(input: &str) -> Vec<Stack> {
    let lines: Vec<&str> = input.lines().collect();
    let stacks: Vec<&str> = lines[lines.len() - 1]
        .trim()
        .split(' ')
        .filter(|c| *c != "")
        .collect();
    let chars: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut built_stacks = Vec::new();
    for i in 0..(stacks.len()) {
        let index = i * 4 + 1;
        let mut stack: Stack = Vec::new();
        for j in 0..(lines.len() - 1) {
            if let Some(&c) = chars[lines.len() - 2 - j].get(index) {
                if c != ' ' {
                    stack.push(c);
                }
            }
        }
        built_stacks.push(stack);
    }
    built_stacks
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| line.parse::<Move>().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);
    for m in moves {
        stacks = move_crates(stacks, m);
    }

    let k: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    Some(k)
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);
    for m in moves {
        stacks = move_crates_at_once(stacks, m);
    }

    let k: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    Some(k)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
