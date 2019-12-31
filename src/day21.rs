use icc::IntCodeComputer;

#[aoc(day21, part1)]
pub fn solution_21a(input: &str) -> u32 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut icc = IntCodeComputer::new(v, false);
    icc.program.resize(1024 * 1024, 0);
    // set inputs
    icc.execute();
    let _output = icc.consume_output();
    0
}

#[aoc(day21, part2)]
pub fn solution_21b(input: &str) -> u32 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut icc = IntCodeComputer::new(v, false);
    icc.program.resize(1024 * 1024, 0);
    // set inputs
    icc.execute();
    let _output = icc.consume_output();
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
    fn t21a() {
        assert_eq!(
            ANSWER_21A,
            solution_21a(&fs::read_to_string("input/2019/day21.txt").unwrap().trim())
        );
    }
    #[test]
    fn t21b() {
        assert_eq!(
            ANSWER_21B,
            solution_21b(&fs::read_to_string("input/2019/day21.txt").unwrap().trim())
        );
    }
}
