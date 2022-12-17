pub fn part_one(input: &str) -> Option<u32> {
    let mut elves = vec![0u32; 1];
    let mut elfi = 0;
    for line in input.lines() {
        if line == "" {
            // If empty line, add new elf
            elves.push(0);
            elfi += 1;
        } else {
            elves[elfi] += line.parse::<u32>().unwrap();
        }
    }
    let mut most = elves[0];
    for elf in elves {
        if elf > most {
            most = elf;
        }
    }
    Some(most)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves = vec![0u32; 1];
    let mut elfi = 0;
    for line in input.lines() {
        if line == "" {
            // If empty line, add new elf
            elves.push(0);
            elfi += 1;
        } else {
            elves[elfi] += line.parse::<u32>().unwrap();
        }
    }
    elves.sort();
    Some(elves[elves.len() - 1] + elves[elves.len() - 2] + elves[elves.len() - 3])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
