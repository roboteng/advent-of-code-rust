use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u32,
    end: u32,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut matches = s.split('-');
        let start = matches.next().map(|a| a.parse::<u32>());
        let end = matches.next().map(|a| a.parse::<u32>());
        match (start, end) {
            (Some(Ok(a)), Some(Ok(b))) => Ok(Range { start: a, end: b }),
            _ => Err(()),
        }
    }
}

struct RangePair(Range, Range);

impl RangePair {
    fn completely_overlap(&self) -> bool {
        self.0.start <= self.1.start && self.0.end >= self.1.end
            || self.0.start >= self.1.start && self.0.end <= self.1.end
    }

    fn partially_overlap(&self) -> bool {
        self.0.start <= self.1.start && self.1.start <= self.0.end
            || self.0.start <= self.1.end && self.1.end <= self.0.end
            || self.completely_overlap()
    }
}

impl FromStr for RangePair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut halves = s.split(',');
        let r1 = halves.next().map(|a| a.parse::<Range>());
        let r2 = halves.next().map(|a| a.parse::<Range>());
        match (r1, r2) {
            (Some(Ok(a)), Some(Ok(b))) => Ok(RangePair(a, b)),
            _ => Err(()),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.parse::<RangePair>().unwrap())
            .filter(|pair| pair.completely_overlap())
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.parse::<RangePair>().unwrap())
            .filter(|pair| pair.partially_overlap())
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
