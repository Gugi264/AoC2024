use aoc_traits::AdventOfCodeDay;
use std::usize;

#[derive(Debug)]
pub struct InputLine {
    result: usize,
    inputs: Vec<usize>,
}

#[derive(Debug)]
pub enum Operators {
    ADD,
    MUL,
    CONCAT,
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<InputLine>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input
            .lines()
            .map(|line| {
                let splitted = line.split_once(':').unwrap();
                let result = splitted.0.parse::<usize>().unwrap();
                let input_numbers = splitted
                    .1
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();

                InputLine {
                    result,
                    inputs: input_numbers,
                }
            })
            .collect()
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let ops = vec![Operators::ADD, Operators::MUL];
        input
            .iter()
            .filter_map(|x| {
                if search_line_result(x.result, &x.inputs[1..], x.inputs[0], &ops) {
                    Some(x.result)
                } else {
                    None
                }
            })
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let ops = vec![Operators::ADD, Operators::MUL, Operators::CONCAT];
        input
            .iter()
            .filter_map(|x| {
                if search_line_result(x.result, &x.inputs[1..], x.inputs[0], &ops) {
                    Some(x.result)
                } else {
                    None
                }
            })
            .sum()
    }
}

pub fn search_line_result(
    result: usize,
    numbers: &[usize],
    current: usize,
    operators: &[Operators],
) -> bool {
    // dbg!(&numbers);
    if numbers.len() == 0 {
        return result == current;
    }
    operators
        .iter()
        .find(|x| {
            let tmp = calculate(current, numbers[0], x);
            // dbg!(current);
            // dbg!(&x);
            // dbg!(&tmp);
            if tmp > result {
                return false;
            }
            search_line_result(result, &numbers[1..], tmp, operators)
        })
        .is_some()
}

pub fn calculate(a: usize, b: usize, operation: &Operators) -> usize {
    match operation {
        Operators::ADD => a + b,
        Operators::MUL => a * b,
        Operators::CONCAT => concat_nr(a, b),
    }
}

pub fn concat_nr(a: usize, b: usize) -> usize {
    a as usize * 10usize.pow(b.ilog10() + 1) + b as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_concat() {
        let a = 42;
        let b = 32;
        let c = 4232;

        let reuslt = concat_nr(a, b);
        assert_eq!(c, reuslt);
    }

    #[test]
    fn test_part1() {
        let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 3749);
    }

    #[test]
    fn solve_part1() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 7579994664753);
    }
    #[test]
    fn test_part2() {
        let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 11387);
    }

    #[test]
    fn solve_part2() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 438027111276610);
    }
}
