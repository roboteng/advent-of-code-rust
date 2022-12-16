use std::collections::{BTreeSet, HashSet};

use nom::{bytes::complete::tag, character::complete, IResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    range: u32,
}

impl Sensor {
    fn new(loc: Point, nearest_beacon: Point) -> Self {
        Self {
            loc,
            nearest_beacon,
            range: loc.distance(&nearest_beacon),
        }
    }
    fn is_in_range(&self, point: Point) -> bool {
        let dist = self.loc.distance(&point);
        dist <= self.range
    }

    fn covers(&self, rect: Rect) -> bool {
        rect.points().iter().all(|point| self.is_in_range(*point))
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

    Ok((input, Sensor::new(loc, nearest_beacon)))
}

pub fn part_one(input: &str) -> Option<u32> {
    let line = 10;
    // let line = 2000000;
    let sensors: Vec<Sensor> = input.lines().map(|line| sensor(line).unwrap().1).collect();
    let min_x = sensors
        .iter()
        .map(|s| s.loc.x - s.range as i32)
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|s| s.loc.x + s.range as i32)
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

pub fn part_two(input: &str) -> Option<u128> {
    let sensors: Vec<Sensor> = input.lines().map(|line| sensor(line).unwrap().1).collect();
    let range = if false { 4_000_000 } else { 20 };

    let mut rects = vec![Rect::new(0, range + 1, 0, range + 1).unwrap()];
    while !rects.is_empty() {
        let rect = rects.pop().unwrap();
        if sensors.iter().all(|sensor| !sensor.covers(rect)) {
            match rect.subdivisions() {
                Some(sub_rects) => rects = [rects, sub_rects].concat(),
                None => {
                    let point = rect.points()[0];
                    println!("{point:?}");
                    return Some(point.x as u128 * 4_000_000 + point.y as u128);
                }
            }
        }
    }

    None
}

fn main() {
    let rect = Rect::new(0, 8, 0, 8).unwrap();
    dbg!("{:?}", rect.subdivisions().unwrap());
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Rect {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Rect {
    fn new(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Option<Self> {
        if (max_x <= min_x) || (max_y <= min_y) {
            None
        } else {
            Some(Self {
                min_x,
                max_x,
                min_y,
                max_y,
            })
        }
    }

    fn points(self) -> Vec<Point> {
        let ps = vec![
            Point {
                x: self.min_x,
                y: self.min_y,
            },
            Point {
                x: self.min_x,
                y: self.max_y - 1,
            },
            Point {
                x: self.max_x - 1,
                y: self.min_y,
            },
            Point {
                x: self.max_x - 1,
                y: self.max_y - 1,
            },
        ];
        let set: HashSet<Point> = HashSet::from_iter(ps.into_iter());
        set.into_iter().collect()
    }

    fn subdivisions(self) -> Option<Vec<Rect>> {
        let dx = self.max_x - self.min_x;
        let dy = self.max_y - self.min_y;
        let hx = dx / 2;
        let hy = dy / 2;
        match (dx, dy) {
            (1, 1) => None,
            (2.., 1) => Some(vec![
                Rect::new(self.min_x, self.min_x + hx, self.min_y, self.max_y).unwrap(),
                Rect::new(self.min_x + hx, self.max_x, self.min_y, self.max_y).unwrap(),
            ]),
            (1, 2..) => Some(vec![
                Rect::new(self.min_x, self.max_x, self.min_y, self.min_y + hy).unwrap(),
                Rect::new(self.min_x, self.max_x, self.min_y + hy, self.max_y).unwrap(),
            ]),
            (2.., 2..) => Some(vec![
                Rect::new(self.min_x, self.min_x + hx, self.min_y, self.min_y + hy).unwrap(),
                Rect::new(self.min_x, self.min_x + hx, self.min_y + hy, self.max_y).unwrap(),
                Rect::new(self.min_x + hx, self.max_x, self.min_y, self.min_y + hy).unwrap(),
                Rect::new(self.min_x + hx, self.max_x, self.min_y + hy, self.max_y).unwrap(),
            ]),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subdivide_1_x_1() {
        let rect = Rect::new(2, 3, 2, 3).unwrap();
        let ssubdivisions = rect.subdivisions();
        assert_eq!(ssubdivisions, None);
    }

    #[test]
    fn subdivide_1_x_2() {
        let rect = Rect::new(2, 3, 2, 4).unwrap();
        let ssubdivisions = rect.subdivisions();
        assert_eq!(
            ssubdivisions,
            Some(vec![
                Rect::new(2, 3, 2, 3).unwrap(),
                Rect::new(2, 3, 3, 4).unwrap()
            ])
        );
    }

    #[test]
    fn subdivide_2_x_2() {
        let rect = Rect::new(0, 2, 0, 2).unwrap();
        assert_eq!(rect.subdivisions().unwrap().len(), 4);
    }

    #[test]
    fn big_sensor_covers_a_small_area() {
        let sensor = Sensor::new(Point { x: 10, y: 10 }, Point { x: 0, y: 10 });
        let rect = Rect::new(8, 13, 8, 13).unwrap();
        assert!(sensor.covers(rect));
    }

    #[test]
    fn rect_with_sides_longer_than_one_return_4_points() {
        let rect = Rect::new(8, 13, 8, 13).unwrap();
        let points = vec![
            Point { x: 8, y: 8 },
            Point { x: 8, y: 12 },
            Point { x: 12, y: 8 },
            Point { x: 12, y: 12 },
        ];
        assert!(rect.points().iter().all(|p| points.iter().any(|o| p == o)));
    }

    #[test]
    fn rect_with_a_side_one_return_2_points() {
        let rect = Rect::new(8, 9, 8, 13).unwrap();
        let points = vec![Point { x: 8, y: 8 }, Point { x: 8, y: 12 }];
        assert!(rect.points().iter().all(|p| points.iter().any(|o| p == o)));
    }

    #[test]
    fn rect_with_other_side_one_return_2_points() {
        let rect = Rect::new(8, 13, 12, 13).unwrap();
        let points = vec![Point { x: 8, y: 12 }, Point { x: 12, y: 12 }];
        assert!(rect.points().iter().all(|p| points.iter().any(|o| p == o)));
    }

    #[test]
    fn rect_with_one_return_1_points() {
        let rect = Rect::new(12, 13, 12, 13).unwrap();
        let points = vec![Point { x: 12, y: 12 }];
        assert!(rect.points().iter().all(|p| points.iter().any(|o| p == o)));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
