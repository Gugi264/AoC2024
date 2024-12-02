pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let report: Vec<i32> = line
                .split_whitespace()
                .map(|word| word.parse().unwrap())
                .collect::<Vec<i32>>();
            if save_lvl(report) {
                return 1;
            } else {
                return 0;
            }
        })
        .sum()
}

pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let report: Vec<i32> = line
                .split_whitespace()
                .map(|word| word.parse().unwrap())
                .collect::<Vec<i32>>();

            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                if save_lvl(new_report) {
                    return 1;
                }
            }
            0
        })
        .sum()
}

// fn check_wrong_index(input: Vec<i32>) -> i32 {
//
//     let mut ascending = false;
//     let mut descending = false;
//     for i in 0..input.len() - 1 {
//         let diff = input[i] - input[i + 1];
//         match diff {
//             x if x >= 1 && x <= 3 => { descending = true;
//                 if ascending {return i.try_into().unwrap()};
//             }
//             x if x <= -1 && x >= -3 => {ascending = true;
// if descending {return i.try_into().unwrap();}},
//             0 => {return i.try_into().unwrap()},
//             x if x > 0 => {if descending = fa}
//
//             _ => {
//                 return i;
//             }
//         }
//     }
//         return 99;
// }

fn save_lvl(input: Vec<i32>) -> bool {
    let mut ascending = false;
    let mut descending = false;
    for i in 0..input.len() - 1 {
        let diff = input[i] - input[i + 1];
        match diff {
            x if x >= 1 && x <= 3 => descending = true,
            x if x <= -1 && x >= -3 => ascending = true,
            _ => {
                return false;
            }
        }
    }
    if ascending != descending {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
        assert_eq!(part1(&file_content), 2);
    }

    #[test]
    fn test_part2() {
        let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
        assert_eq!(part2(&file_content), 4);
    }

    #[test]
    fn solve_part1() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        assert_eq!(part1(&file_content), 559);
    }

    #[test]
    fn solve_part2() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        assert_eq!(part2(&file_content), 601);
    }
}
