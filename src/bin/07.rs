use std::collections::VecDeque;


#[derive(Debug, PartialEq)]
enum Node {
    File(usize),
    Dir {
        name: String,
        contents: Box<Vec<Node>>
    },
}

impl Node {
    pub fn new_file(size: usize) -> Self {
        Node::File(size)
    }

    pub fn new_dir(name: String) -> Self {
        Node::Dir { name, contents: Box::new(vec![]) }
    }

    pub fn insert_dir<'a>(&mut self, parent_dir: String, dir: Node) {
        let mut queue: VecDeque<&mut Node> = VecDeque::new();
        queue.push_front(self);

        loop {
            match queue.pop_back() {
                Some(Node::File(_)) => { panic!("A file is improperly on the queqe: internal logic error")},
                Some(Node::Dir { ref name, contents }) => {
                    if *name == parent_dir {
                        contents.push(dir);
                        return;
                    }
                    for mut item in contents.iter() {
                        match item {
                            Node::File(_) => { },
                            Node::Dir { name: _, contents: _ } => {
                                // queue.push_back(&mut item);
                            }
                        }
                    }
                },
                None => { return; }
            }
        }

    }
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
