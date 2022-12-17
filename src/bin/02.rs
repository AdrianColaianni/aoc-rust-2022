/* A X Rock
B Y Paper
C Z Scissors
*/

pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0u32;
    for line in input.lines() {
        let mut play: Vec<char> = line.chars().collect();
        play.remove(1);
        // println!("{}, {}", play[0], play[1]);

        // Add to score for play
        score += match play[1] {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            p => {
                println!("Invalid play of {p}");
                return None;
            }
        };
        // Add to score based off win
        score += match play[0] {
            'A' => match play[1] {
                'X' => 3, // Tied
                'Y' => 6, // Won
                'Z' => 0, // Lost
                p => {
                    println!("Invalid play of {p}");
                    return None;
                }
            },
            'B' => match play[1] {
                'X' => 0, // Lost
                'Y' => 3, // Tied
                'Z' => 6, // Won
                p => {
                    println!("Invalid play of {p}");
                    return None;
                }
            },
            'C' => match play[1] {
                'X' => 6, // Won
                'Y' => 0, // Lost
                'Z' => 3, // Tied
                p => {
                    println!("Invalid play of {p}");
                    return None;
                }
            },
            p => {
                println!("Invalid play of {p}");
                return None;
            }
        }
    }
    Some(score)
}

/* A Rock       1
B Paper      2
C Scissors   3
X Lose       0
Y Draw       3
Z Win        6

13005 is too low
13830 is too high
*/

pub fn part_two(input: &str) -> Option<u32> {
    let mut score = 0u32;
    for line in input.lines() {
        let mut play: Vec<char> = line.chars().collect();
        play.remove(1);
        // println!("{}, {}", play[0], play[1]);

        // Add to score based off outcome and play
        // First match matches to outcome
        // Second match matches to what you must play to get the outcome
        score += match play[1] {
            'X' => {
                0 + match play[0] {
                    // Lose
                    'A' => 3, // Scissors
                    'B' => 1, // Rock
                    'C' => 2, // Paper
                    p => {
                        println!("Invalid play of {p}");
                        return None;
                    }
                }
            }
            'Y' => {
                3 + match play[0] {
                    // Tie
                    'A' => 1, // Rock
                    'B' => 2, // Paper
                    'C' => 3, // Scissors
                    p => {
                        println!("Invalid play of {p}");
                        return None;
                    }
                }
            }
            'Z' => {
                6 + match play[0] {
                    // Win
                    'A' => 2, // Paper
                    'B' => 3, // Scissors
                    'C' => 1, // Rock
                    p => {
                        println!("Invalid play of {p}");
                        return None;
                    }
                }
            }
            p => {
                println!("Invalid play of {p}");
                return None;
            }
        };
    }
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
