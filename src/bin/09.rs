#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: i32,
    y: i32,
}

impl core::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

#[derive(Debug)]
struct Rope {
    head: Position,
    prev_head: Position,
    tail: Position,
    tail_visit: Vec<Position>,
}

impl Rope {
    pub fn new() -> Self {
        let start = Position { x: 0, y: 0 };
        Rope {
            head: start,
            prev_head: start,
            tail: start,
            tail_visit: vec![start],
        }
    }

    pub fn tail_too_far(&self) -> bool {
        self.head.x.abs_diff(self.tail.x) > 1 || self.head.y.abs_diff(self.tail.y) > 1
    }

    pub fn move_head(&mut self, d: &Dir) {
        self.prev_head = self.head;
        match d {
            Dir::U => self.head.y += 1,
            Dir::D => self.head.y -= 1,
            Dir::L => self.head.x -= 1,
            Dir::R => self.head.x += 1,
        }
        // println!(" {:?}", self.head);
        if self.tail_too_far() {
            // println!("Tail is too far!  Moving to {:?}", self.prev_head);
            self.move_tail();

            // Insert if it's not in the vector
            if self.tail_visit.binary_search(&self.prev_head).is_err() {
                let idx = self.tail_visit.partition_point(|&x| x < self.prev_head);
                self.tail_visit.insert(idx, self.prev_head);
            }

        }
    }

    pub fn tail_visits(&mut self) -> i32 {
        self.tail_visit.sort_by(|a, b| if a.y != b.y { a.y.cmp(&b.y) } else { a.x.cmp(&b.x) });
        // println!("{:?}", self.tail_visit);
        self.tail_visit.len() as i32
    }

    fn move_tail(&mut self) {
        if !self.tail_too_far() {
            return;
        }
        self.tail = self.prev_head;
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let input: Vec<(Dir, i32)> = input
        .lines()
        .map(|x| {
            // println!("Reading line {}", x);
            let line: Vec<&str> = x.split(' ').collect();
            let dir = match line[0] {
                "U" => Dir::U,
                "D" => Dir::D,
                "L" => Dir::L,
                "R" => Dir::R,
                _ => panic!("Read faulty direction"),
            };
            let dst: i32 = line[1].parse().unwrap();

            (dir, dst)
        })
        .collect();
    let mut rope = Rope::new();

    // println!("{:?}", input);

    for inst in input {
        for _ in 0..inst.1 {
            // print!("Moved head {:?}", inst.0);
            rope.move_head(&inst.0);
        }
    }

    Some(rope.tail_visits())
}

pub fn part_two(input: &str) -> Option<i32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
        // assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
