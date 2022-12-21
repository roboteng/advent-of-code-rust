use std::collections::HashSet;

use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete, IResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Blueprint {
    id: u32,
    ore_robot_cost: MaterialCost,
    clay_robot_cost: MaterialCost,
    obsidian_robot_cost: MaterialCost,
    geode_robot_cost: MaterialCost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BuildAction {
    Nothing,
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl BuildAction {
    fn all() -> Vec<BuildAction> {
        vec![
            BuildAction::Nothing,
            BuildAction::Ore,
            BuildAction::Clay,
            BuildAction::Obsidian,
            BuildAction::Geode,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MaterialCost {
    ore: u8,
    clay: u8,
    obsidian: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GameState {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geodes: u8,
    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    geode_robots: u8,
    time: u8,
    blueprint: Blueprint,
}

impl GameState {
    fn new(blueprint: Blueprint) -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            time: 0,
            blueprint,
        }
    }

    fn tick(&self, action: BuildAction) -> Option<GameState> {
        let mut new_state = self.clone();
        let clone = new_state.clone();

        match action {
            BuildAction::Nothing => {}
            BuildAction::Ore => {
                if new_state.has_enough(new_state.blueprint.ore_robot_cost) {
                    new_state.spend(new_state.blueprint.ore_robot_cost);
                    new_state.ore_robots += 1;
                } else {
                    return None;
                }
            }
            BuildAction::Clay => {
                if new_state.has_enough(new_state.blueprint.clay_robot_cost) {
                    new_state.spend(new_state.blueprint.clay_robot_cost);
                    new_state.clay_robots += 1;
                } else {
                    return None;
                }
            }
            BuildAction::Obsidian => {
                if new_state.has_enough(new_state.blueprint.obsidian_robot_cost) {
                    new_state.spend(new_state.blueprint.obsidian_robot_cost);
                    new_state.obsidian_robots += 1;
                } else {
                    return None;
                }
            }
            BuildAction::Geode => new_state.build_geode()?,
        }

        new_state.ore += clone.ore_robots;
        new_state.clay += clone.clay_robots;
        new_state.obsidian += clone.obsidian_robots;
        new_state.geodes += clone.geode_robots;
        new_state.time += 1;

        Some(new_state)
    }

    fn build_geode(&mut self) -> Option<()> {
        if self.has_enough(self.blueprint.geode_robot_cost) {
            self.spend(self.blueprint.geode_robot_cost);
            self.geode_robots += 1;
            Some(())
        } else {
            None
        }
    }

    fn has_enough(&self, cost: MaterialCost) -> bool {
        self.ore >= cost.ore && self.clay >= cost.clay && self.obsidian >= cost.obsidian
    }

    fn spend(&mut self, cost: MaterialCost) {
        self.ore -= cost.ore;
        self.clay -= cost.clay;
        self.obsidian -= cost.obsidian;
    }

    fn optimal_geodes(&self) -> u32 {
        let mut known_states = vec![self.clone()];
        while known_states[0].time < 24 {
            println!(
                "{} {} {:?}",
                known_states[0].time,
                known_states.len(),
                known_states.iter().map(|s| s.geodes).max()
            );
            let mut processed_states = Vec::new();
            for game in known_states.clone().iter() {
                if game.has_enough(game.blueprint.geode_robot_cost) {
                    let new = game.tick(BuildAction::Geode).unwrap();
                    processed_states.push(new);
                } else {
                    for action in BuildAction::all() {
                        if let Some(new) = game.tick(action) {
                            processed_states.push(new);
                        }
                    }
                }
            }
            known_states.clear();
            let set: HashSet<GameState> = HashSet::from_iter(processed_states.iter().copied());
            let k = set.iter().copied().unique().collect();
            known_states = k;
        }
        known_states.iter().map(|s| s.geodes as u32).max().unwrap()
    }
}

fn blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, _) = tag("Blueprint ")(input)?;
    let (input, id) = complete::u32(input)?;
    let (input, _) = tag(": Each ore robot costs ")(input)?;
    let (input, ore_robot) = complete::u8(input)?;
    let (input, _) = tag(" ore. Each clay robot costs ")(input)?;
    let (input, clay_robot) = complete::u8(input)?;
    let (input, _) = tag(" ore. Each obsidian robot costs ")(input)?;
    let (input, obs_robot_ore) = complete::u8(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, obs_robot_clay) = complete::u8(input)?;
    let (input, _) = tag(" clay. Each geode robot costs ")(input)?;
    let (input, geode_robot_ore) = complete::u8(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, geode_robot_clay) = complete::u8(input)?;
    let (input, _) = tag(" obsidian.")(input)?;

    Ok((
        input,
        Blueprint {
            id,
            ore_robot_cost: MaterialCost {
                ore: ore_robot,
                clay: 0,
                obsidian: 0,
            },
            clay_robot_cost: MaterialCost {
                ore: clay_robot,
                clay: 0,
                obsidian: 0,
            },
            obsidian_robot_cost: MaterialCost {
                ore: obs_robot_ore,
                clay: obs_robot_clay,
                obsidian: 0,
            },
            geode_robot_cost: MaterialCost {
                ore: geode_robot_ore,
                clay: geode_robot_clay,
                obsidian: 0,
            },
        },
    ))
}

pub fn part_one(input: &str) -> Option<u32> {
    let blueprints = input.lines().map(|line| blueprint(line).unwrap().1);
    let games = blueprints.map(GameState::new);
    let optimal_geodes = games.map(|game| game.optimal_geodes());

    Some(
        optimal_geodes
            .inspect(|g| println!("{g:?}"))
            .enumerate()
            .map(|(i, g)| (i as u32 + 1) * g)
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blueprint_parser() {
        let s= "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 7 clay. Each geode robot costs 4 ore and 13 obsidian.";
        let (parsed, _) = blueprint(s).unwrap();
        assert_eq!(parsed, "");
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}
