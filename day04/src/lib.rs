use std::{collections::HashMap, usize};

use aoc_traits::AdventOfCodeDay;

#[derive(PartialEq, Debug)]
pub enum LETTERS {
    X,
    M,
    A,
    S,
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = HashMap<(usize, usize), LETTERS>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let mut parsed_input = HashMap::new();
        input.lines().enumerate().for_each(|(i, line)| {
            for (j, c) in line.chars().enumerate() {
                match c {
                    'X' => drop(parsed_input.insert((i, j), LETTERS::X)),
                    'M' => drop(parsed_input.insert((i, j), LETTERS::M)),
                    'A' => drop(parsed_input.insert((i, j), LETTERS::A)),
                    'S' => drop(parsed_input.insert((i, j), LETTERS::S)),
                    _ => {}
                };
            }
        });
        parsed_input
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input
            .iter()
            .filter(|(_, value)| **value == LETTERS::X || **value == LETTERS::S)
            .map(|(key, _)| {
                // println!("{:?}", key);
                let mut current = 0;
                let (row, col) = *key;
                //right
                if horizontal(input, row, col) {
                    current += 1;
                }
                //down
                if down(input, row, col) {
                    current += 1;
                }

                //dia up
                if diagonal_up(input, row, col) {
                    current += 1;
                }
                //dia down
                if diagonal_down(input, row, col) {
                    current += 1;
                }
                current
            })
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input
            .iter()
            .filter(|(_, value)| **value == LETTERS::A)
            .filter_map(|(key, _)| {
                let (row, col) = *key;
                let mut m_nr = 0;
                let mut s_nr = 0;
                let mut cross_vals: [&LETTERS; 4] = [&LETTERS::X; 4];

                if col == 0 {
                    return None;
                };
                if row == 0 {
                    return None;
                }

                cross_vals[0] = input.get(&(row - 1, col - 1)).unwrap_or(&LETTERS::X);
                cross_vals[1] = input.get(&(row - 1, col + 1)).unwrap_or(&LETTERS::X);
                cross_vals[2] = input.get(&(row + 1, col - 1)).unwrap_or(&LETTERS::X);
                cross_vals[3] = input.get(&(row + 1, col + 1)).unwrap_or(&LETTERS::X);

                if cross_vals[0] == cross_vals[3] {
                    return None;
                }

                cross_vals.iter().for_each(|x| match x {
                    LETTERS::M => m_nr += 1,
                    LETTERS::S => s_nr += 1,
                    _ => {}
                });
                if m_nr == 2 && s_nr == 2 {
                    // println!("{:?}", key);
                    return Some(0);
                }
                None
            })
            .count()
    }
}

pub fn horizontal(input: &HashMap<(usize, usize), LETTERS>, row: usize, col: usize) -> bool {
    let cur_letter = input.get(&(row, col)).unwrap();
    if cur_letter == &LETTERS::X
        && input.get(&(row, col + 1)) == Some(&LETTERS::M)
        && input.get(&(row, col + 2)) == Some(&LETTERS::A)
        && input.get(&(row, col + 3)) == Some(&LETTERS::S)
    {
        return true;
    }

    if cur_letter == &LETTERS::S
        && input.get(&(row, col + 1)) == Some(&LETTERS::A)
        && input.get(&(row, col + 2)) == Some(&LETTERS::M)
        && input.get(&(row, col + 3)) == Some(&LETTERS::X)
    {
        return true;
    }
    return false;
}

pub fn down(input: &HashMap<(usize, usize), LETTERS>, row: usize, col: usize) -> bool {
    let cur_letter = input.get(&(row, col)).unwrap();
    if cur_letter == &LETTERS::X
        && input.get(&(row + 1, col)) == Some(&LETTERS::M)
        && input.get(&(row + 2, col)) == Some(&LETTERS::A)
        && input.get(&(row + 3, col)) == Some(&LETTERS::S)
    {
        return true;
    }
    if cur_letter == &LETTERS::S
        && input.get(&(row + 1, col)) == Some(&LETTERS::A)
        && input.get(&(row + 2, col)) == Some(&LETTERS::M)
        && input.get(&(row + 3, col)) == Some(&LETTERS::X)
    {
        return true;
    }
    return false;
}

pub fn diagonal_up(input: &HashMap<(usize, usize), LETTERS>, row: usize, col: usize) -> bool {
    let cur_letter = input.get(&(row, col)).unwrap();
    if row < 3 {
        return false;
    };
    if cur_letter == &LETTERS::X
        && input.get(&(row - 1, col + 1)) == Some(&LETTERS::M)
        && input.get(&(row - 2, col + 2)) == Some(&LETTERS::A)
        && input.get(&(row - 3, col + 3)) == Some(&LETTERS::S)
    {
        return true;
    }
    if cur_letter == &LETTERS::S
        && input.get(&(row - 1, col + 1)) == Some(&LETTERS::A)
        && input.get(&(row - 2, col + 2)) == Some(&LETTERS::M)
        && input.get(&(row - 3, col + 3)) == Some(&LETTERS::X)
    {
        return true;
    }
    return false;
}

pub fn diagonal_down(input: &HashMap<(usize, usize), LETTERS>, row: usize, col: usize) -> bool {
    let cur_letter = input.get(&(row, col)).unwrap();
    if cur_letter == &LETTERS::X
        && input.get(&(row + 1, col + 1)) == Some(&LETTERS::M)
        && input.get(&(row + 2, col + 2)) == Some(&LETTERS::A)
        && input.get(&(row + 3, col + 3)) == Some(&LETTERS::S)
    {
        return true;
    }
    if cur_letter == &LETTERS::S
        && input.get(&(row + 1, col + 1)) == Some(&LETTERS::A)
        && input.get(&(row + 2, col + 2)) == Some(&LETTERS::M)
        && input.get(&(row + 3, col + 3)) == Some(&LETTERS::X)
    {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 18);
    }

    #[test]
    fn solve_part1() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 2603);
    }

    #[test]
    fn test_part2() {
        let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 9);
    }

    #[test]
    fn solve_part2() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 1965);
    }
}
