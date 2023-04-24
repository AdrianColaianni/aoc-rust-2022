use core::fmt;
use std::{collections::VecDeque, str::Lines};

#[derive(Debug)]
enum Token {
    Command(Command),
    Node(Node),
}

fn tokenize(mut lines: Lines) -> Node {
    let mut root = Node::new_dir("/".to_owned());
    let mut cur_dir = vec![];

    let mut line = lines.next().unwrap();
    'command: loop {
        // println!("{:?}", tokenize(line));
        match tokenize_line(line) {
            Token::Node(_) => unreachable!(),
            Token::Command(cmd) => match cmd {
                Command::Cd(dir) => {
                    if dir == ".." {
                        cur_dir.pop();
                    } else {
                        cur_dir.push(dir);
                    }
                }
                Command::Ls => loop {
                    line = match lines.next() {
                        Some(line) => line,
                        None => break 'command,
                    };
                    match tokenize_line(line) {
                        Token::Command(_) => continue 'command,
                        Token::Node(node) => {
                            // println!("Inserting dir into {:?}", cur_dir);
                            root.insert_dir(cur_dir.to_owned().into(), node);
                        }
                    }
                },
            },
        }
        line = match lines.next() {
            Some(line) => line,
            None => break 'command,
        };
    }
    return root;
}

fn tokenize_line(line: &str) -> Token {
    let line: Vec<&str> = line.split(' ').collect();
    match line[0] {
        "$" => match line[1] {
            "cd" => Token::Command(Command::Cd(line[2].to_owned())),
            "ls" => Token::Command(Command::Ls),
            _ => unreachable!(),
        },
        "dir" => Token::Node(Node::new_dir(line[1].to_owned())),
        _ => Token::Node(Node::File(
            line[0].parse().expect("Couldn't parse file size"),
        )),
    }
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug, PartialEq, Clone)]
enum Node {
    File(usize),
    Dir { name: String, contents: Vec<Node> },
}

impl Node {
    pub fn print(&self, pred: String) -> Vec<String> {
        let mut out = vec![];
        match self {
            Node::File(size) => out.push(format!("{}{} (file)", pred, size)),
            Node::Dir { name, contents } => {
                out.push(format!("{}{} (dir)", pred, name));
                for item in contents {
                    out.append(&mut item.print(format!("  {}", pred)))
                }
            }
        }
        out
    }

    pub fn new_dir(name: String) -> Self {
        Node::Dir {
            name,
            contents: vec![],
        }
    }

    pub fn is_dir(&self) -> bool {
        match self {
            Node::File(_) => false,
            Node::Dir {
                name: _,
                contents: _,
            } => true,
        }
    }

    pub fn insert_dir(&mut self, mut path: VecDeque<String>, dir: Node) {
        match self {
            Node::File(_) => unreachable!(),
            Node::Dir { name, contents } => {
                if name == &path[0] {
                    path.pop_front();
                    if path.is_empty() {
                        contents.push(dir);
                        return;
                    }
                    for item in contents {
                        match item {
                            Node::File(_) => continue,
                            Node::Dir {
                                name: _,
                                contents: _,
                            } => item.insert_dir(path.to_owned(), dir.to_owned()),
                        }
                    }
                }
            }
        }
    }

    pub fn get_size(&self) -> usize {
        match self {
            Node::File(size) => size.to_owned(),
            Node::Dir { name: _, contents } => contents.iter().map(|n| n.get_size()).sum(),
        }
    }

    pub fn get_dir_size_below_limit(&self, limit: usize) -> Vec<usize> {
        let size = self.get_size();
        if size == 0 {
            return vec![];
        }
        let mut total = if size < limit { vec![size] } else { vec![] };

        match self {
            Node::File(_) => unreachable!(),
            Node::Dir { name: _, contents } => {
                let mut contents: Vec<usize> = contents
                    .into_iter()
                    .filter(|&c| c.is_dir())
                    .flat_map(|s| s.get_dir_size_below_limit(limit))
                    .collect();
                total.append(&mut contents);
            }
        };
        total
    }

    pub fn get_dir_size_above_limit(&self, limit: usize) -> Vec<usize> {
        let size = self.get_size();
        if size == 0 {
            return vec![];
        }
        let mut total = if size > limit { vec![size] } else { vec![] };

        match self {
            Node::File(_) => unreachable!(),
            Node::Dir { name: _, contents } => {
                let mut contents: Vec<usize> = contents
                    .into_iter()
                    .filter(|&c| c.is_dir())
                    .flat_map(|s| s.get_dir_size_above_limit(limit))
                    .collect();
                total.append(&mut contents);
            }
        };
        total
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print("".to_owned()).join("\n"))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    const SIZE_LIMIT: usize = 100000;

    let lines = input.lines();
    let root = tokenize(lines);

    // println!("{}", root);
    return Some(
        root.get_dir_size_below_limit(SIZE_LIMIT)
            .iter()
            .sum::<usize>() as u32,
    );
    // < 16135449
}

pub fn part_two(input: &str) -> Option<u32> {
    const DISK_SIZE: usize = 70000000;

    let lines = input.lines();
    let root = tokenize(lines);

    let root_size: usize = root.get_size();
    let space_left = DISK_SIZE - root_size;
    let space_needed = 30000000 - space_left;
    // println!("Disk size: {}, space left: {}, size needed: {}", root_size, space_left, size_needed);

    let mut root = root.get_dir_size_above_limit(space_needed);
    root.sort();

    // println!("{:?}", root);
    return Some(
        root.iter()
            .find(|&&s| s > space_needed)
            .unwrap()
            .to_owned() as u32,
    );
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
