use nom::{
    self,
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, newline, u32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq)]
enum Op<'a> {
    Cd(Cd<'a>),
    Ls(Vec<FSObj<'a>>),
}

#[derive(Debug, PartialEq)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug, PartialEq)]
enum FSObj<'a> {
    File { size: u32, name: &'a str },
    Dir(&'a str),
}

#[derive(Debug, Clone)]
struct File<'a> {
    size: u32,
    #[allow(dead_code)]
    name: &'a str,
}

#[derive(Debug, Clone)]
struct Folder<'a> {
    name: &'a str,
    files: Vec<File<'a>>,
    folders: Vec<Folder<'a>>,
}

impl<'a> Folder<'a> {
    fn find(&mut self, path: &[&'a str]) -> &mut Folder<'a> {
        if path.is_empty() {
            return self;
        }
        for folder in &mut self.folders {
            if folder.name == path[0] {
                return folder.find(&path[1..]);
            }
        }
        panic!("No folder found at {path:?}")
    }

    fn size(&self) -> u32 {
        self.files.iter().map(|f| f.size).sum::<u32>()
            + self.folders.iter().map(|f| f.size()).sum::<u32>()
    }
}

#[derive(Debug, Clone)]
struct FolderWalker<'a> {
    folder: &'a Folder<'a>,
    state: NextIteration<'a>,
}

#[derive(Debug, Clone)]
enum NextIteration<'a> {
    Own,
    StartChild(usize),
    Child(usize, Box<FolderWalker<'a>>),
    Done,
}

impl<'a> FolderWalker<'a> {
    fn new(folder: &'a Folder<'a>) -> Self {
        Self {
            folder,
            state: NextIteration::Own,
        }
    }
}

impl<'a> Iterator for FolderWalker<'a> {
    type Item = &'a Folder<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state.clone() {
            NextIteration::Own => {
                self.state = NextIteration::StartChild(0);
                Some(self.folder)
            }
            NextIteration::StartChild(i) => {
                match self.folder.folders.get(i) {
                    Some(folder) => {
                        self.state = NextIteration::Child(i, Box::new(FolderWalker::new(folder)))
                    }
                    None => self.state = NextIteration::Done,
                }
                self.next()
            }
            NextIteration::Child(i, mut child) => match child.next() {
                Some(folder) => {
                    self.state = NextIteration::Child(i, child);
                    Some(folder)
                }
                None => {
                    self.state = NextIteration::StartChild(i + 1);
                    self.next()
                }
            },
            NextIteration::Done => None,
        }
    }
}

fn dir(input: &str) -> IResult<&str, FSObj> {
    let (input, (_, name)) = separated_pair(tag("dir"), tag(" "), alpha1)(input)?;
    Ok((input, FSObj::Dir(name)))
}

fn file(input: &str) -> IResult<&str, FSObj> {
    let (input, (size, name)) =
        separated_pair(u32, tag(" "), is_a("abcdefghijklmnopqrstuvwxyz."))(input)?;
    Ok((input, FSObj::File { size, name }))
}

fn ls(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, objs) = separated_list1(newline, alt((dir, file)))(input)?;

    Ok((input, Op::Ls(objs)))
}

fn cd(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, cd) = alt((tag(".."), tag("/"), alpha1))(input)?;
    let op = match cd {
        ".." => Cd::Up,
        "/" => Cd::Root,
        s => Cd::Down(s),
    };
    Ok((input, Op::Cd(op)))
}

fn commands(input: &str) -> IResult<&str, Vec<Op>> {
    let (input, cmd) = separated_list1(newline, alt((ls, cd)))(input)?;
    Ok((input, cmd))
}

pub fn part_one(input: &str) -> Option<u32> {
    match commands(input) {
        Ok((_, cmds)) => {
            let mut root = Folder {
                name: "",
                files: vec![],
                folders: vec![],
            };
            let mut current_path = Vec::new();
            for cmd in cmds {
                let current = root.find(&current_path[0..]);
                match cmd {
                    Op::Cd(Cd::Root) => {}
                    Op::Cd(Cd::Up) => {
                        current_path.pop();
                    }
                    Op::Cd(Cd::Down(dir)) => {
                        current_path.push(dir);
                    }

                    Op::Ls(fs_objs) => {
                        for obj in fs_objs {
                            match obj {
                                FSObj::File { size, name } => {
                                    current.files.push(File { size, name })
                                }
                                FSObj::Dir(dir) => current.folders.push(Folder {
                                    name: dir,
                                    files: Vec::new(),
                                    folders: Vec::new(),
                                }),
                            }
                        }
                    }
                }
            }
            let walker = FolderWalker::new(&root);
            Some(
                walker
                    .map(|dir| dir.size())
                    .filter(|&size| size <= 100_000)
                    .sum(),
            )
        }
        Err(err) => panic!("{err}"),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, cmds) = commands(input).unwrap();
    let mut root = Folder {
        name: "",
        files: vec![],
        folders: vec![],
    };
    let mut current_path = Vec::new();
    for cmd in cmds {
        let current = root.find(&current_path[0..]);
        match cmd {
            Op::Cd(Cd::Root) => {}
            Op::Cd(Cd::Up) => {
                current_path.pop();
            }
            Op::Cd(Cd::Down(dir)) => {
                current_path.push(dir);
            }

            Op::Ls(fs_objs) => {
                for obj in fs_objs {
                    match obj {
                        FSObj::File { size, name } => current.files.push(File { size, name }),
                        FSObj::Dir(dir) => current.folders.push(Folder {
                            name: dir,
                            files: Vec::new(),
                            folders: Vec::new(),
                        }),
                    }
                }
            }
        }
    }
    let walker = FolderWalker::new(&root);
    let total_size = 70000000;
    let required_space = 30000000;
    let max_size = total_size - required_space;
    let current_size = root.size();
    let mut sizes = walker
        .map(|dir| dir.size())
        .filter(|size| current_size - size < max_size)
        .collect::<Vec<u32>>();
    sizes.sort();
    Some(sizes[0])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ls() {
        let input = "$ ls
dir a";
        let k = ls(input).unwrap();
        assert_eq!(("", Op::Ls(vec![FSObj::Dir("a")])), k);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
