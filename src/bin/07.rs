use core::fmt;

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

    pub fn insert_dir(&mut self, parent_dir: &str, dir: Node) {
        match self {
            Node::File(_) => unreachable!(),
            Node::Dir { name, contents } => {
                if name == parent_dir {
                    contents.push(dir);
                    return;
                }
                for item in contents {
                    match item {
                        Node::File(_) => continue,
                        Node::Dir {
                            name: _,
                            contents: _,
                        } => item.insert_dir(parent_dir, dir.to_owned()),
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

    pub fn get_dir_size_below_limit(&self, limit: usize) -> usize {
        let size = self.get_size();
        if size == 0 {
            return 0;
        }
        let mut total = if size < limit { size } else { 0 };

        match self {
            Node::File(_) => unreachable!(),
            Node::Dir { name: _, contents } => {
                if total != 0 {
                    // println!("Dir {:9} with size {:5} added to total", name, size);
                    // println!("{}", name);
                }
                total += contents
                    .iter()
                    .filter(|&c| c.is_dir())
                    .map(|s| s.get_dir_size_below_limit(limit))
                    .sum::<usize>();
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

    let mut lines = input.lines();
    let mut root = Node::new_dir("/".to_owned());
    let mut cur_dir = "/".to_owned();

    let mut line = lines.next().unwrap();
    'command: loop {
        // println!("{:?}", tokenize(line));
        match tokenize(line) {
            Token::Node(_) => unreachable!(),
            Token::Command(cmd) => match cmd {
                Command::Cd(dir) => cur_dir = dir,
                Command::Ls => loop {
                    line = match lines.next() {
                        Some(line) => line,
                        None => break 'command,
                    };
                    match tokenize(line) {
                        Token::Command(_) => continue 'command,
                        Token::Node(node) => root.insert_dir(&cur_dir, node),
                    }
                },
            },
        }
        line = match lines.next() {
            Some(line) => line,
            None => break 'command,
        };
    }
    println!("{}", root);
    return Some(root.get_dir_size_below_limit(SIZE_LIMIT) as u32);
    // < 16135449
}

#[derive(Debug)]
enum Token {
    Command(Command),
    Node(Node),
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

fn tokenize(line: &str) -> Token {
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

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
