#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn fight(self, other: RPS) -> Outcome {
        match (self, other) {
            (RPS::Rock, RPS::Paper) => Outcome::Lose,
            (RPS::Rock, RPS::Scissors) => Outcome::Win,
            (RPS::Paper, RPS::Rock) => Outcome::Win,
            (RPS::Paper, RPS::Scissors) => Outcome::Lose,
            (RPS::Scissors, RPS::Rock) => Outcome::Lose,
            (RPS::Scissors, RPS::Paper) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }

    fn score(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

#[derive(Debug)]
struct RPSMatch {
    p1: RPS,
    p2: RPS,
}

impl RPSMatch {
    fn score(&self) -> (u32, u32) {
        let outcome = self.p1.fight(self.p2);
        let score = match outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        };
        let p1_score = self.p1.score() + score;
        let p2_score = self.p2.score() + 6 - score;
        (p1_score, p2_score)
    }
}

impl From<&str> for RPSMatch {
    fn from(line: &str) -> Self {
        let mut chars = line.split(' ');
        let p1 = match chars.next().unwrap() {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            x => panic!("Unknown string: {}", x),
        };
        let outcome = match chars.next().unwrap() {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            x => panic!("Unknown string: {}", x),
        };
        let p2 = {
            if RPS::Rock.fight(p1) == outcome {
                RPS::Rock
            } else if RPS::Paper.fight(p1) == outcome {
                RPS::Paper
            } else {
                RPS::Scissors
            }
        };
        Self { p1, p2 }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let m = RPSMatch::from("A Z");
    println!("{:?}", m);
    let score = input
        .lines()
        .map(|line| line.into())
        .map(|m: RPSMatch| m.score().1)
        .reduce(|a, b| a + b)
        .unwrap();
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|line| line.into())
        .map(|m: RPSMatch| m.score().1)
        .reduce(|a, b| a + b)
        .unwrap();
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
