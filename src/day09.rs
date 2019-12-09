use icc::IntCodeComputer;

#[aoc(day09, part1, original)]
pub fn original_9a(input: &str) -> i64 {
    let v: Vec<i64> = input
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut icc = IntCodeComputer {
        program: v.clone(),
        pc: 0,
        input0: 1,
        input1: 0,
        output: 0,
        input0_read: false,
        terminated: false,
        relative_base: 0,
        output_string: "".to_string(),
    };
    icc.program.resize(1024 * 1024, 0);
    icc.execute();
    icc.show_output().parse().unwrap()
}

#[aoc(day09, part2, original)]
pub fn original_9b(input: &str) -> i64 {
    let v: Vec<i64> = input
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut icc = IntCodeComputer {
        program: v.clone(),
        pc: 0,
        input0: 2,
        input1: 0,
        output: 0,
        input0_read: false,
        terminated: false,
        relative_base: 0,
        output_string: "".to_string(),
    };
    icc.program.resize(1024 * 1024, 0);
    icc.execute();
    icc.show_output().parse().unwrap()
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
