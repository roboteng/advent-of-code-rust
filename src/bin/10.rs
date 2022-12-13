#![feature(iter_intersperse)]
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn noop(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Instruction::Noop))
}

fn addx(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, v)) = separated_pair(tag("addx"), tag(" "), complete::i32)(input)?;
    Ok((input, Instruction::Addx(v)))
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, alt((noop, addx)))(input)
}

struct Processor {
    n_cycles: u32,
    intructions: Vec<Instruction>,
    state: i32,
}
impl Processor {
    fn tick(&mut self) {
        match self.intructions.get(0) {
            Some(inst) => match inst {
                Instruction::Noop => self.n_cycles += 1,
                Instruction::Addx(v) => {
                    self.n_cycles += 2;
                    self.state += v;
                }
            },
            None => {}
        };
        self.intructions = self.intructions[1..].to_vec();
    }
}

fn state_at(states: &Vec<(u32, i32)>, at: u32) -> i32 {
    states
        .iter()
        .filter(|s| s.0 <= at)
        .last()
        .unwrap_or(&(0, 1))
        .1
}

fn cycle_n_at(n: u32) -> u32 {
    20 + n * 40
}

pub fn part_one(input: &str) -> Option<i32> {
    let (input, instrs) = instructions(input).unwrap();
    let mut p = Processor {
        n_cycles: 0,
        intructions: instrs,
        state: 1,
    };
    let mut states = Vec::new();
    states.push((p.n_cycles, p.state));
    while p.intructions.len() > 0 {
        p.tick();
        states.push((p.n_cycles, p.state));
    }
    let total_cycles = states[states.len() - 1].0;
    let mut i = 0;
    let mut total = 0;
    while cycle_n_at(i) < total_cycles {
        let cycles = cycle_n_at(i);
        total += state_at(&states, cycles) * cycles as i32;
        i += 1;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (input, instrs) = instructions(input).unwrap();
    let mut p = Processor {
        n_cycles: 0,
        intructions: instrs,
        state: 1,
    };
    let mut states = Vec::new();
    states.push((p.n_cycles, p.state));
    while p.intructions.len() > 0 {
        p.tick();
        states.push((p.n_cycles, p.state));
    }
    let mut screen = Vec::new();
    for row in 0..6 {
        let mut pixel_row = Vec::new();
        for column in 0..40 {
            let i = 40 * row + column;
            let state = state_at(&states, i);
            println!("State: {state}, Position: {i}");
            pixel_row.push(match column as i32 - state {
                -1..=1 => '#',
                _ => ' ',
            });
        }
        screen.push(pixel_row);
    }
    let s = screen
        .iter()
        .map(|row| row.iter().collect::<String>())
        .intersperse('\n'.to_string())
        .collect::<String>();
    println!("{}", s);
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13360));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
