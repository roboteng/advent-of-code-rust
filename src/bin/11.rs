use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u32, u64},
    multi::separated_list1,
    IResult,
};

fn monkey_id(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, id) = u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = newline(input)?;

    Ok((input, id))
}

fn starting_items(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("  Starting items: ")(input)?;
    let (input, ids) = separated_list1(tag(", "), u64)(input)?;
    let (input, _) = newline(input)?;

    Ok((input, ids))
}

#[derive(Debug, Clone, Copy)]
enum Val {
    Old,
    Const(u32),
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Times,
    Plus,
}

#[derive(Debug, Copy, Clone)]
struct Expr {
    op: Op,
    right: Val,
}

fn old(input: &str) -> IResult<&str, Val> {
    let (input, _) = tag("old")(input)?;
    Ok((input, Val::Old))
}

fn const_(input: &str) -> IResult<&str, Val> {
    let (input, c) = u32(input)?;
    Ok((input, Val::Const(c)))
}

fn op(input: &str) -> IResult<&str, Op> {
    let (input, op) = alt((tag("* "), tag("+ ")))(input)?;
    Ok((
        input,
        match op {
            "* " => Op::Times,
            "+ " => Op::Plus,
            _ => panic!("couldn't parse: {op}"),
        },
    ))
}

fn operation(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("  Operation: new = old ")(input)?;
    let (input, op) = op(input)?;
    let (input, right) = alt((old, const_))(input)?;
    let (input, _) = newline(input)?;

    Ok((input, Expr { op, right }))
}

#[derive(Debug, Clone, Copy)]
struct Test {
    divisible_by: u32,
    true_monkey: u32,
    false_monkey: u32,
}

fn test(input: &str) -> IResult<&str, Test> {
    let (input, _) = tag("  Test: divisible by ")(input)?;
    let (input, num) = u32(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("    If true: throw to monkey ")(input)?;
    let (input, true_monkey) = u32(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("    If false: throw to monkey ")(input)?;
    let (input, false_monkey) = u32(input)?;

    Ok((
        input,
        Test {
            divisible_by: num,
            true_monkey,
            false_monkey,
        },
    ))
}

#[derive(Debug)]
struct Monkey {
    #[allow(dead_code)]
    id: u32,
    items: Vec<u64>,
    operation: Expr,
    test: Test,
    inspections: u32,
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, id) = monkey_id(input).unwrap();
    let (input, ids) = starting_items(input).unwrap();
    let (input, op) = operation(input).unwrap();
    let (input, num) = test(input).unwrap();

    Ok((
        input,
        Monkey {
            id,
            items: ids,
            operation: op,
            test: num,
            inspections: 0,
        },
    ))
}

fn evaluate(old: u128, expr: Expr) -> u128 {
    match (expr.op, expr.right) {
        (Op::Times, Val::Old) => old * old,
        (Op::Times, Val::Const(c)) => old * c as u128,
        (Op::Plus, Val::Old) => old + old,
        (Op::Plus, Val::Const(c)) => old + c as u128,
    }
}

fn process_monkey(i: usize, monkeys: &mut Vec<Monkey>) {
    while !monkeys[i].items.is_empty() {
        let mut item = monkeys[i].items[0];
        monkeys[i].items = monkeys[i].items[1..].to_vec();
        item = evaluate(item as u128, monkeys[i].operation) as u64;
        item /= 3;
        monkeys[i].inspections += 1;
        let test = monkeys[i].test;
        if item % monkeys[i].test.divisible_by as u64 == 0 {
            monkeys[test.true_monkey as usize].items.push(item);
        } else {
            monkeys[test.false_monkey as usize].items.push(item);
        }
    }
}

fn process_monkey_2(i: usize, monkeys: &mut Vec<Monkey>, modulus: u128) {
    while !monkeys[i].items.is_empty() {
        let mut item = monkeys[i].items[0];
        monkeys[i].items = monkeys[i].items[1..].to_vec();
        let k = evaluate(item as u128, monkeys[i].operation);
        item = (k % modulus) as u64;
        monkeys[i].inspections += 1;
        let test = monkeys[i].test;
        if item % monkeys[i].test.divisible_by as u64 == 0 {
            monkeys[test.true_monkey as usize].items.push(item);
        } else {
            monkeys[test.false_monkey as usize].items.push(item);
        }
    }
}

fn lcm(a: u32, b: u32) -> u32 {
    a * b / gcd(a, b)
}

fn gcd(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    while a % b != 0 {
        let other = a % b;
        a = b;
        b = other;
    }
    b
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut monkeys) = separated_list1(tag("\n\n"), monkey)(input).unwrap();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            process_monkey(i, &mut monkeys);
        }
    }
    let mut inspections: Vec<u32> = monkeys.iter().map(|monkey| monkey.inspections).collect();
    inspections.sort();
    let len = inspections.len();
    Some(inspections[len - 1] * inspections[len - 2])
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, mut monkeys) = separated_list1(tag("\n\n"), monkey)(input).unwrap();
    let modulus = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .reduce(lcm)
        .unwrap();
    println!("Modulus is {modulus}");
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            process_monkey_2(i, &mut monkeys, modulus as u128);
        }
    }
    let mut inspections: Vec<u32> = monkeys.iter().map(|monkey| monkey.inspections).collect();
    inspections.sort();
    let len = inspections.len();
    Some(inspections[len - 1] as u64 * inspections[len - 2] as u64)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
