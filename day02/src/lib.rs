use aoc_traits::AdventOfCodeDay;

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<Vec<u32>>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|word| word.parse().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect()
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input.iter().filter(|x| save_lvl(x)).count()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input
            .iter()
            .map(|report| {
                for i in 0..report.len() {
                    let mut new_report = report.clone();
                    new_report.remove(i);
                    if save_lvl(&new_report) {
                        return 1;
                    }
                }
                0
            })
            .sum()
    }
}

fn save_lvl(input: &Vec<u32>) -> bool {
    let mut ascending = false;
    let mut descending = false;
    for i in 0..input.len() - 1 {
        let this: i32 = input[i].try_into().unwrap();
        let next: i32 = input[i + 1].try_into().unwrap();
        let diff: i32 = this - next;
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
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 4);
    }

    #[test]
    fn solve_part1() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 559);
    }

    #[test]
    fn solve_part2() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 601);
    }
}
