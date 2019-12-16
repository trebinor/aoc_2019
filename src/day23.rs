#[aoc(day23, part1)]
pub fn solution_23a(input: &str) -> u32 {
    let s: Vec<u32> = input
        .trim()
        .chars()
        .map(|n| n.to_string().parse::<u32>().unwrap())
        .collect();
    println!("{:?}", s);
    0
}

#[aoc(day23, part2)]
pub fn solution_23b(input: &str) -> u32 {
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
    use day23::solution_23a;
    use day23::solution_23b;
    use std::fs;
    const ANSWER_23A: u32 = 0;
    const ANSWER_23B: u32 = 0;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_23A,
            solution_23a(&fs::read_to_string("input/2019/day23.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_23B,
            solution_23b(&fs::read_to_string("input/2019/day23.txt").unwrap().trim())
        );
    }
}
