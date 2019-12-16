#[aoc(day17, part1)]
pub fn solution_17a(input: &str) -> u32 {
    let s: Vec<u32> = input
        .trim()
        .chars()
        .map(|n| n.to_string().parse::<u32>().unwrap())
        .collect();
    println!("{:?}", s);
    0
}

#[aoc(day17, part2)]
pub fn solution_17b(input: &str) -> u32 {
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
    use day17::solution_17a;
    use day17::solution_17b;
    use std::fs;
    const ANSWER_17A: u32 = 0;
    const ANSWER_17B: u32 = 0;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_17A,
            solution_17a(&fs::read_to_string("input/2019/day17.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_17B,
            solution_17b(&fs::read_to_string("input/2019/day17.txt").unwrap().trim())
        );
    }
}
