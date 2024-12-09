use aoc_traits::AdventOfCodeDay;
use itertools::concat;
use itertools::repeat_n;
use itertools::Itertools;
use std::iter;
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
                let combinations =
                    repeat_n(ops.iter(), x.inputs.len() - 1).multi_cartesian_product();
                let found = combinations.into_iter().find(|ops_vec| {
                    let mut tmp_result = x.inputs[0];
                    ops_vec.iter().enumerate().for_each(|(i, ops)| {
                        match ops {
                            Operators::ADD => tmp_result += x.inputs[i + 1],
                            Operators::MUL => tmp_result *= x.inputs[i + 1],
                            Operators::CONCAT => {}
                        }
                        // if tmp_result > x.result {
                        //     cont
                        // }
                    });
                    if tmp_result == x.result {
                        true
                    } else {
                        false
                    }
                });
                if found.is_some() {
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
                let combinations =
                    repeat_n(ops.iter(), x.inputs.len() - 1).multi_cartesian_product();
                let found = combinations.into_iter().find(|ops_vec| {
                    let mut tmp_result = x.inputs[0];
                    ops_vec.iter().enumerate().for_each(|(i, ops)| {
                        match ops {
                            Operators::ADD => tmp_result += x.inputs[i + 1],
                            Operators::MUL => tmp_result *= x.inputs[i + 1],
                            Operators::CONCAT => {
                                tmp_result = concat_nr(tmp_result, x.inputs[i + 1]);
                            }
                        }
                        // if tmp_result > x.result {
                        //     cont
                        // }
                    });
                    if tmp_result == x.result {
                        true
                    } else {
                        false
                    }
                });
                if found.is_some() {
                    Some(x.result)
                } else {
                    None
                }
            })
            .sum()
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
        assert_eq!(Solver::solve_part2(&input), 1523);
    }
}
