use std::ops::Deref;

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|line| line.parse::<u32>().unwrap_or(0))
                .reduce(|a, b| a + b)
                .unwrap()
        })
        .reduce(|a, b| if a > b { a } else { b })
}

pub fn part_two(input: &str) -> Option<u32> {
    let food_per_elf = input.split("\n\n").map(|lines| {
        lines
            .lines()
            .map(|line| line.parse::<u32>().unwrap_or(0))
            .reduce(|a, b| a + b)
            .unwrap()
    });
    let mut food: Vec<_> = food_per_elf.collect();
    food.sort();
    let len = food.len();
    let last_three = food.iter().skip(len - 3).map(|&a| a);
    last_three.reduce(|a, b| a + b)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
