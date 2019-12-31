use icc::IntCodeComputer;

#[aoc(day19, part1)]
pub fn solution_19a(input: &str) -> u32 {
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

#[aoc(day19, part2)]
pub fn solution_19b(input: &str) -> u32 {
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
    use day19::solution_19a;
    use day19::solution_19b;
    use std::fs;
    const ANSWER_19A: u32 = 0;
    const ANSWER_19B: u32 = 0;

    #[test]
    fn t19a() {
        assert_eq!(
            ANSWER_19A,
            solution_19a(&fs::read_to_string("input/2019/day19.txt").unwrap().trim())
        );
    }
    #[test]
    fn t19b() {
        assert_eq!(
            ANSWER_19B,
            solution_19b(&fs::read_to_string("input/2019/day19.txt").unwrap().trim())
        );
    }
}
