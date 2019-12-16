#[aoc(day16, part1)]
pub fn solution_16a(input: &str) -> u32 {
    let s: Vec<u32> = input
        .trim()
        .chars()
        .map(|n| n.to_string().parse::<u32>().unwrap())
        .collect();
    println!("{:?}", s);
    0
}

#[aoc(day16, part2)]
pub fn solution_16b(input: &str) -> u32 {
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
    use day16::solution_16a;
    use day16::solution_16b;
    use std::fs;
    const ANSWER_16A: u32 = 0;
    const ANSWER_16B: u32 = 0;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_16A,
            solution_16a(&fs::read_to_string("input/2019/day16.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_16B,
            solution_16b(&fs::read_to_string("input/2019/day16.txt").unwrap().trim())
        );
    }
}
