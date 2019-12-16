#[aoc(day21, part1)]
pub fn solution_21a(input: &str) -> u32 {
    let s: Vec<u32> = input
        .trim()
        .chars()
        .map(|n| n.to_string().parse::<u32>().unwrap())
        .collect();
    println!("{:?}", s);
    0
}

#[aoc(day21, part2)]
pub fn solution_21b(input: &str) -> u32 {
    let s: Vec<u32> = input
        .trim()
        .chars()
        .map(|n| n.to_string().parse::<u32>().unwrap())
        .collect();
    println!("{:?}", s);
    0
}

#[cfg(test)]
mod tests {
    use day21::solution_21a;
    use day21::solution_21b;
    use std::fs;
    const ANSWER_21A: u32 = 0;
    const ANSWER_21B: u32 = 0;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_21A,
            solution_21a(&fs::read_to_string("input/2019/day21.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_21B,
            solution_21b(&fs::read_to_string("input/2019/day21.txt").unwrap().trim())
        );
    }
}
