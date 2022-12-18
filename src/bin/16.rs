use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    multi::separated_list1,
    IResult,
};

type Name<'a> = &'a str;

#[derive(Debug, Clone, Copy)]
enum Action<'a> {
    MoveTo(Name<'a>),
    Open(Name<'a>),
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct ParsedValve<'a> {
    name: Name<'a>,
    rate: u32,
    tunnels_to: Vec<Name<'a>>,
}

fn parse_valve(input: &str) -> IResult<&str, ParsedValve> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, rate) = complete::u32(input)?;
    let (input, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ))(input)?;
    let (input, tunnels_to) = separated_list1(tag(", "), alpha1)(input)?;

    Ok((
        input,
        ParsedValve {
            name,
            rate,
            tunnels_to,
        },
    ))
}

pub fn part_one(input: &str) -> Option<u32> {
    let valves: Vec<_> = input
        .lines()
        .map(|line| parse_valve(line).unwrap().1)
        .collect();

    let pairs = valves
        .iter()
        .map(|v| v.tunnels_to.iter().map(|&n| (v.name, n)))
        .flatten()
        .collect::<Vec<(Name, Name)>>();

    let table = DistanceTable::new(pairs);
    let valved_nodes = valves
        .iter()
        .sorted_by(|a, b| a.rate.cmp(&b.rate))
        .filter_map(|v| if v.rate > 0 { Some(v.name) } else { None })
        .collect::<Vec<Name>>();

    println!("{table:?}");

    let mut current_max = 0;
    let nodes = BTreeMap::from_iter(valves.iter().map(|v| (v.name, v)));
    let max = valved_nodes
        .iter()
        .copied()
        .permutations(10)
        .enumerate()
        .map(|(i, mut perm)| {
            if i % 10_000_000 == 0 {
                println!("{i}");
            }
            let mut path = vec!["AA"];
            perm = perm.iter().rev().copied().collect();
            path.append(&mut perm);
            let eval = eval_path(&path[0..], &table, &nodes);
            if eval > current_max {
                current_max = eval;
                println!("New max: {current_max}");
            }
            eval
        })
        .max()
        .unwrap();

    Some(max)
}

fn eval_path(path: &[Name], graph: &DistanceTable, nodes: &BTreeMap<Name, &ParsedValve>) -> u32 {
    let mut pressure = 0;
    let mut minutes = 0;
    for (_, pair) in path.windows(2).enumerate() {
        let dist = graph.distance(pair[0], pair[1]);
        minutes += dist + 1;
        minutes = minutes.min(30);
        pressure += nodes[pair[1]].rate * (30 - minutes);
    }
    pressure
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug)]
struct DistanceTable<'a> {
    distances: HashMap<(Name<'a>, Name<'a>), u32>,
    nodes: BTreeSet<Name<'a>>,
}

impl<'a> DistanceTable<'a> {
    fn distance(&self, start: Name, end: Name) -> u32 {
        let [start, end] = ordered(start, end);
        *self
            .distances
            .get(&(start, end))
            .or(Some(&u32::MAX))
            .unwrap()
    }

    fn new(edges: Vec<(Name<'a>, Name<'a>)>) -> Self {
        let mut this = Self {
            distances: HashMap::new(),
            nodes: BTreeSet::new(),
        };
        this.create_nodes_and_distances(edges);
        this
    }

