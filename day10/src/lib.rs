use std::collections::{hash_set, HashSet};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug)]
pub struct TopgraphicMap {
    heights: Vec<u8>,
    trail_starts: Vec<usize>,
    line_length: usize,
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = TopgraphicMap;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let line_length = input.lines().next().unwrap().chars().count();
        let mut trail_starts = Vec::new();
        let mut heights = Vec::with_capacity(3_000);

        for _ in 0..line_length {
            heights.push(0 as u8);
        }

        input.lines().for_each(|line| {
            line.chars().for_each(|c| {
                let digit = c.to_digit(10).unwrap() as u8;
                heights.push(digit);
                if digit == 0 {
                    trail_starts.push(heights.len() - 1);
                }
            });
        });

        for _ in 0..line_length {
            heights.push(0);
        }

        TopgraphicMap {
            heights,
            trail_starts,
            line_length,
        }
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input.solve_part1()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input.solve_part2()
    }
}

impl TopgraphicMap {
    fn solve_part1(&self) -> usize {
        self.trail_starts
            .iter()
            .map(|trail_start| {
                assert!(self.heights[*trail_start] == 0);
                self.search_path_part1(*trail_start)
            })
            .sum()
    }
    fn solve_part2(&self) -> usize {
        self.trail_starts
            .iter()
            .map(|trail_start| {
                assert!(self.heights[*trail_start] == 0);
                self.search_path_part2(*trail_start)
            })
            .sum()
    }

    fn search_path_part1(&self, start_pos: usize) -> usize {
        let mut possible_paths = HashSet::new();
        possible_paths.insert(start_pos);
        for current_val in 0..9 {
            possible_paths = possible_paths
                .iter()
                .map(|pos| self.one_step(*pos, current_val))
                .flatten()
                .collect();
        }
        possible_paths.len()
    }

    fn search_path_part2(&self, start_pos: usize) -> usize {
        let mut possible_paths = vec![start_pos];
        for current_val in 0..9 {
            possible_paths = possible_paths
                .iter()
                .map(|pos| self.one_step(*pos, current_val))
                .flatten()
                .collect();
        }
        possible_paths.len()
    }

    fn one_step(&self, current_pos: usize, current_val: u8) -> Vec<usize> {
        let mut result = Vec::new();
        let search_val = current_val + 1;
        let position_in_line = current_pos % self.line_length;
        //left
        if position_in_line != 0 && self.heights[current_pos - 1] == search_val {
            result.push(current_pos - 1);
        }
        //right
        if position_in_line != self.line_length - 1 && self.heights[current_pos + 1] == search_val {
            result.push(current_pos + 1);
        }
        //above
        if self.heights[current_pos - self.line_length] == search_val {
            result.push(current_pos - self.line_length);
        }
        //below
        if self.heights[current_pos + self.line_length] == search_val {
            result.push(current_pos + self.line_length);
        }
        result
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
        assert_eq!(Solver::solve_part1(&input), 36);
    }

    #[test]
    fn solve_part1() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 517);
    }
    #[test]
    fn test_part2() {
        let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 81);
    }

    #[test]
    fn solve_part2() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 1116);
    }
}
