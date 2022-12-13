use std::collections::HashSet;

struct Streamer<'a> {
    s: &'a str,
    i: usize,
    size: usize,
}

impl<'a> Streamer<'a> {
    fn new(s: &'a str, size: usize) -> Self {
        Self { s, i: 0, size }
    }
}

impl<'a> Iterator for Streamer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.s.len() - self.size {
            let s = &self.s[(self.i)..(self.i + self.size)];
            self.i += 1;
            Some(s)
        } else {
            None
        }
    }
}

fn all_unique(s: &str) -> bool {
    let set = HashSet::<char>::from_iter(s.chars());
    set.len() == s.len()
}

pub fn part_one(input: &str) -> Option<u32> {
    let size = 4;
    let s = Streamer::new(input, size);
    for (i, k) in s.enumerate() {
        let i = i + size;
        if all_unique(k) {
            return Some(i as u32);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let size = 14;
    let s = Streamer::new(input, size);
    for (i, k) in s.enumerate() {
        let i = i + size;
        if all_unique(k) {
            return Some(i as u32);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
