use std::usize;

use aoc_traits::AdventOfCodeDay;

#[derive(Debug)]
pub struct INPUTS {
    rules: Vec<Vec<usize>>,
    pages: Vec<Vec<usize>>,
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = INPUTS;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let mut lines = input.lines().into_iter();

        let mut current_line = lines.next().unwrap();

        let mut rules = vec![vec![]; 256];
        let mut to_check = Vec::new();
        while !current_line.is_empty() {
            let (left, right) = current_line.trim().split_once('|').unwrap();
            rules[right.parse::<usize>().unwrap()].push(left.parse::<usize>().unwrap());
            current_line = lines.next().unwrap();
        }
        current_line = lines.next().unwrap();

        while !current_line.is_empty() {
            let pages = current_line.split(',');
            let mut manual: Vec<usize> = Vec::new();
            pages
                .into_iter()
                .for_each(|x| manual.push(x.parse().unwrap()));
            to_check.push(manual);
            if let Some(i) = lines.next() {
                current_line = i;
            } else {
                break;
            }
        }

        INPUTS {
            rules,
            pages: to_check,
        }
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input
            .pages
            .iter()
            .filter_map(|x| {
                // let good = x.iter().all(|p| {
                //     dissallowed.extend(input.rules[*p].clone());
                //     !dissallowed.contains(p)
                // });

                let good = good_array(input, x).0;

                if good {
                    // dbg!(&x);
                    // dbg!(x[x.len() / 2]);
                    Some(x[x.len() / 2])
                } else {
                    None
                }
            })
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input
            .pages
            .iter()
            .filter_map(|x| {
                let (mut good, mut index) = good_array(input, x);
                if good {
                    return None;
                }
                let mut new_x = x.clone();
                while !good {
                    let tmp = new_x[index - 1];
                    new_x[index - 1] = new_x[index];
                    new_x[index] = tmp;
                    (good, index) = good_array(input, &new_x);
                }

                Some(new_x[x.len() / 2])
            })
            .sum()
    }
}

pub fn good_array(input: &INPUTS, current_line: &Vec<usize>) -> (bool, usize) {
    let mut dissallowed = Vec::new();
    let mut wrong_index = 0;
    current_line.iter().enumerate().all(|(index, p)| {
        dissallowed.extend(input.rules[*p].clone());
        if dissallowed.contains(p) {
            wrong_index = index;
            return false;
        }
        true
    });
    if wrong_index != 0 {
        return (false, wrong_index);
    }
    return (true, 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 143);
    }

    #[test]
    fn solve_part1() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 4790);
    }

    #[test]
    fn test_part2() {
        let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 123);
    }

    #[test]
    fn solve_part2() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 6319);
    }
}
