use aoc_traits::AdventOfCodeDay;

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<InputLine>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input.lines().enumerate.for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, c)| {});
        })
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {}

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {}
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

    // #[test]
    // fn solve_part1() {
    //     let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
    //     let input = Solver::parse_input(&file_content);
    //     assert_eq!(Solver::solve_part1(&input), 7579994664753);
    // }
    // #[test]
    // fn test_part2() {
    //     let file_content = fs::read_to_string("src/test_puzzle.txt").expect("unable to read file");
    //     let input = Solver::parse_input(&file_content);
    //     assert_eq!(Solver::solve_part2(&input), 11387);
    // }
    //
    // #[test]
    // fn solve_part2() {
    //     let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
    //     let input = Solver::parse_input(&file_content);
    //     assert_eq!(Solver::solve_part2(&input), 438027111276610);
    // }
}
