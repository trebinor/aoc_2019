#[aoc(day22, part1)]
pub fn solution_22a(input: &str) -> u32 {
    let s: Vec<u32> = input
        .trim()
        .chars()
        .map(|n| n.to_string().parse::<u32>().unwrap())
        .collect();
    println!("{:?}", s);
    0
}

#[aoc(day22, part2)]
pub fn solution_22b(input: &str) -> u32 {
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
    use day22::solution_22a;
    use day22::solution_22b;
    use std::fs;
    const ANSWER_22A: u32 = 0;
    const ANSWER_22B: u32 = 0;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_22A,
            solution_22a(&fs::read_to_string("input/2019/day22.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_22B,
            solution_22b(&fs::read_to_string("input/2019/day22.txt").unwrap().trim())
        );
    }
}
