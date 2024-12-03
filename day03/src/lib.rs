use aoc_traits::AdventOfCodeDay;
use regex::Regex;

#[derive(Debug)]
pub struct Data {
    left: Option<u32>,
    right: Option<u32>,
    is_do: bool,
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<Data>;
    type Part1Output = u32;
    type Part2Output = u32;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let re = Regex::new(r"mul[(](?<left>\d+)[,](?<right>\d+)[)]|do(n't)?").unwrap();
        re.captures_iter(input)
            .map(|x| match x.name("left") {
                Some(_) => Data {
                    left: Some(x.name("left").unwrap().as_str().parse().unwrap()),
                    right: Some(x.name("right").unwrap().as_str().parse().unwrap()),
                    is_do: false,
                },
                None => Data {
                    left: None,
                    right: None,
                    is_do: match x.get(3) {
                        Some(_) => false,
                        None => true,
                    },
                },
            })
            .collect()
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input
            .iter()
            .filter(|data| data.left != None)
            .map(|data| data.left.unwrap() * data.right.unwrap())
            .sum()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let mut do_mult = true;
        let mut sum = 0;
        for element in input {
            match element.left {
                x if x.is_some() && do_mult == true => {
                    sum += element.left.unwrap() * element.right.unwrap()
                }
                None => do_mult = element.is_do,
                _ => {}
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let file_content = fs::read_to_string("src/test_puzzle1.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 161);
    }

    #[test]
    fn test_part2() {
        let file_content = fs::read_to_string("src/test_puzzle2.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 48);
    }

    #[test]
    fn solve_part1() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part1(&input), 174336360);
    }

    #[test]
    fn solve_part2() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        let input = Solver::parse_input(&file_content);
        assert_eq!(Solver::solve_part2(&input), 88802350);
    }
}
