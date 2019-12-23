use icc::IntCodeComputer;
use std::collections::VecDeque;

#[aoc(day09, part1, original)]
pub fn original_9a(input: &str) -> i64 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut icc = IntCodeComputer {
        program: v.clone(),
        pc: 0,
        input: 1,
        amp_input: 0,
        use_amp_input: false,
        input_read: false,
        break_on_input: false,
        break_on_output: false,
        terminated: false,
        relative_base: 0,
        output: "".to_string(),
        previous_operation: 0,
        inputq: VecDeque::new(),
    };
    icc.program.resize(1024 * 1024, 0);
    icc.execute();
    icc.consume_output().parse().unwrap()
}

#[aoc(day09, part2, original)]
pub fn original_9b(input: &str) -> i64 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut icc = IntCodeComputer {
        program: v.clone(),
        pc: 0,
        input: 2,
        amp_input: 0,
        use_amp_input: false,
        input_read: false,
        break_on_input: false,
        break_on_output: false,
        terminated: false,
        relative_base: 0,
        output: "".to_string(),
        previous_operation: 0,
        inputq: VecDeque::new(),
    };
    icc.program.resize(1024 * 1024, 0);
    icc.execute();
    icc.consume_output().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use day09::original_9a;
    use day09::original_9b;
    use std::fs;
    const ANSWER_9A: i64 = 2932210790;
    const ANSWER_9B: i64 = 73144;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_9A,
            original_9a(&fs::read_to_string("input/2019/day9.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_9B,
            original_9b(&fs::read_to_string("input/2019/day9.txt").unwrap().trim())
        );
    }
}
