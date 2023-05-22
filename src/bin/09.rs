#![allow(dead_code)]
#![allow(unused_variables)]

use std::{cmp::Ordering::*, ops::{Add, AddAssign}, fmt::Debug};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    pub fn too_far(&self, other: &Point) -> bool {
        self.x.abs_diff(other.x) > 1 || self.y.abs_diff(other.y) > 1
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Clone)]
struct Knot {
    p: Point,
    past_pos: Vec<Point>,
}

impl Debug for Knot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}: {:?}", self.p, self.past_pos)
    }
}

impl Knot {
    pub fn new(x: isize, y: isize) -> Self {
        let p = Point::new(0, 0);
        Self {
            p,
            past_pos: vec![p],
        }
    }

    pub fn add_pos(&mut self, pos: Point) {
        self.p += pos;

        // println!("New pos: ({}, {})", self.p.x, self.p.y);

        // Insert in order if not found
        if self.past_pos.binary_search(&pos).is_err() {
            let idx = self.past_pos.partition_point(|&x| x < pos);
            self.past_pos.insert(idx, pos);
        }
    }

    pub fn move_near_pos(&mut self, pos: &Point) {
        let x = pos.x - self.p.x;
        let y = pos.y - self.p.y;

        let adjust = |z: isize| -> isize {
            if z.is_positive() {
                -1
            } else {
                1
            }
        };

        // print!("from {:?} ", self.p);

        if x == 0 {
            // print!("x: y={y} ");
            self.p.y = pos.y + adjust(y);
        } else if y == 0 {
            // print!("y: x={x} ");
            self.p.x = pos.x + adjust(x);
        } else {
            // print!("else ({}, {}) ", x, y);
            match x.abs().cmp(&y.abs()) {
                Less => {
                    // x < y
                    self.p.x = pos.x;
                    self.p.y = pos.y + adjust(y);
                }
                Equal => {
                    self.p.x = pos.x + adjust(x);
                    self.p.y = pos.y + adjust(y);
                }
                Greater => {
                    // x > y
                    self.p.x = pos.x + adjust(x);
                    self.p.y = pos.y;
                }
            }
        }

        // println!("to {:?}", self.p);

        if self.past_pos.binary_search(&self.p).is_err() {
            let idx = self.past_pos.partition_point(|&x| x < self.p);
            self.past_pos.insert(idx, self.p);
        }
    }

    pub fn visits(&self) -> usize {
        self.past_pos.len()
    }
}

struct Rope {
    rope: Vec<Knot>,
    min: Point,
    max: Point,
}

impl Rope {
    pub fn new(len: usize) -> Self {
        let knot = Knot::new(0, 0);
        let p = Point::new(0, 0);
        Rope {
            rope: vec![knot; len],
            min: p,
            max: p,
        }
    }

    pub fn move_rope(&mut self, motion: &str) {
        let (dir, dist) = Self::parse_move(motion);

        // println!("Moving {:?}x{}", dir, dist);

        for _ in 0..dist {
            self.rope[0].add_pos(dir);
            self.update_bounds();
            self.move_tail();
            // self.print_board();
        }
    }

    fn parse_move(motion: &str) -> (Point, usize) {
        let motion: Vec<&str> = motion.split(' ').collect();
        let dir = match motion[0] {
            "U" => Point::new(0, 1),
            "D" => Point::new(0, -1),
            "L" => Point::new(-1, 0),
            "R" => Point::new(1, 0),
            _ => panic!("Impossible motion given"),
        };
        let dist: usize = motion[1].parse().expect("Unparsable distance given");

        (dir, dist)
    }

    fn move_tail(&mut self) {
        for i in 1..self.rope.len() {
            let lead = &self.rope[i - 1];
            let lead = lead.p;
            let cur = &mut self.rope[i];

            if !cur.p.too_far(&lead) {
                break;
            }

            // print!("Moving tail [{i}]: ");

            cur.move_near_pos(&lead);
        }
    }

    pub fn visits(&self, i: usize) -> usize {
        self.rope[i].visits()
    }

    fn update_bounds(&mut self) {
        let head = &self.rope[0].p;
        if head.x > self.max.x {
            self.max.x = head.x
        }
        if head.x < self.min.x {
            self.min.x = head.x
        }
        if head.y > self.max.y {
            self.max.y = head.y
        }
        if head.y < self.min.y {
            self.min.y = head.y
        }
    }

    pub fn print_board(&self) {
        let width = self.max.x - self.min.x;
        let heigh = self.max.y - self.min.y;

        for _ in 0..width {
            print!("-");
        }

        println!("\n{:?}", self.rope);

        for y in (0..=heigh).rev() {
            'w: for x in 0..=width {
                if x == 0 && y == 0 {
                    print!("s");
                    continue 'w;
                }

                let p = Point::new(x, y);
                for i in 0..self.rope.len() {
                    if self.rope[i].p == p {
                        print!("{i}");
                        continue 'w;
                    }
                }

                print!(",");
            }
            println!();
        }
        for _ in 0..width {
            print!("-");
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut rope = Rope::new(2);

    input.lines().for_each(|x| {
        // println!("Line {}", x);
        rope.move_rope(x);
    });

    Some(rope.visits(1) as i32)
}

pub fn part_two(input: &str) -> Option<i32> {
    // return None;
    let mut rope = Rope::new(10);

    input.lines().for_each(|x| rope.move_rope(x));

    Some(rope.visits(9) as i32)
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
