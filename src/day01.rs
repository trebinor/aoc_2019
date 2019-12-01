#[aoc(day01, part1)]
pub fn solve_1a(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.trim().parse::<i32>().unwrap())
        .sum()
}

#[aoc(day01, part2)]
pub fn solve_1b(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.trim().parse::<i32>().unwrap())
        .sum()
}
