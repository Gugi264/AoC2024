use std::collections::{HashSet};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug)]
pub struct MAP {
    start_pos: (usize, usize),
    row_obj: Vec<Vec<usize>>,
    col_obj: Vec<Vec<usize>>,
}

#[derive(PartialEq, Debug)]
pub enum DIRECTION {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = MAP;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let lines = input.lines().count();
        let cols = input
            .lines()
            .next()
            .map(|first_line| first_line.chars().count())
            .unwrap();
        let mut row_obj = vec![vec![]; lines];
        let mut col_obj = vec![vec![]; cols];
        let mut start_pos = (0, 0);
        input.lines().enumerate().for_each(|(i, line)| {
            line.chars().enumerate().for_each(|(j, c)| match c {
                '#' => {
                    row_obj[i].push(j);
                    col_obj[j].push(i);
                }
                '^' => start_pos = (i, j),
                _ => {}
            });
        });

        MAP {
            start_pos,
            row_obj,
            col_obj,
        }
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let mut direction = DIRECTION::UP;
        let mut current_pos = input.start_pos;
        let mut visited_pos = HashSet::new();
        visited_pos.insert(current_pos);
        loop {
            match direction {
                DIRECTION::UP => {
                    let Some(&(mut new_row)) = input.col_obj[current_pos.1]
                        .iter()
                        .rev()
                        .find(|&&x| x < current_pos.0)
                    else {
                        for i in 0..current_pos.0 {
                            visited_pos.insert((i, current_pos.1));
                        }
                        break;
                    };
                    new_row += 1;
                    for i in new_row..current_pos.0 {
                        visited_pos.insert((i, current_pos.1));
                    }
                    current_pos = (new_row, current_pos.1);
                    direction = DIRECTION::RIGHT;
                }
                DIRECTION::DOWN => {
                    let Some(&(mut new_row)) = input.col_obj[current_pos.1]
                        .iter()
                        .find(|&&x| x > current_pos.0)
                    else {
                        for i in current_pos.0..input.row_obj.len() {
                            visited_pos.insert((i, current_pos.1));
                        }
                        break;
                    };
                    new_row -= 1;
                    for i in current_pos.0..new_row {
                        visited_pos.insert((i, current_pos.1));
                    }
                    current_pos = (new_row, current_pos.1);
                    direction = DIRECTION::LEFT;
                }
                DIRECTION::RIGHT => {
                    let Some(&(mut new_col)) = input.row_obj[current_pos.0]
                        .iter()
                        .find(|&&x| x > current_pos.1)
                    else {
                        for i in current_pos.1..input.col_obj.len() + 1 {
                            visited_pos.insert((current_pos.0, i));
                        }
                        break;
                    };
                    new_col -= 1;
                    for i in current_pos.1..new_col + 1 {
                        visited_pos.insert((current_pos.0, i));
                    }
                    current_pos = (current_pos.0, new_col);
                    direction = DIRECTION::DOWN;
                }
                DIRECTION::LEFT => {
                    let Some(&(mut new_col)) = input.row_obj[current_pos.0]
                        .iter()
                        .rev()
                        .find(|&&x| x < current_pos.1)
                    else {
                        for i in 0..current_pos.1 + 1 {
                            visited_pos.insert((current_pos.0, i));
                        }
                        break;
                    };
                    new_col += 1;
                    for i in new_col..current_pos.1 + 1 {
                        visited_pos.insert((current_pos.0, i));
                    }
                    current_pos = (current_pos.0, new_col);
                    direction = DIRECTION::UP;
                }
            }
        }
        visited_pos.len()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        todo!()
    }
}

pub fn walk(input: MAP, position: (usize, usize), direction: DIRECTION) -> {
            
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 41);
    }

    #[test]
    fn solve_part1() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 5145);
    }

    // #[test]
    // fn test_part2() {
    //     let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
    //     let input = Solver::parse_input(&file_content);
    //     assert_eq!(Solver::solve_part2(&input), 6);
    // }
    //
    // #[test]
    // fn solve_part2() {
    //     let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
    //     let input = Solver::parse_input(&file_content);
    //     assert_eq!(Solver::solve_part2(&input), 1523);
    // }
}
