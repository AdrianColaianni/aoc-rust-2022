use scanf::sscanf;

pub fn part_one(input: &str) -> Option<u32> {
    let mut encapsulations = 0;
    for line in input.lines() {
        let (mut first_low, mut first_high, mut second_low, mut second_high) = (0u32, 0u32, 0u32, 0u32);
        if sscanf!(&line, "{}-{},{}-{}", first_low, first_high, second_low, second_high).is_ok() {
            if (first_low >= second_low && first_high <= second_high)
                || (second_low >= first_low && second_high <= first_high) {
                    encapsulations += 1;
                }
        } else {
            panic!("Couldn't parse line");
        }
        // println!("{:?}", line);
    }
    Some(encapsulations)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut overlaps = 0;
    for line in input.lines() {
        let (mut first_low, mut first_high, mut second_low, mut second_high) = (0u32, 0u32, 0u32, 0u32);
        if sscanf!(&line, "{}-{},{}-{}", first_low, first_high, second_low, second_high).is_ok() {
            if (second_low >= first_low && second_low <= first_high)
                || (second_high >= first_low && second_high <= first_high)
                    || (first_low >= second_low && first_low <= second_high)
                    || (first_high >= second_low && first_high <= second_high) {
                        overlaps += 1;
                    }
        } else {
            println!("Couldn't parse line");
            return None;
        }
    }
    Some(overlaps)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
