pub fn part_one(input: &str) -> Option<u32> {
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    let row_width = input.len();
    let col_width = input[0].len();

    let mut visible_trees = row_width * 2 + col_width * 2 - 4;

    // println!("Start with: {}", visible_trees);

    for c in 1..col_width - 1 {
        for r in 1..row_width - 1 {
            if visible(&input, c, r) {
                visible_trees += 1;
                // println!("There are now {} visible", visible_trees);
            }
        }
    }

    Some(visible_trees as u32)
}

/*
 * This function must determine if the tree is visible from any direction,
 * reporting true if any are the case. Each direction will be handled seporately
 * returning false if any report visible. loc is (col, row) like a graph
 *        _________ - col
 *      / [ 1 2 3 ]
 * row |  [ 4 5 6 ]
 *     \  [ 7 8 9 ]
 */
fn visible(forest: &Vec<Vec<u32>>, row: usize, column: usize) -> bool {
    let tree = &forest[row][column];
    let trow = &forest[row];
    let tcol: &Vec<u32> = &forest.iter().map(|x| x[column]).collect();
    let taller = |x: &u32| x >= tree;

    // println!("({}, {}) {}: row-{:?} col-{:?}", row, column, tree, trow, tcol);

    // println!("Left: {:?}", trow.get(0..column));
    // println!("Right {:?}", trow.get(column + 1..trow.len()));
    // println!("Up:   {:?}", tcol.get(0..row));
    // println!("Down: {:?}", tcol.get(row+1..tcol.len()));

    // Left
    if !trow.get(0..column).unwrap().iter().any(taller) {
        // println!("Visible on left");
        return true;
    }

    // Right
    if !trow.get(column + 1..trow.len()).unwrap().iter().any(taller) {
        // println!("Visible on right");
        return true;
    }

    // Up
    if !tcol.get(0..row).unwrap().iter().any(taller) {
        // println!("Visible above");
        return true;
    }

    // Down
    if !tcol.get(row + 1..tcol.len()).unwrap().iter().any(taller) {
        // println!("Visible below");
        return true;
    }

    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    let row_lim = input.len() - 1;
    let col_lim = input[0].len() - 1;

    let mut best_ss = 0;

    for r in 1..row_lim {
        for c in 1..col_lim {
            let ss = scene_score(&input, r, c);
            if ss > best_ss {
                best_ss = ss;
            }
        }
    }

    Some(best_ss)
}

fn scene_score(forest: &Vec<Vec<u32>>, row: usize, column: usize) -> u32 {
    let tree = &forest[row][column];
    let trow = &forest[row];
    let tcol: &Vec<u32> = &forest.iter().map(|x| x[column]).collect();
    let filter = |x| if x >= tree { None } else { Some(1) };

    let mut left = trow
        .get(1..column)
        .unwrap()
        .to_owned();
    left.reverse();
    let right = trow
        .get(column + 1..trow.len()-1)
        .unwrap();
    let mut up = tcol
        .get(1..row)
        .unwrap()
        .to_owned();
    up.reverse();
    let down = tcol
        .get(row + 1..tcol.len()-1)
        .unwrap();

//     println!("Left: {:?}", left);
//     println!("Right {:?}", right);
//     println!("Up:   {:?}", up);
//     println!("Down: {:?}", down);

    let left: u32 = left
        .iter()
        .map_while(filter)
        .sum::<u32>()
        + 1;
    let right: u32 = right
        .iter()
        .map_while(filter)
        .sum::<u32>()
        + 1;
    let up: u32 = up
        .iter()
        .map_while(filter)
        .sum::<u32>()
        + 1;
    let down: u32 = down
        .iter()
        .map_while(filter)
        .sum::<u32>()
        + 1;

    let ss = left * right * up * down;

    // println!("({}, {}) {}: score: {}*{}*{}*{}={}", row, column, tree, left, right, up, down, ss);

    ss
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
        assert_eq!(part_one(&input), Some(21));
        // assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
        // assert_eq!(part_two(&input), None);
    }
}
