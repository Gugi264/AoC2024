use std::{usize, vec};

use aoc_traits::AdventOfCodeDay;
use itertools::Itertools;

#[derive(Debug)]
pub struct DiskMap {
    files: Vec<u8>,
    free_blocks: Vec<u8>,
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = DiskMap;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let mut free_blocks = Vec::with_capacity(10_000);
        let mut files = Vec::with_capacity(10_000);
        let mut input_iter = input.chars();
        let first = input_iter.next().unwrap().to_digit(10).unwrap();
        files.push(first as u8);

        dbg!(first);
        input_iter.tuples().for_each(|(block, file)| {
            free_blocks.push(block.to_digit(10).unwrap() as u8);
            files.push(file.to_digit(10).unwrap() as u8);
        });
        dbg!(files.len());
        dbg!(free_blocks.len());
        DiskMap { files, free_blocks }
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input.solve_part1()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input.solve_part2()
    }
}

impl DiskMap {
    fn solve_part2(&self) -> usize {
        let mut result: usize = 0;
        let mut file_fwd = self.files.iter().enumerate();
        let mut block_it = self.free_blocks.iter();
        let mut finished_ids = Vec::with_capacity(6000);
        let mut file_rev_id = self.files.len() - 1;

        let mut index = 0;

        let mut current_free_blocks;
        'outer: loop {
            // dbg!("loop start");
            let next_fwd = file_fwd.next();
            if next_fwd.is_none() {
                println!("breakd next_fwd is none");
                break;
            }
            let (fwd_id, fwd_nr_elem) = next_fwd.unwrap();
            dbg!(fwd_id);
            if fwd_id > file_rev_id {
                break;
            }
            if !finished_ids.contains(&fwd_id) {
                for _ in 0..*fwd_nr_elem {
                    // dbg!(index);
                    // dbg!(fwd_id);
                    result += index * fwd_id;
                    index += 1;
                }
            } else {
                // dbg!("skipped {}", fwd_nr_elem);
                index += *fwd_nr_elem as usize;
            }

            if let Some(free_block) = block_it.next() {
                current_free_blocks = *free_block;
            } else {
                println!("free block break");
                break 'outer;
            }

            for x in (fwd_id + 1..file_rev_id + 1).rev() {
                if finished_ids.contains(&x) {
                    continue;
                }
                if self.files[x] <= current_free_blocks {
                    for _ in 0..self.files[x] {
                        result += index * x;
                        index += 1;
                    }
                    if x == file_rev_id {
                        file_rev_id -= 1;
                    } else {
                        finished_ids.push(x);
                    }
                    current_free_blocks -= self.files[x];
                    while finished_ids.contains(&file_rev_id) {
                        file_rev_id -= 1;
                    }
                }
                if current_free_blocks == 0 {
                    continue 'outer;
                }
            }
            index += current_free_blocks as usize;
        }

        result
    }

    fn solve_part1(&self) -> usize {
        let mut result: usize = 0;
        let mut file_rev = self.files.iter().enumerate().rev();
        let mut file_fwd = self.files.iter().enumerate();
        let mut block_it = self.free_blocks.iter();

        let mut index = 0;

        let (rev_id, rev_nr_elem) = file_rev.next().unwrap();
        let mut rev_id = rev_id;
        let mut rev_nr_elem = rev_nr_elem.clone();
        'outer: loop {
            let (fwd_id, fwd_nr_elem) = file_fwd.next().unwrap();
            if fwd_id >= rev_id {
                if fwd_id == rev_id {
                    for _ in 0..rev_nr_elem {
                        result += index * rev_id;
                        index += 1;
                    }
                }
                break 'outer;
            }
            for _ in 0..*fwd_nr_elem {
                result += index * fwd_id;
                index += 1;
            }

            if let Some(free_block) = block_it.next() {
                for _ in 0..*free_block {
                    if rev_nr_elem == 0 {
                        let (tmp_rev_id, tmp_rev_elem) = file_rev.next().unwrap();
                        rev_id = tmp_rev_id;
                        rev_nr_elem = tmp_rev_elem.clone();

                        if rev_id == fwd_id {
                            break 'outer;
                        };
                    }
                    result += index * rev_id;
                    index += 1;
                    rev_nr_elem -= 1;
                }
            } else {
                break;
            }
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
        assert_eq!(Solver::solve_part1(&input), 1928);
    }

    #[test]
    fn solve_part1() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 6446899523367);
    }
    #[test]
    fn test_part2() {
        let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 2858);
    }

    #[test]
    fn extra_test() {
        let input_string = "1313165";
        let input = Solver::parse_input(&input_string);
        assert_eq!(Solver::solve_part2(&input), 169);
    }

    #[test]
    fn solve_part2() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 6478232739671);
    }
}
