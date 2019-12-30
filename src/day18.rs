//use petgraph::algo::astar;
//use petgraph::Graph;

#[aoc(day18, part1)]
pub fn solution_18a(input: &str) -> u32 {
    let s: Vec<u32> = input
        .trim()
        .chars()
        .map(|n| n.to_string().parse::<u32>().unwrap())
        .collect();
    println!("{:?}", s);
    0
}

#[aoc(day18, part2)]
pub fn solution_18b(input: &str) -> u32 {
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
    use day18::solution_18a;
    use day18::solution_18b;
    use std::fs;
    const ANSWER_18A: u32 = 0;
    const ANSWER_18B: u32 = 0;

    #[test]
    fn t18a() {
        assert_eq!(
            ANSWER_18A,
            solution_18a(&fs::read_to_string("input/2019/day18.txt").unwrap().trim())
        );
    }
    #[test]
    fn t18b() {
        assert_eq!(
            ANSWER_18B,
            solution_18b(&fs::read_to_string("input/2019/day18.txt").unwrap().trim())
        );
    }
}
