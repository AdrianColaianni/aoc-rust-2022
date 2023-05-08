// 689 < answ < 4139
// != 1576
pub fn part_one(input: &str) -> Option<u32> {
    let forest: Vec<Vec<u32>> = input
        .lines()
        .map(|s| s.chars().map(|s| s.to_digit(10).unwrap()).collect())
        .collect();

    // println!("{:?}", forest);

    // Account for all exterior trees
    let mut visible_count = forest[0].len() * 2 + forest.len() * 2 - 4;

    // println!("{} trees are already visible", visible_count);

    for row in 1..forest.len() - 1 {
        for column in 1..forest[row].len() - 1 {
            if visible(&forest, (column, row)) {
                visible_count += 1;
            }
        }
    }
    visible_count += 1;
    Some(visible_count.try_into().unwrap())
    // != 8085
}

/*
 * This function must determine if the tree is visible from any direction,
 * reporting true if any are the case. Each direction will be handled seporately
 * returning false if any report visible. loc is (col, row) like a graph
 *        _________ - col
 *      / [ 1 3 3 ]
 * row |  [ 3 9 9 ]
 *     \  [ 9 3 1 ]
 */
fn visible(forest: &Vec<Vec<u32>>, loc: (usize, usize)) -> bool {
    let h_limit = forest[loc.1].len();
    let row = &forest[loc.1];
    let tree = &forest[loc.1][loc.0];

    // Check left
    if !row.get(0..loc.0).unwrap().iter().any(|t| t >= tree) {
        println!("{:?}: {:?} visible left", row.get(0..loc.0).unwrap(), loc);
        return false;
    }

    // Check right
    if !row
        .get(loc.0 + 1..h_limit)
        .unwrap()
        .iter()
        .any(|t| t >= tree)
    {
        println!("{:?}: {:?} visible right", row.get(loc.0 + 1..h_limit).unwrap(), loc);
        return false;
    }

    let column: Vec<u32> = (0..forest.len())
        .map(|row_index| forest[row_index][loc.1])
        .collect();

    // Check up
    if !column.get(0..loc.0).unwrap().iter().any(|t| t >= tree) {
        println!("{:?}: {:?} visible up", column.get(0..loc.0).unwrap(), loc);
        return false;
    }

    // Check down
    if !column
        .get(loc.1 + 1..h_limit)
        .unwrap()
        .iter()
        .any(|t| t >= tree)
    {
        println!("{:?}: {:?} visible down", column.get(loc.1 + 1..h_limit).unwrap(), loc);
        return false;
    }

    true
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        // assert_eq!(part_one(&input), Some(21));
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
