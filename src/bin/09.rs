#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Position {
    y: i32,
    x: i32,
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

#[derive(Debug, Clone)]
struct Tail {
    pos: Position,
    visited: Vec<Position>,
}

impl Tail {
    pub fn new() -> Self {
        let pos = Position { x: 0, y: 0 };
        Tail {
            pos,
            visited: vec![pos],
        }
    }
    pub fn visited(&self) -> u32 {
        self.visited.len() as u32
    }

    pub fn move_tail(&mut self, x: i32, y: i32) {
        self.pos.x += x;
        self.pos.y += y;

        if self.visited.binary_search(&self.pos).is_err() {
            let idx = self.visited.partition_point(|&x| x < self.pos);
            self.visited.insert(idx, self.pos);
            // println!("New visited: {:?}", self.visited);
        }
    }
}

#[derive(Debug)]
struct Rope {
    head: Position,
    tails: Vec<Tail>,
    max: Position,
    min: Position,
}

impl Rope {
    pub fn new(t_count: usize) -> Self {
        let pos = Position { x: 0, y: 0 };
        Rope {
            head: pos,
            tails: vec![Tail::new(); t_count],
            max: pos,
            min: pos,
        }
    }

    pub fn tail_too_far(&self, i: usize) -> bool {
        if i >= self.tails.len() {
            panic!("Attemped to find the distance of non-existant tail");
        }

        let tail = &self.tails[i].pos;
        let prev = if i == 0 {
            &self.head
        } else {
            &self.tails[i - 1].pos
        };

        prev.x.abs_diff(tail.x) > 1 || prev.y.abs_diff(tail.y) > 1
    }

    pub fn move_head(&mut self, d: &Dir) {
        match d {
            Dir::U => self.head.y += 1,
            Dir::D => self.head.y -= 1,
            Dir::L => self.head.x -= 1,
            Dir::R => self.head.x += 1,
        }
        // println!(" {:?}", self.head);

        self.update_range();

        for i in 0..self.tails.len() {
            if !self.move_tail(i) {
                break;
            }
        }

        // self.print_board();
    }

    fn update_range(&mut self) {
        if self.max.x < self.head.x {
            self.max.x = self.head.x;
        }
        if self.max.y < self.head.y {
            self.max.y = self.head.y;
        }
        if self.min.x > self.head.x {
            self.min.x = self.head.x;
        }
        if self.min.y > self.head.y {
            self.min.y = self.head.y;
        }
    }

    pub fn tail_visits(&mut self, i: usize) -> u32 {
        self.tails[i].visited()
    }

    fn move_tail(&mut self, i: usize) -> bool {
        if !self.tail_too_far(i) {
            return false;
        }
        let tail = self.tails[i].to_owned().pos;
        let front = if i == 0 {
            &self.head
        } else {
            &self.tails[i - 1].pos
        };

        let x = front.x - tail.x;
        let y = front.y - tail.y;
        let (x, y) = match (x, x.is_positive(), y, y.is_positive()) {
            (x, _, 0, _) => ((x - 1).max(-1), 0),
            (0, _, y, _) => (0, (y - 1).max(-1)),
            (x, true, -1, _) => (x - 1, -1),
            (x, false, -1, _) => (x + 1, -1),
            (-1, _, y, true) => (-1, y - 1),
            (-1, _, y, false) => (-1, y + 1),
            (x, true, 1, _) => (x - 1, 1),
            (x, false, 1, _) => (x + 1, 1),
            (x, _, 2, _) => (x, 1),
            (x, _, -2, _) => (x, -1),
            (2, _, y, _) => (1, y),
            (-2, _, y, _) => (-1, y),
            (x, true, y, true) => match x.cmp(&y) {
                std::cmp::Ordering::Less => {
                    self.print_board();
                    todo!("Unimplemented move ({}, {})", x, y);
                }
                std::cmp::Ordering::Equal => (x / 2, y / 2),
                std::cmp::Ordering::Greater => {
                    self.print_board();
                    todo!("Unimplemented move ({}, {})", x, y);
                }
            },
            (x, _, y, _) => {
                self.print_board();
                panic!("Reached unforscene move_tail position ({x}, {y})")
            }
        };

        self.tails[i].move_tail(x, y);

        // println!(
        //     "Tail {} is too far!  Moving from {:?} to {:?}",
        //     i, tail, self.tails[i].pos
        // );

        true
    }

    pub fn print_board(&self) {
        println!(
            "Head: {:?}, min: {:?}, max: {:?}",
            self.head, self.min, self.max
        );
        for y in (self.min.y..=self.max.y).rev() {
            'x: for x in self.min.x..=self.max.x {
                let pos = Position { x, y };
                if self.head == pos {
                    print!("H");
                    continue 'x;
                }

                for (i, tail) in self.tails.iter().enumerate() {
                    if tail.pos == pos {
                        print!("{}", i + 1);
                        continue 'x;
                    }
                }

                if x == 0 && y == 0 {
                    print!("s");
                } else {
                    print!(",");
                }
            }
            println!();
        }
    }
}

fn read_input(input: &str) -> Vec<(Dir, i32)> {
    input
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
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let input: Vec<(Dir, i32)> = read_input(input);
    let mut rope = Rope::new(1);

    // println!("{:?}", input);

    for inst in input {
        for _ in 0..inst.1 {
            // print!("Moved head {:?}", inst.0);
            rope.move_head(&inst.0);
        }
    }

    Some(rope.tail_visits(0) as i32)
}

// > 2650
pub fn part_two(input: &str) -> Option<i32> {
    let input: Vec<(Dir, i32)> = read_input(input);
    let mut rope = Rope::new(9);

    // println!("{:?}", input);

    for inst in input {
        for _ in 0..inst.1 {
            // print!("Moved head {:?}", inst.0);
            rope.move_head(&inst.0);
        }
        // rope.print_board();
    }

    Some(rope.tail_visits(8) as i32)
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
        assert_eq!(part_one(&input), Some(88));
        // assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
        // assert_eq!(part_two(&input), None);
    }
}
