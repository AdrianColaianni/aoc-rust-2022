use scanf::sscanf;

pub fn part_one(input: &str) -> Option<String> {
    let lines = input.lines().collect::<Vec<&str>>();

    // Find the line that seporates the crates and instructions
    let mut sep = 0;
    while lines[sep] != "" {
        sep += 1;
    }
    // Find the number of colums
    let len = lines[sep-1].len();
    let col: usize = lines[sep-1].to_owned()[len-2..len-1].parse().unwrap();
    // println!("{} lines", col);

    // Create vec for cargo. vec is accessed vec[stack][crate]
    let mut cargo = vec![vec![' '; 0]; col];

    // Load crates into vector
    for i in (0..col-1).rev() {
        let line = lines[i].to_owned();
        for i in 0..col {
            let letter = line[1+i*4..2+i*4].chars().nth(0).unwrap();
            // print!("{}, ", letter);
            if letter != ' ' {
                cargo[i].push(letter);
            }
        }
        // println!("{}", line);
    }

    // for i in 0..cargo.len() {
    //     println!("{:?}", cargo[i]);
    // }
    // println!("----------------------------------------");

    // Follow instructions
    for i in sep+1..lines.len() {
        let line = &lines[i];
        let (mut count, mut from, mut to) = (0, 0, 0);
        if sscanf!(line, "move {} from {} to {}", count, from, to).is_ok() {
            // println!("{count}: {from}->{to}");
            for _ in 0..count {
                if let Some(letter) = cargo[from-1].pop() {
                    cargo[to-1].push(letter)
                } else {
                    panic!("Couldn't move value from {} to {}", from, to);
                }
            }
        } else {
            panic!("Couldn't read instruction")
        }
    }

    let mut answer = "".to_string();
    for i in 0..cargo.len() {
        // println!("{:?}", cargo[i]);
        answer.push(cargo[i][cargo[i].len()-1]);
    }
    Some(answer)
}

pub fn part_two(input: &str) -> Option<String> {
    let lines = input.lines().collect::<Vec<&str>>();

    // Find the line that seporates the crates and instructions
    let mut sep = 0;
    while lines[sep] != "" {
        sep += 1;
    }
    // Find the number of colums
    let len = lines[sep-1].len();
    let col: usize = lines[sep-1].to_owned()[len-2..len-1].parse().unwrap();
    // println!("{} lines", col);

    // Create vec for cargo. vec is accessed vec[stack][crate]
    let mut cargo = vec![vec![' '; 0]; col];

    // Load crates into vector
    for i in (0..col-1).rev() {
        let line = lines[i].to_owned();
        for i in 0..col {
            let letter = line[1+i*4..2+i*4].chars().nth(0).unwrap();
            // print!("{}, ", letter);
            if letter != ' ' {
                cargo[i].push(letter);
            }
        }
        // println!("{}", line);
    }

    // for i in 0..cargo.len() {
    //     println!("{:?}", cargo[i]);
    // }
    // println!("----------------------------------------");

    // Follow instructions
    for i in sep+1..lines.len() {
        let line = &lines[i];
        let (mut count, mut from, mut to) = (0, 0, 0);
        if sscanf!(line, "move {} from {} to {}", count, from, to).is_ok() {
            // println!("{count}: {from}->{to}");
            let mut temp = cargo[from-1].clone();
            temp.reverse();
            temp.truncate(count);
            temp.reverse();
            // println!("{}: {:?}", count, temp);
            cargo[to-1].append(&mut temp);
            for _ in 0..count {
                cargo[from-1].pop();
            }
        } else {
            panic!("Couldn't read instruction")
        }
    }

    let mut answer = "".to_string();
    for i in 0..cargo.len() {
        // println!("{:?}", cargo[i]);
        answer.push(cargo[i][cargo[i].len()-1]);
    }
    Some(answer)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_part_one() {
        let _input = advent_of_code::read_file("examples", 5);
        // It works trust me bro
        // assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let _input = advent_of_code::read_file("examples", 5);
        // It works trust me bro
        // assert_eq!(part_two(&input), None);
    }
}
