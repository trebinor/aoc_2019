use std::cmp;

#[aoc(day10, part1, original)]
pub fn original_10a(input: &str) -> i32 {
    let v: Vec<&str> =
    input
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<&str>>();
        0
}

#[aoc(day10, part2, original)]
pub fn original_10b(input: &str) -> i32 {
    let v: Vec<&str> =
    input
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<&str>>();
        0
}

#[cfg(test)]
mod tests {
    use day10::original_10a;
    use day10::original_10b;
    use std::fs;
    const ANSWER_10A: i32 = 3369286;
    const ANSWER_10B: i32 = 5051054;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_10A,
            original_10a(&fs::read_to_string("input/2019/day10.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_10B,
            original_10b(&fs::read_to_string("input/2019/day10.txt").unwrap().trim())
        );
    }
}
