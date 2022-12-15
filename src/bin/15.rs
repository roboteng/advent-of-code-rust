use nom::{bytes::complete::tag, character::complete, IResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Self) -> u32 {
        (self.x - other.x).abs() as u32 + (self.y - other.y).abs() as u32
    }
}

struct Sensor {
    loc: Point,
    nearest_beacon: Point,
}

impl Sensor {
    fn range(&self) -> u32 {
        self.loc.distance(&self.nearest_beacon)
    }

    fn is_in_range(&self, point: Point) -> bool {
        let dist = self.loc.distance(&point);
        dist <= self.range()
    }
}

fn point(input: &str) -> IResult<&str, Point> {
    let (input, _) = tag("x=")(input)?;
    let (input, x) = complete::i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = complete::i32(input)?;
    Ok((input, Point { x, y }))
}

fn sensor(input: &str) -> IResult<&str, Sensor> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, loc) = point(input)?;
    let (input, _) = tag(": closest beacon is at ")(input)?;
    let (input, nearest_beacon) = point(input)?;
    Ok((
        input,
        Sensor {
            loc,
            nearest_beacon,
        },
    ))
}

pub fn part_one(input: &str) -> Option<u32> {
    let line = 10;
    // let line = 2000000;
    let sensors: Vec<Sensor> = input.lines().map(|line| sensor(line).unwrap().1).collect();
    let min_x = sensors
        .iter()
        .map(|s| s.loc.x - s.range() as i32)
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|s| s.loc.x + s.range() as i32)
        .max()
        .unwrap();

    let mut count = 0;
    'outer: for x in min_x..=max_x {
        let point = Point { x, y: line };
        for sensor in sensors.iter() {
            if sensor.nearest_beacon == point {
                continue 'outer;
            }
        }
        for sensor in sensors.iter() {
            if sensor.is_in_range(point) {
                count += 1;
                break;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
