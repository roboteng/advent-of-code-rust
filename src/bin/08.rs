struct Forest {
    trees: Vec<Vec<u8>>,
}

impl Forest {
    fn trees_left(&self, x: usize, y: usize) -> Vec<u8> {
        let k = &self.trees[y][0..x].to_vec();
        k.iter().rev().map(|u| *u).collect()
    }
    fn trees_right(&self, x: usize, y: usize) -> Vec<u8> {
        let k = &self.trees[y][(x + 1)..].to_vec();
        k.clone()
    }
    fn trees_up(&self, x: usize, y: usize) -> Vec<u8> {
        let mut vec = Vec::new();
        for pos in 0..y {
            vec.push(self.trees[pos][x])
        }
        vec.iter().rev().map(|u| *u).collect()
    }
    fn trees_down(&self, x: usize, y: usize) -> Vec<u8> {
        let mut vec = Vec::new();
        for pos in (y + 1)..(self.trees.len()) {
            vec.push(self.trees[pos][x])
        }
        vec
    }

    fn is_tree_visible(&self, x: usize, y: usize) -> bool {
        let lines_of_sight = vec![
            self.trees_left(x, y),
            self.trees_right(x, y),
            self.trees_up(x, y),
            self.trees_down(x, y),
        ];
        lines_of_sight
            .iter()
            .any(|los| los.iter().all(|t| *t < self.trees[y][x]))
    }

    fn sceinic_score(&self, x: usize, y: usize) -> u32 {
        let dirs = vec![
            self.trees_up(x, y),
            self.trees_left(x, y),
            self.trees_right(x, y),
            self.trees_down(x, y),
        ];
        dirs.iter()
            .map(|trees| visible_trees(self.trees[y][x], trees))
            .reduce(|a, b| a * b)
            .unwrap()
    }
}

fn visible_trees(height: u8, trees: &Vec<u8>) -> u32 {
    let mut count = 0;
    for tree in trees.iter() {
        count += 1;
        if *tree >= height {
            return count;
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let trees: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - '0' as u8).collect())
        .collect();
    let width = trees[0].len();
    let height = trees.len();
    let forest = Forest { trees };

    let mut count = 0;
    for x in 0..width {
        for y in 0..height {
            if forest.is_tree_visible(x, y) {
                count += 1;
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let trees: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - '0' as u8).collect())
        .collect();
    let width = trees[0].len();
    let height = trees.len();
    let forest = Forest { trees };

    let mut current_max = u32::MIN;
    for x in 0..width {
        for y in 0..height {
            current_max = current_max.max(forest.sceinic_score(x, y));
        }
    }
    Some(current_max)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
