pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    // println!("{:?}", matching);
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        // print!("{}, {} -> ", first, second);
        let first: Vec<char> = first.chars().collect();
        let second: Vec<char> = second.chars().collect();
        // println!("{}, {}", rucksack.0, rucksack.1);
        'first: for fi in &first {
            for si in &second {
                if fi == si {
                    if fi.is_uppercase() {
                        count += fi.to_owned() as u32 - 38;
                    } else {
                        count += fi.to_owned() as u32 - 96;
                    }
                    break 'first;
                }
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut badge_sum = 0;
    // Iterate over the index.  The inside for loop will always run for three elves in a group
    for index in 0..(lines.len() / 3) {
        let index = index * 3;
        'elves: for elf1 in &lines[index] {
            for elf2 in &lines[index + 1] {
                for elf3 in &lines[index + 2] {
                    if elf1 == elf2 && elf2 == elf3 {
                        if elf1.is_uppercase() {
                            badge_sum += elf1.to_owned() as u32 - 38;
                        } else {
                            badge_sum += elf1.to_owned() as u32 - 96;
                        }
                        break 'elves;
                    }
                }
            }
        }
    }
    // matching = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    // println!("{:?}", matching);
    // a 1, A 27
    Some(badge_sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
