pub fn part_one(input: &str) -> Option<u32> {
    let line: Vec<char> = input.chars().collect();
    let mut last = vec!['.'; 4];
    let mut packet_marker = 0;
    for (i, letter) in line.iter().enumerate() {
        last[i%4] = *letter;
        if i>3 && uniq(&last) {
            packet_marker = i+1;
            break;
        }
    }
    Some(packet_marker as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let line: Vec<char> = input.chars().collect();
    let mut last = vec!['.'; 14];
    let mut packet_marker = 0;
    // println!("{:?}", line);
    for (i, letter) in line.iter().enumerate() {
        last[i%14] = *letter;
        if i>13 && uniq(&last) {
            packet_marker = i+1;
            break;
        }
    }
    Some(packet_marker as u32)
}

fn uniq(array: &Vec<char>) -> bool {
    let mut sorted = array.to_owned();
    sorted.sort();
    sorted.dedup();
    array.len() == sorted.len()
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