    fn create_nodes_and_distances(&mut self, edges: Vec<(Name<'a>, Name<'a>)>) {
        for edge in edges {
            self.nodes.insert(edge.0);
            self.nodes.insert(edge.1);
            let [start, end] = ordered(edge.0, edge.1);
            self.distances.entry((start, end)).or_insert(1);
        }

        for &node in self.nodes.iter() {
            self.distances
                .entry((node, node))
                .and_modify(|e| *e = 0)
                .or_insert(0);
        }

        let mut changed = true;
        while changed {
            changed = false;
            for start in self.nodes.iter() {
                for middle in self.nodes.iter() {
                    let middle_distance = self.distance(&start, &middle);
                    if 1 <= middle_distance && middle_distance < u32::MAX {
                        for end in self.nodes.iter() {
                            let end_distance = self.distance(&middle, &end);
                            if 1 <= end_distance && end_distance < u32::MAX {
                                if middle_distance + end_distance < self.distance(start, end) {
                                    changed = true;
                                    let [start, end] = ordered(start, end);
                                    self.distances
                                        .entry((start, end))
                                        .and_modify(|e| *e = middle_distance + end_distance)
                                        .or_insert(middle_distance + end_distance);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn known_distances(&self, node: Name) -> Vec<Name> {
        let mut ret = Vec::new();
        for &n in self.nodes.iter() {
            let distance = self.distance(node, n);
            if 1 <= distance && distance < u32::MAX {
                ret.push(n);
            }
        }
        ret
    }
}

fn ordered<'a>(a: Name<'a>, b: Name<'a>) -> [Name<'a>; 2] {
    let mut k = [a, b];
    k.sort();
    k
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

struct ActionNode<'a> {
    action: Action<'a>,
    children: Vec<ActionNode<'a>>,
}

fn generate_moves(graph: &Graph) {
    let mut pos = "AA";
    let mut depth = 0;
    let mut choices = vec![];
    choices.push(
        graph
            .neighbors(pos)
            .iter()
            .map(|&point| Action::MoveTo(point))
            .collect::<Vec<Action>>(),
    );
    while !choices.is_empty() {
        match choices.get(depth as usize) {
            Some(k) => {
                match k[0] {
                    Action::MoveTo(new_pos) => pos = new_pos,
                    Action::Open(_) => {}
                }
                if depth <= 30 {
                    let mut new_choices = graph
                        .neighbors(pos)
                        .iter()
                        .map(|point| Action::MoveTo(point))
                        .collect::<Vec<Action>>();
                    if graph.node(pos).rate > 0 {
                        new_choices.push(Action::Open(pos));
                    }
                    choices.push(new_choices);
                    depth += 1;
                } else {
                    let moves: Vec<_> = choices.iter().map(|c| c[0]).collect();
                    println!("{moves:?}");
                }
            }
            None => {
                choices.pop();
                depth -= 1;
                choices[depth] = choices[depth][1..].to_vec();
            }
        }
    }
}

fn moves<'a>(moves: &'a mut Vec<Action<'a>>) -> Vec<[Action<'a>; 30]> {
    todo!()
}

struct Graph<'a> {
    nodes: HashMap<Name<'a>, &'a ParsedValve<'a>>,
}

impl<'a> Graph<'a> {
    fn new(nodes: &'a Vec<ParsedValve<'a>>) -> Option<Self> {
        Some(Self {
            nodes: HashMap::from_iter(nodes.iter().map(|node| (node.name, node))),
        })
    }

    fn node(&self, name: Name<'a>) -> &ParsedValve<'a> {
        &self.nodes[name]
    }

    fn neighbors(&'a self, node: Name<'a>) -> &'a Vec<Name<'a>> {
        &self.node(node).tunnels_to
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_graph() {
        let input = vec![
            ParsedValve {
                name: "AA",
                rate: 0,
                tunnels_to: vec!["BB"],
            },
            ParsedValve {
                name: "BB",
                rate: 1,
                tunnels_to: vec!["AA", "CC"],
            },
            ParsedValve {
                name: "CC",
                rate: 2,
                tunnels_to: vec!["BB"],
            },
        ];
        let graph = Graph::new(&input).unwrap();
        assert_eq!(graph.node("AA"), &input[0]);
        assert_eq!(graph.neighbors("CC"), &vec!["BB"])
    }

    #[test]
    fn can_parse_valve() {
        let s = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
        let valve = parse_valve(s);

        assert_eq!(
            valve,
            Ok((
                "",
                ParsedValve {
                    name: "AA",
                    rate: 0,
                    tunnels_to: vec!["DD", "II", "BB"]
                }
            ))
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
