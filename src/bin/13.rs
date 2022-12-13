use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace1, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone, Ord)]
enum PacketItem {
    Value(u32),
    SubPacket(Packet),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Packet {
    items: Vec<PacketItem>,
}

#[derive(Debug)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

impl PartialOrd for PacketItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (PacketItem::Value(a), PacketItem::Value(b)) => Some(a.cmp(b)),
            (PacketItem::Value(a), PacketItem::SubPacket(b)) => PacketItem::SubPacket(Packet {
                items: vec![PacketItem::Value(*a)],
            })
            .partial_cmp(&PacketItem::SubPacket(b.clone())),
            (PacketItem::SubPacket(a), PacketItem::Value(b)) => PacketItem::SubPacket(a.clone())
                .partial_cmp(&PacketItem::SubPacket(Packet {
                    items: vec![PacketItem::Value(*b)],
                })),
            (PacketItem::SubPacket(a), PacketItem::SubPacket(b)) => {
                let k = a.items.iter().zip(b.items.iter());
                for (left, right) in k {
                    if left > right {
                        return Some(Ordering::Greater);
                    } else if left < right {
                        return Some(Ordering::Less);
                    }
                }
                Some(a.items.len().cmp(&b.items.len()))
            }
        }
    }
}

fn packet_value(input: &str) -> IResult<&str, PacketItem> {
    let (input, packet_i) = complete::u32(input)?;
    Ok((input, PacketItem::Value(packet_i)))
}

fn sub_packet(input: &str) -> IResult<&str, PacketItem> {
    let (input, sub_packet) = packet(input)?;
    Ok((input, PacketItem::SubPacket(sub_packet)))
}

fn packet(input: &str) -> IResult<&str, Packet> {
    let (input, items) = delimited(
        tag("["),
        separated_list0(tag(","), alt((packet_value, sub_packet))),
        tag("]"),
    )(input)?;
    Ok((input, Packet { items }))
}

fn packet_pair(input: &str) -> IResult<&str, PacketPair> {
    let (input, (left, right)) = separated_pair(packet, newline, packet)(input)?;
    Ok((input, PacketPair { left, right }))
}

fn all_packet_pairs(input: &str) -> IResult<&str, Vec<PacketPair>> {
    separated_list1(multispace1, packet_pair)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, pairs) = all_packet_pairs(input).unwrap();
    let k: usize = pairs
        .iter()
        .enumerate()
        .map(|p| (p.0 + 1, p.1))
        .map(|(i, pair)| (i, pair.left < pair.right))
        .filter(|(_, correct)| *correct)
        .map(|(i, _)| i)
        .inspect(|i| println!("{i}"))
        .sum();

    Some(k as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, packets) = separated_list1(multispace1, packet)(input).unwrap();
    let (_, additional_packets) = separated_list1(tag(","), packet)("[[2]],[[6]]").unwrap();
    let mut packets = [packets, additional_packets.clone()].concat();
    packets.sort();
    let k: usize = packets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if *p == additional_packets[0] || *p == additional_packets[1] {
                Some(i + 1)
            } else {
                None
            }
        })
        .product();
    Some(k as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_packet() {
        let s = "[]";
        let k = packet(s);
        assert_eq!(k, Ok(("", Packet { items: Vec::new() })))
    }

    #[test]
    fn nested_empty_packet() {
        let s = "[[]]";
        let k = packet(s);
        assert_eq!(
            k,
            Ok((
                "",
                Packet {
                    items: vec![PacketItem::SubPacket(Packet { items: vec![] })]
                }
            ))
        )
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
