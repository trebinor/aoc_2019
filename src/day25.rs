use icc::IntCodeComputer;

#[aoc(day25, part1)]
pub fn solution_25a(input: &str) -> u32 {
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

#[aoc(day25, part2)]
pub fn solution_25b(input: &str) -> u32 {
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
    use day25::solution_25a;
    use day25::solution_25b;
    use std::fs;
    const ANSWER_25A: u32 = 0;
    const ANSWER_25B: u32 = 0;

    #[test]
    fn t25a() {
        assert_eq!(
            ANSWER_25A,
            solution_25a(&fs::read_to_string("input/2019/day25.txt").unwrap().trim())
        );
    }
    #[test]
    fn t25b() {
        assert_eq!(
            ANSWER_25B,
            solution_25b(&fs::read_to_string("input/2019/day25.txt").unwrap().trim())
        );
    }
}
