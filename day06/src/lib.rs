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
        todo!();
        //     let mut direction = DIRECTION::UP;
        //     let mut current_pos = input.start_pos;
        //     let mut visited_pos = HashMap::new();
        //     let mut obstacle_pos = HashSet::new();
        //     visited_pos.insert(current_pos, vec![DIRECTION::UP]);
        //     dbg!(&current_pos);
        //     loop {
        //         match direction {
        //             DIRECTION::UP => {
        //                 let Some(&(mut new_row)) = input.col_obj[current_pos.1]
        //                     .iter()
        //                     .rev()
        //                     .find(|&&x| x < current_pos.0)
        //                 else {
        //                     for i in (0..current_pos.0).rev() {
        //                         let pos = (i, current_pos.1);
        //                         let to_check = visited_pos
        //                             .entry(pos)
        //                             .and_modify(|x| x.push(DIRECTION::UP))
        //                             .or_insert(vec![DIRECTION::UP]);
        //                         if i > 0
        //                             && to_check.contains(&DIRECTION::RIGHT)
        //                             && visited_pos.get(&(i - 1, current_pos.1)).is_none()
        //                         {
        //                             obstacle_pos.insert((i - 1, current_pos.1));
        //                         }
        //                         for col in (0..current_pos.1).rev() {
        //                             if visited_pos
        //                                 .get(&(i, col))
        //                                 .is_some_and(|x| x.contains(&DIRECTION::RIGHT))
        //                             {
        //                                 obstacle_pos.insert((i - 1, current_pos.1));
        //                             }
        //                         }
        //                     }
        //
        //                     break;
        //                 };
        //                 new_row += 1;
        //                 for i in (new_row..current_pos.0).rev() {
        //                     let pos = (i, current_pos.1);
        //                     let to_check = visited_pos
        //                         .entry(pos)
        //                         .and_modify(|x| x.push(DIRECTION::UP))
        //                         .or_insert(vec![DIRECTION::UP]);
        //                     if i > 0
        //                         && to_check.contains(&DIRECTION::RIGHT)
        //                         && visited_pos.get(&(i - 1, current_pos.1)).is_none()
        //                     {
        //                         obstacle_pos.insert((i - 1, current_pos.1));
        //                     }
        //                     for col in (0..current_pos.1).rev() {
        //                         if visited_pos
        //                             .get(&(i, col))
        //                             .is_some_and(|x| x.contains(&DIRECTION::RIGHT))
        //                         {
        //                             obstacle_pos.insert((i - 1, current_pos.1));
        //                         }
        //                     }
        //                 }
        //                 current_pos = (new_row, current_pos.1);
        //                 direction = DIRECTION::RIGHT;
        //             }
        //             DIRECTION::DOWN => {
        //                 let Some(&(mut new_row)) = input.col_obj[current_pos.1]
        //                     .iter()
        //                     .find(|&&x| x > current_pos.0)
        //                 else {
        //                     for i in current_pos.0..input.row_obj.len() {
        //                         let pos = (i, current_pos.1);
        //                         let to_check = visited_pos
        //                             .entry(pos)
        //                             .and_modify(|x| x.push(DIRECTION::DOWN))
        //                             .or_insert(vec![DIRECTION::DOWN]);
        //                         if to_check.contains(&DIRECTION::LEFT)
        //                             && visited_pos.get(&(i + 1, current_pos.1)).is_none()
        //                         {
        //                             obstacle_pos.insert((i + 1, current_pos.1));
        //                         }
        //                         for col in (0..current_pos.1).rev() {
        //                             if visited_pos
        //                                 .get(&(i, col))
        //                                 .is_some_and(|x| x.contains(&DIRECTION::LEFT))
        //                             {
        //                                 obstacle_pos.insert((i + 1, current_pos.1));
        //                             }
        //                         }
        //                     }
        //                     break;
        //                 };
        //                 new_row -= 1;
        //                 for i in current_pos.0..new_row {
        //                     let pos = (i, current_pos.1);
        //                     let to_check = visited_pos
        //                         .entry(pos)
        //                         .and_modify(|x| x.push(DIRECTION::DOWN))
        //                         .or_insert(vec![DIRECTION::DOWN]);
        //                     if to_check.contains(&DIRECTION::LEFT)
        //                         && visited_pos.get(&(i + 1, current_pos.1)).is_none()
        //                     {
        //                         obstacle_pos.insert((i + 1, current_pos.1));
        //                     }
        //                     for col in (0..current_pos.1).rev() {
        //                         if visited_pos
        //                             .get(&(i, col))
        //                             .is_some_and(|x| x.contains(&DIRECTION::LEFT))
        //                         {
        //                             obstacle_pos.insert((i + 1, current_pos.1));
        //                         }
        //                     }
        //                 }
        //                 current_pos = (new_row, current_pos.1);
        //                 direction = DIRECTION::LEFT;
        //             }
        //             DIRECTION::RIGHT => {
        //                 let Some(&(mut new_col)) = input.row_obj[current_pos.0]
        //                     .iter()
        //                     .find(|&&x| x > current_pos.1)
        //                 else {
        //                     for i in current_pos.1..input.col_obj.len() + 1 {
        //                         let pos = (current_pos.0, i);
        //                         let to_check = visited_pos
        //                             .entry(pos)
        //                             .and_modify(|x| x.push(DIRECTION::RIGHT))
        //                             .or_insert(vec![DIRECTION::RIGHT]);
        //                         if to_check.contains(&DIRECTION::DOWN)
        //                             && visited_pos.get(&(current_pos.0, i + 1)).is_none()
        //                         {
        //                             obstacle_pos.insert((current_pos.0, i + 1));
        //                         }
        //                         for row in (0..current_pos.0).rev() {
        //                             if visited_pos
        //                                 .get(&(row, i))
        //                                 .is_some_and(|x| x.contains(&DIRECTION::DOWN))
        //                             {
        //                                 obstacle_pos.insert((current_pos.0, i + 1));
        //                             }
        //                         }
        //                     }
        //                     break;
        //                 };
        //                 new_col -= 1;
        //                 for i in current_pos.1..new_col + 1 {
        //                     let pos = (current_pos.0, i);
        //                     let to_check = visited_pos
        //                         .entry(pos)
        //                         .and_modify(|x| x.push(DIRECTION::RIGHT))
        //                         .or_insert(vec![DIRECTION::RIGHT]);
        //                     if to_check.contains(&DIRECTION::DOWN)
        //                         && visited_pos.get(&(current_pos.0, i + 1)).is_none()
        //                     {
        //                         obstacle_pos.insert((current_pos.0, i + 1));
        //                     }
        //                     for row in (0..current_pos.0).rev() {
        //                         if visited_pos
        //                             .get(&(row, i))
        //                             .is_some_and(|x| x.contains(&DIRECTION::DOWN))
        //                         {
        //                             obstacle_pos.insert((current_pos.0, i + 1));
        //                         }
        //                     }
        //                 }
        //                 current_pos = (current_pos.0, new_col);
        //                 direction = DIRECTION::DOWN;
        //             }
        //             DIRECTION::LEFT => {
        //                 dbg!("gong left");
        //                 dbg!(&current_pos);
        //                 let Some(&(mut new_col)) = input.row_obj[current_pos.0]
        //                     .iter()
        //                     .rev()
        //                     .find(|&&x| x < current_pos.1)
        //                 else {
        //                     for i in (0..current_pos.1 + 1).rev() {
        //                         let pos = (current_pos.0, i);
        //                         let to_check = visited_pos
        //                             .entry(pos)
        //                             .and_modify(|x| x.push(DIRECTION::LEFT))
        //                             .or_insert(vec![DIRECTION::LEFT]);
        //                         if to_check.contains(&DIRECTION::UP)
        //                             && visited_pos.get(&(current_pos.0, i - 1)).is_none()
        //                         {
        //                             obstacle_pos.insert((current_pos.0, i - 1));
        //                         }
        //                         for row in (0..current_pos.0).rev() {
        //                             if visited_pos
        //                                 .get(&(row, i))
        //                                 .is_some_and(|x| x.contains(&DIRECTION::UP))
        //                             {
        //                                 obstacle_pos.insert((current_pos.0, i - 1));
        //                             }
        //                         }
        //                     }
        //                     break;
        //                 };
        //                 new_col += 1;
        //                 for i in (new_col..current_pos.1 + 1).rev() {
        //                     let pos = (current_pos.0, i);
        //                     let to_check = visited_pos
        //                         .entry(pos)
        //                         .and_modify(|x| x.push(DIRECTION::LEFT))
        //                         .or_insert(vec![DIRECTION::LEFT]);
        //                     // let asdf = visited_pos.get(&(current_pos.0, i - 1)).is_none();
        //                     if to_check.contains(&DIRECTION::UP)
        //                         && visited_pos.get(&(current_pos.0, i - 1)).is_none()
        //                     {
        //                         obstacle_pos.insert((current_pos.0, i - 1));
        //                     }
        //                     for row in (0..current_pos.0).rev() {
        //                         if visited_pos
        //                             .get(&(row, i))
        //                             .is_some_and(|x| x.contains(&DIRECTION::UP))
        //                         {
        //                             obstacle_pos.insert((current_pos.0, i - 1));
        //                         }
        //                     }
        //                     // dbg!(to_check);
        //                 }
        //                 current_pos = (current_pos.0, new_col);
        //                 direction = DIRECTION::UP;
        //             }
        //         }
        //     }
        //
        //     let mut sorted_obj: Vec<&(usize, usize)> = obstacle_pos.iter().collect();
        //     sorted_obj.sort_by(|a, b| a.0.cmp(&b.0));
        //     dbg!(sorted_obj);
        //     obstacle_pos.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // #[test]
    // fn test_part1() {
    //     let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
    //     let input = Solver::parse_input(&file_content);
    //     assert_eq!(Solver::solve_part1(&input), 41);
    // }
    //
    // #[test]
    // fn solve_part1() {
    //     let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
    //     let input = Solver::parse_input(&file_content);
    //     assert_eq!(Solver::solve_part1(&input), 5145);
    // }

    // #[test]
    // fn test_part2() {
    //     let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
    //     let input = Solver::parse_input(&file_content);
    //     assert_eq!(Solver::solve_part2(&input), 6);
    // }

    #[test]
    fn solve_part2() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 1523);
    }
}
