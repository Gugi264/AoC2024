use aoc_traits::AdventOfCodeDay;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Grid {
    nodes: HashMap<char, Vec<(usize, usize)>>,
    row_len: usize,
    col_len: usize,
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Grid;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let mut nodes = HashMap::new();
        let mut row_len = 0;
        let col_len = input.lines().next().unwrap().chars().count();
        input.lines().enumerate().for_each(|(row, line)| {
            row_len = row;
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .for_each(|(col, c)| {
                    nodes
                        .entry(c)
                        .and_modify(|x: &mut Vec<(usize, usize)>| x.push((row, col)))
                        .or_insert(vec![(row, col)]);
                });
        });
        Grid {
            nodes,
            row_len: row_len + 1,
            col_len,
        }
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let mut antinodes = HashSet::new();
        input.nodes.values().for_each(|val| {
            val.iter().combinations(2).for_each(|combi| {
                combi.iter().tuple_windows().for_each(|(first, second)| {
                    let row_diff = first.0 as i32 - second.0 as i32;
                    let col_diff = first.1 as i32 - second.1 as i32;

                    let first_antinode = (first.0 as i32 + row_diff, first.1 as i32 + col_diff);
                    let second_antinode = (second.0 as i32 - row_diff, second.1 as i32 - col_diff);
                    if first_antinode.0 >= 0
                        && first_antinode.1 >= 0
                        && first_antinode.0 < input.row_len as i32
                        && first_antinode.1 < input.col_len as i32
                    {
                        antinodes.insert(first_antinode);
                    }
                    if second_antinode.0 >= 0
                        && second_antinode.1 >= 0
                        && second_antinode.0 < input.row_len as i32
                        && second_antinode.1 < input.col_len as i32
                    {
                        antinodes.insert(second_antinode);
                    }
                });
            });
        });
        antinodes.len()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let mut antinodes = HashSet::new();
        input.nodes.values().for_each(|val| {
            val.iter().combinations(2).for_each(|combi| {
                combi.iter().tuple_windows().for_each(|(first, second)| {
                    let row_diff = first.0 as i32 - second.0 as i32;
                    let col_diff = first.1 as i32 - second.1 as i32;
                    antinodes.insert((first.0 as i32, first.1 as i32));
                    antinodes.insert((second.0 as i32, second.1 as i32));
                    let mut first_antinode = (first.0 as i32 + row_diff, first.1 as i32 + col_diff);
                    let mut second_antinode =
                        (second.0 as i32 - row_diff, second.1 as i32 - col_diff);

                    loop {
                        if first_antinode.0 >= 0
                            && first_antinode.1 >= 0
                            && first_antinode.0 < input.row_len as i32
                            && first_antinode.1 < input.col_len as i32
                        {
                            antinodes.insert(first_antinode);
                            first_antinode =
                                (first_antinode.0 + row_diff, first_antinode.1 + col_diff);
                        } else {
                            break;
                        }
                    }

                    loop {
                        if second_antinode.0 >= 0
                            && second_antinode.1 >= 0
                            && second_antinode.0 < input.row_len as i32
                            && second_antinode.1 < input.col_len as i32
                        {
                            antinodes.insert(second_antinode);
                            second_antinode =
                                (second_antinode.0 - row_diff, second_antinode.1 - col_diff);
                        } else {
                            break;
                        }
                    }
                });
            });
        });
        antinodes.len()
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
        assert_eq!(Solver::solve_part1(&input), 14);
    }

    #[test]
    fn solve_part1() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 252);
    }
    #[test]
    fn test_part2() {
        let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 34);
    }

    #[test]
    fn solve_part2() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 839);
    }
}
