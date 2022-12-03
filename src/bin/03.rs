#![feature(iter_array_chunks)]
#![feature(iter_next_chunk)]
use std::collections::HashSet;

fn halves(s: &str) -> [&str; 2] {
    let len = s.len();
    if len % 2 != 0 {
        panic!("Not divisibleby two");
    } else {
        let half_len = len / 2;
        let s1 = &s[0..half_len];
        let s2 = &s[half_len..len];
        [s1, s2]
    }
}

fn priority(letter: &char) -> u32 {
    match letter {
        'a'..='z' => (*letter as u8 - 'a' as u8 + 1) as u32,
        'A'..='Z' => (*letter as u8 - 'A' as u8 + 27) as u32,
        _ => panic!("Unknown letter {}", letter),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(halves)
        .map(|halves| {
            let set1 = halves[0].chars().fold(HashSet::new(), |mut set, letter| {
                set.insert(letter);
                set
            });
            let set2 = halves[1].chars().fold(HashSet::new(), |mut set, letter| {
                set.insert(letter);
                set
            });
            let mut sum = 0;
            for letter in set1.intersection(&set2) {
                sum += priority(letter);
            }
            sum
        })
        .reduce(|a, b| a + b)
}

fn intersection(sets: [HashSet<char>; 3]) -> char {
    for letters0 in sets[0].iter() {
        for letters1 in sets[1].iter() {
            for letters2 in sets[2].iter() {
                if letters0 == letters1 && letters1 == letters2 {
                    return *letters0;
                }
            }
        }
    }
    panic!("no intersection found");
}

pub fn part_two(input: &str) -> Option<u32> {
    let pack_groups = input.lines().array_chunks::<3>();

    let badges = pack_groups.map(|pack_group| {
        let mut items = pack_group.iter().map(|pack| {
            let mut set = HashSet::new();
            for letter in pack.chars() {
                set.insert(letter);
            }
            set
        });
        items.next_chunk::<3>().unwrap()
    });
    badges
        .map(intersection)
        .map(|c| priority(&c))
        .reduce(|a, b| a + b)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
