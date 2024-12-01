use std::collections::HashMap;

pub fn part1(input: &str) -> i32 {
    let mut left_side: Vec<i32> = Vec::new();
    let mut right_side: Vec<i32> = Vec::new();
    let mut sum = 0;
    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        left_side.push(parts.next().unwrap().parse().expect(".."));
        right_side.push(parts.next().unwrap().parse().expect(".."));
    });

    left_side.sort_unstable();
    right_side.sort_unstable();

    assert_eq!(left_side.len(), right_side.len());

    for i in 0..left_side.len() {
        let diff = left_side[i] - right_side[i];
        sum += diff.abs();
    }
    return sum;
}

pub fn part2(input: &str) -> i32 {
    let mut left_side = HashMap::new();
    let mut right_side = HashMap::new();
    let mut sum = 0;
    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        let left: i32 = parts.next().unwrap().parse().expect("..");
        let right: i32 = parts.next().unwrap().parse().expect("..");
        left_side.entry(left).and_modify(|x| *x += 1).or_insert(1);
        right_side.entry(right).and_modify(|x| *x += 1).or_insert(1);
    });

    for (key, value) in left_side.into_iter() {
        match right_side.get(&key) {
            Some(x) => sum += (key * x) * value,
            None => {}
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let input = " 3   4
                4   3
                2   5
                1   3
                3   9
                3   3";
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_part2() {
        let input = " 3   4
                4   3
                2   5
                1   3
                3   9
                3   3";
        assert_eq!(part2(input), 31);
    }

    #[test]
    fn solve_part1() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        assert_eq!(part1(&file_content), 2904518);
    }

    #[test]
    fn solve_part2() {
        let file_content = fs::read_to_string("src/puzzle.txt").expect("unable to read file");
        assert_eq!(part2(&file_content), 18650129);
    }
}
