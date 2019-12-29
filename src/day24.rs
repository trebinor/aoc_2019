#[aoc(day24, part1)]
pub fn solution_24a(input: &str) -> u32 {
    let s: Vec<u32> = input
        .trim()
        .chars()
        .map(|n| n.to_string().parse::<u32>().unwrap())
        .collect();
    println!("{:?}", s);
    0
}

#[aoc(day24, part2)]
pub fn solution_24b(input: &str) -> u32 {
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
    use day24::solution_24a;
    use day24::solution_24b;
    use std::fs;
    const ANSWER_24A: u32 = 0;
    const ANSWER_24B: u32 = 0;

    #[test]
    fn t24a() {
        assert_eq!(
            ANSWER_24A,
            solution_24a(&fs::read_to_string("input/2019/day24.txt").unwrap().trim())
        );
    }
    #[test]
    fn t24b() {
        assert_eq!(
            ANSWER_24B,
            solution_24b(&fs::read_to_string("input/2019/day24.txt").unwrap().trim())
        );
    }
}
