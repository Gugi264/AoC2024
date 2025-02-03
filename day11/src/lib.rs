use std::collections::HashMap;

use aoc_traits::AdventOfCodeDay;
type Cache = HashMap<(u64, u16), u64>;

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<u64>;
    type Part1Output = u64;
    type Part2Output = u64;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect()
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        blink_n_times(input, 25)
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        blink_n_times(input, 75)
    }
}

pub fn blink_n_times(stones: &Vec<u64>, blinks: u16) -> u64 {
    let mut cache: Cache = HashMap::new();
    stones
        .iter()
        .map(|stone| blink_rec(*stone, blinks, &mut cache))
        .sum()
}

pub fn blink_rec(stone: u64, blinks_left: u16, cache: &mut Cache) -> u64 {
    if let Some(x) = cache.get(&(stone, blinks_left)) {
        return *x;
    } else {
        if blinks_left == 0 {
            return 1;
        }

        let new_stones = blink_once(stone);
        new_stones
            .iter()
            .map(|new_stone| {
                let tmp_res = blink_rec(*new_stone, blinks_left - 1, cache);
                cache.insert((*new_stone, blinks_left - 1), tmp_res);
                tmp_res
            })
            .sum()
    }
}

pub fn blink_once(input_stone: u64) -> Vec<u64> {
    let nr_digits = count_digits(input_stone);
    match input_stone {
        0 => vec![1],
        _ if nr_digits % 2 == 0 => {
            let raised = 10_u64.pow(nr_digits / 2);
            let left = input_stone / raised;
            let right = input_stone % raised;
            vec![left, right]
        }
        _ => vec![input_stone * 2024],
    }
}

pub fn count_digits(input: u64) -> u32 {
    input.checked_ilog10().unwrap_or_default() + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1_25_blinks() {
        let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 55312);
    }

    #[test]
    fn solve_part1() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 204022);
    }

    #[test]
    fn test_part2() {
        let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 65601038650482);
    }

    #[test]
    fn solve_part2() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 241651071960597);
    }
}
