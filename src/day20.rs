#[aoc(day20, part1)]
pub fn solution_20a(input: &str) -> u32 {
    let s: Vec<u32> = input
        .trim()
        .chars()
        .map(|n| n.to_string().parse::<u32>().unwrap())
        .collect();
    println!("{:?}", s);
    0
}

#[aoc(day20, part2)]
pub fn solution_20b(input: &str) -> u32 {
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
    use day20::solution_20a;
    use day20::solution_20b;
    use std::fs;
    const ANSWER_20A: u32 = 0;
    const ANSWER_20B: u32 = 0;

    #[test]
    fn t20a() {
        assert_eq!(
            ANSWER_20A,
            solution_20a(&fs::read_to_string("input/2019/day20.txt").unwrap().trim())
        );
    }
    #[test]
    fn t20b() {
        assert_eq!(
            ANSWER_20B,
            solution_20b(&fs::read_to_string("input/2019/day20.txt").unwrap().trim())
        );
    }
}
