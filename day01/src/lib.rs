use std::{collections::HashMap, iter::zip};

use aoc_traits::AdventOfCodeDay;

pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = (Vec<u32>, Vec<u32>);
    type Part1Output = u32;
    type Part2Output = u32;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let mut left_side = Vec::new();
        let mut right_side = Vec::new();

        input.lines().for_each(|line| {
            let mut parts = line.split_whitespace();
            left_side.push(parts.next().unwrap().parse().expect(".."));
            right_side.push(parts.next().unwrap().parse().expect(".."));
        });

        (left_side, right_side)
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let (mut left, mut right) = input.clone();
        assert_eq!(left.len(), right.len());
        left.sort_unstable();
        right.sort_unstable();
        zip(left, right).map(|(l, r)| l.abs_diff(r)).sum()
    }

    fn solve_part2((left_input, right_input): &Self::ParsedInput<'_>) -> Self::Part2Output {
        let mut left_side = HashMap::new();
        let mut right_side = HashMap::new();

        zip(left_input, right_input).for_each(|(left, right)| {
            left_side.entry(left).and_modify(|x| *x += 1).or_insert(1);
            right_side.entry(right).and_modify(|x| *x += 1).or_insert(1);
        });

        left_side
            .into_iter()
            .map(|(lkey, lval)| right_side.get(&lkey).unwrap_or(&0) * lkey * lval)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;

    #[test]
    fn test_part1() {
        let puzzle_input = " 3   4
                4   3
                2   5
                1   3
                3   9
                3   3";
        let input = Solver::parse_input(puzzle_input);
        assert_eq!(Solver::solve_part1(&input), 11);
    }

    #[test]
    fn test_part2() {
        let puzzle_input = " 3   4
                4   3
                2   5
                1   3
                3   9
                3   3";
        let input = Solver::parse_input(puzzle_input);
        assert_eq!(Solver::solve_part2(&input), 31);
    }

    #[test]
    fn solve_part1() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 2904518);
    }

    #[test]
    fn solve_part2() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 18650129);
    }
}
