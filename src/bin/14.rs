use std::fmt::Display;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn part_one(input: &str) -> Option<u32> {
    let (input, points) = all_line_segments(input).unwrap();
    assert!(input.is_empty());
    let mut scene = Scene::new(points).unwrap();
    let mut count = 0;
    while scene.drop_sand() {
        count += 1;
    }
    println!("{scene}");
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (input, points) = all_line_segments(input).unwrap();
    assert!(input.is_empty());
    let mut scene = Scene::new_2(points).unwrap();
    let mut count = 0;
    while scene.drop_sand() {
        count += 1;
    }
    println!("{scene}");
    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Material {
    Rock,
    Sand,
}

#[derive(Debug)]
struct Scene {
    min_x: u32,
    max_x: u32,
    max_y: u32,
    things: Vec<Vec<Option<Material>>>,
}

impl Scene {
    fn new(lines: Vec<Vec<Point>>) -> Option<Self> {
        let points = lines.iter().flatten().collect::<Vec<&Point>>();

        let mut ret = Self {
            min_x: points.iter().map(|p| p.x).min()?,
            max_x: points.iter().map(|p| p.x).max()?,
            max_y: points.iter().map(|p| p.y).max()?,
            things: Vec::new(),
        };
        ret.set_up_scene();

        ret.draw_lines(lines);

        Some(ret)
    }

    fn new_2(lines: Vec<Vec<Point>>) -> Option<Self> {
        let points = lines.iter().flatten().collect::<Vec<&Point>>();

        let mut ret = Self {
            min_x: points.iter().map(|p| p.x).min()?,
            max_x: points.iter().map(|p| p.x).max()?,
            max_y: points.iter().map(|p| p.y).max()?,
            things: Vec::new(),
        };

        ret.max_y += 2;

        ret.min_x = ret.min_x.min(500 - ret.max_y - 1);
        ret.max_x = ret.max_x.max(500 + ret.max_y + 1);

        ret.set_up_scene();

        ret.draw_lines(lines);
        let floor = vec![vec![
            Point {
                x: ret.min_x,
                y: ret.max_y,
            },
            Point {
                x: ret.max_x,
                y: ret.max_y,
            },
        ]];
        ret.draw_lines(floor);

        Some(ret)
    }

    fn set_up_scene(&mut self) {
        self.things.resize(self.max_y as usize + 1, {
            let mut v = Vec::new();
            v.resize((self.max_x - self.min_x + 1) as usize, None);
            v
        });
    }

    fn draw_lines(&mut self, lines: Vec<Vec<Point>>) {
        for segments in lines {
            let mut _lines = segments.iter().peekable();
            while let Some(start) = _lines.next() {
                if let Some(&end) = _lines.peek() {
                    if let Some(points) = points_between(start, end) {
                        for point in points {
                            *self.at_mut(point) = Some(Material::Rock)
                        }
                    }
                }
            }
        }
    }

    fn at(&self, pos: Point) -> &Option<Material> {
        &self.things[pos.y as usize][diff(pos.x, self.min_x) as usize]
    }

    fn at_mut(&mut self, pos: Point) -> &mut Option<Material> {
        &mut self.things[pos.y as usize][diff(pos.x, self.min_x) as usize]
    }

    fn drop_sand(&mut self) -> bool {
        let mut point = Point { x: 500, y: 0 };
        if self.at(point).is_some() {
            return false;
        }
        let mut placed = false;
        loop {
            let options = self.options(point);
            let options = (
                options.0.map(|pos| (pos, self.at(pos))),
                options.1.map(|pos| (pos, self.at(pos))),
                options.2.map(|pos| (pos, self.at(pos))),
            );
            let next_move = match options {
                (_, Some((pos, None)), _) => Some(pos),
                (Some((pos, None)), Some((_, Some(_))), _) => Some(pos),
                (Some((_, Some(_))), Some((_, Some(_))), Some((pos, None))) => Some(pos),
                (Some((_, Some(_))), Some((_, Some(_))), Some((_, Some(_)))) => None,
                _ => break,
            };
            if let Some(pos) = next_move {
                point = pos;
            } else {
                *self.at_mut(point) = Some(Material::Sand);
                placed = true;
                break;
            }
        }
        placed
    }

    fn options(&self, pos: Point) -> (Option<Point>, Option<Point>, Option<Point>) {
        if pos.y + 1 > self.max_y {
            (None, None, None)
        } else {
            let bl = if pos.x - 1 >= self.min_x {
                Some(Point {
                    x: pos.x - 1,
                    y: pos.y + 1,
                })
            } else {
                None
            };
            let down = Some(Point {
                x: pos.x,
                y: pos.y + 1,
            });
            let br = if pos.x + 1 <= self.max_x {
                Some(Point {
                    x: pos.x + 1,
                    y: pos.y + 1,
                })
            } else {
                None
            };
            (bl, down, br)
        }
    }
}

fn points_between(start: &Point, end: &Point) -> Option<Vec<Point>> {
    if start.x == end.x {
        let mut ret = Vec::new();
        let d_y = diff(end.y, start.y);
        for y in 0..=d_y {
            ret.push(Point {
                x: start.x,
                y: y + start.y.min(end.y),
            })
        }
        Some(ret)
    } else if start.y == end.y {
        let mut ret = Vec::new();
        let d_x = diff(start.x, end.x);
        for x in 0..=d_x {
            ret.push(Point {
                x: x + start.x.min(end.x),
                y: start.y,
            })
        }
        Some(ret)
    } else {
        None
    }
}

impl Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let generator = 500 - self.min_x;
        let mut lines = self
            .things
            .iter()
            .map(|line| {
                line.iter()
                    .map(|spot| match spot {
                        Some(Material::Rock) => '#',
                        Some(Material::Sand) => 'o',
                        None => '.',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>();
        if self.things[0][generator as usize].is_none() {
            unsafe {
                (lines[0].as_bytes_mut())[generator as usize] = b'+';
            }
        }

        write!(f, "{}", lines.join("\n"))
    }
}

fn point(input: &str) -> IResult<&str, Point> {
    let (input, (x, y)) = separated_pair(complete::u32, tag(","), complete::u32)(input)?;
    Ok((input, Point { x, y }))
}

fn line_segments(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag(" -> "), point)(input)
}

fn all_line_segments(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    separated_list1(newline, line_segments)(input)
}

fn diff(a: u32, b: u32) -> u32 {
    a.max(b) - a.min(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drop_sand() {
        let input = advent_of_code::read_file("examples", 14);
        let (_, lines) = all_line_segments(input.as_str()).unwrap();
        let mut scene = Scene::new(lines).unwrap();
        scene.drop_sand();
        assert_eq!(*scene.at(Point { x: 500, y: 8 }), Some(Material::Sand))
    }

    #[test]
    fn draw_scene() {
        let input = advent_of_code::read_file("examples", 14);
        let (_, lines) = all_line_segments(input.as_str()).unwrap();
        let scene = Scene::new(lines).unwrap();
        let expected = "......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########.";
        assert_eq!(format!("{scene}"), expected);
    }

    #[test]
    fn prase_lines() {
        let input = "456,12 -> 456,34 -> 567,34";
        let res = line_segments(input);
        assert_eq!(
            res,
            Ok((
                "",
                vec![
                    Point { x: 456, y: 12 },
                    Point { x: 456, y: 34 },
                    Point { x: 567, y: 34 },
                ]
            ))
        );
    }

    #[test]
    fn parse_point() {
        let input = "456,12";
        let res = point(input);
        assert_eq!(res, Ok(("", Point { x: 456, y: 12 })));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
