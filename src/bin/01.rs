use std::collections::BinaryHeap;

fn food_per_elf(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|line| line.parse::<u32>().unwrap_or(0))
                .sum::<u32>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        food_per_elf(input)
            .iter()
            .reduce(|a, b| a.max(b))
            .copied()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let food: Vec<u32> = food_per_elf(input);
    let maxes = BinaryHeap::from_iter(food);

    Some(maxes.iter().take(3).sum::<u32>())
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
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
