use aoc_traits::AdventOfCodeDay;
use memoize::memoize;

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<usize>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect()
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        blink_n_times_new(input, 25)
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        blink_n_times_new(input, 75)
    }
}

pub fn blink_n_times(input_stones: &Vec<usize>, blinks: usize) -> usize {
    let mut stones = Vec::with_capacity(250_000);
    stones.extend(input_stones);
    for blink in 0..blinks {
        println!("blinked {} times", blink);
        for i in 0..stones.len() {
            match stones[i] {
                0 => stones[i] = 1,
                x if count_digits(x) % 2 == 0 => {
                    let (left, right) = split_in_two(stones[i]);
                    stones[i] = left;
                    stones.push(right);
                }
                _ => stones[i] = stones[i] * 2024,
            }
        }
    }
    dbg!(stones.len());
    stones.len()
}

pub fn blink_n_times_new(input_stones: &[usize], blinks: usize) -> usize {
    // println!("blinking left: {}", blinks);
    if blinks == 0 {
        return input_stones.len();
    }
    input_stones
        .iter()
        .map(|start_stone| {
            let mut result = 0;
            // println!(
            //     "stone: {}/{}; blink: {}/{}",
            //     stone_counter,
            //     input_stones.len(),
            //     blink,
            //     blinks
            // );
            match start_stone {
                0 => result += blink_n_times_new(&[1], blinks - 1),
                x if count_digits(*x) % 2 == 0 => {
                    let (left, right) = split_in_two(*start_stone);
                    result += blink_n_times_new(&[left, right], blinks - 1)
                }
                _ => result += blink_n_times_new(&[start_stone * 2024], blinks - 1),
            }
            result
        })
        .sum()
}

// #[memoize]
pub fn split_in_two(input: usize) -> (usize, usize) {
    let stringisze = input.to_string();
    let (left, right) = stringisze.split_at(stringisze.len() / 2);
    (left.parse().unwrap(), right.parse().unwrap())
}

// #[memoize]
pub fn count_digits(input: usize) -> usize {
    if input == 0 {
        return 1;
    }

    let mut n = input;
    let mut cnt = 0;
    while n != 0 {
        n = n / 10;
        cnt += 1;
    }
    cnt
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
    // #[test]
    // fn test_part2() {
    //     let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
    //     let input = Solver::parse_input(&file_content);
    //     assert_eq!(Solver::solve_part2(&input), 81);
    // }

    #[test]
    fn solve_part2() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 1116);
    }
}
