use std::collections::{HashMap, HashSet};

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
        let visited_pos = walk(input).unwrap();
        visited_pos.len()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let visited_pos = walk(input).unwrap();
        let mut obstacles = HashSet::new();
        visited_pos.keys().for_each(|pos| {
            let mut new_rows = input.row_obj.clone();
            let mut new_cols = input.col_obj.clone();
            new_rows[pos.0].push(pos.1);
            new_cols[pos.1].push(pos.0);
            new_rows[pos.0].sort_unstable();
            new_cols[pos.1].sort_unstable();
            let new_map = MAP {
                start_pos: input.start_pos,
                row_obj: new_rows,
                col_obj: new_cols,
            };
            let res = walk(&new_map);
            if res.is_none() {
                obstacles.insert(pos);
            }
        });
        obstacles.len()
    }
}

pub fn walk(input: &MAP) -> Option<HashMap<(usize, usize), Vec<DIRECTION>>> {
    let mut direction = DIRECTION::UP;
    let mut current_pos = input.start_pos;
    let mut visited_pos = HashMap::new();
    visited_pos.insert(current_pos, vec![DIRECTION::UP]);
    loop {
        match direction {
            DIRECTION::UP => {
                let Some(&(mut new_row)) = input.col_obj[current_pos.1]
                    .iter()
                    .rev()
                    .find(|&&x| x < current_pos.0)
                else {
                    for i in 0..current_pos.0 {
                        visited_pos
                            .entry((i, current_pos.1))
                            .and_modify(|x| x.push(DIRECTION::UP))
                            .or_insert(vec![DIRECTION::UP]);
                    }
                    break;
                };
                new_row += 1;
                for i in new_row..current_pos.0 {
                    let entry = visited_pos.get(&(i, current_pos.1));
                    if let Some(x) = &entry {
                        if x.contains(&DIRECTION::UP) {
                            return None;
                        } else {
                            visited_pos
                                .entry((i, current_pos.1))
                                .and_modify(|x| x.push(DIRECTION::UP));
                        }
                    } else {
                        visited_pos.insert((i, current_pos.1), vec![DIRECTION::UP]);
                    }
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
                        visited_pos
                            .entry((i, current_pos.1))
                            .and_modify(|x| x.push(DIRECTION::DOWN))
                            .or_insert(vec![DIRECTION::DOWN]);
                    }
                    break;
                };
                new_row -= 1;
                for i in current_pos.0..new_row {
                    let entry = visited_pos.get(&(i, current_pos.1));
                    if let Some(x) = &entry {
                        if x.contains(&DIRECTION::DOWN) {
                            return None;
                        } else {
                            visited_pos
                                .entry((i, current_pos.1))
                                .and_modify(|x| x.push(DIRECTION::DOWN));
                        }
                    } else {
                        visited_pos.insert((i, current_pos.1), vec![DIRECTION::DOWN]);
                    }
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
                        visited_pos
                            .entry((current_pos.0, i))
                            .and_modify(|x| x.push(DIRECTION::RIGHT))
                            .or_insert(vec![DIRECTION::RIGHT]);
                    }
                    break;
                };
                new_col -= 1;
                for i in current_pos.1..new_col + 1 {
                    let entry = visited_pos.get(&(current_pos.0, i));
                    if let Some(x) = &entry {
                        if x.contains(&DIRECTION::RIGHT) {
                            return None;
                        } else {
                            visited_pos
                                .entry((current_pos.0, i))
                                .and_modify(|x| x.push(DIRECTION::RIGHT));
                        }
                    } else {
                        visited_pos.insert((current_pos.0, i), vec![DIRECTION::RIGHT]);
                    }
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
                        visited_pos
                            .entry((current_pos.0, i))
                            .and_modify(|x| x.push(DIRECTION::LEFT))
                            .or_insert(vec![DIRECTION::LEFT]);
                    }
                    break;
                };
                new_col += 1;
                for i in new_col..current_pos.1 + 1 {
                    let entry = visited_pos.get(&(current_pos.0, i));
                    if let Some(x) = &entry {
                        if x.contains(&DIRECTION::LEFT) {
                            return None;
                        } else {
                            visited_pos
                                .entry((current_pos.0, i))
                                .and_modify(|x| x.push(DIRECTION::LEFT));
                        }
                    } else {
                        visited_pos.insert((current_pos.0, i), vec![DIRECTION::LEFT]);
                    }
                }
                current_pos = (current_pos.0, new_col);
                direction = DIRECTION::UP;
            }
        }
    }
    Some(visited_pos)
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

    #[test]
    fn test_part2() {
        let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 6);
    }

    #[test]
    fn solve_part2() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 1523);
    }
}
