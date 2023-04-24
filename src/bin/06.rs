pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .chars()
            .collect::<Vec<char>>()
            .windows(4)
            .position(|w| {
                let mut vec = Vec::with_capacity(4);
                for x in w {
                    if vec.contains(x) {
                        return false;
                    }

                    vec.push(*x);
                }
                return true;
            })
            .unwrap() as u32 + 4,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .chars()
            .collect::<Vec<char>>()
            .windows(14)
            .position(|w| {
                let mut vec = Vec::with_capacity(14);
                for x in w {
                    if vec.contains(x) {
                        return false;
                    }

                    vec.push(*x);
                }
                return true;
            })
            .unwrap() as u32 + 14,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
