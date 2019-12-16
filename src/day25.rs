#[aoc(day25, part1)]
pub fn solution_25a(input: &str) -> u32 {
    let s: Vec<u32> = input
        .trim()
        .chars()
        .map(|n| n.to_string().parse::<u32>().unwrap())
        .collect();
    println!("{:?}", s);
    0
}

#[aoc(day25, part2)]
pub fn solution_25b(input: &str) -> u32 {
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
    use day25::solution_25a;
    use day25::solution_25b;
    use std::fs;
    const ANSWER_25A: u32 = 0;
    const ANSWER_25B: u32 = 0;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_25A,
            solution_25a(&fs::read_to_string("input/2019/day25.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_25B,
            solution_25b(&fs::read_to_string("input/2019/day25.txt").unwrap().trim())
        );
    }
}
