use icc::IntCodeComputer;

#[aoc(day05, part1, original)]
pub fn original_5a(input: &str) -> String {
    let mut v: Vec<i32> = input
        .split(',')
        .map(|o| o.parse::<i32>().unwrap())
        .collect();
    compute_5a(&mut v)
}

fn compute_5a(v: &mut Vec<i32>) -> String {
    let mut pc: usize = 0;
    const INPUT: i32 = 1;
    let mut output: String = "".to_string();
    loop {
        let s = format!("{:0>5}", v[pc].to_string());
        let mut c = s.chars();
        let _p2 = c.next();
        let p1 = match c.next().unwrap() {
            '0' => ParameterMode::Position,
            '1' => ParameterMode::Immediate,
            _ => unreachable!(),
        };
        let p0 = match c.next().unwrap() {
            '0' => ParameterMode::Position,
            '1' => ParameterMode::Immediate,
            _ => unreachable!(),
        };
        let operation = c.take(2).collect::<String>().parse::<i32>().unwrap();
        match operation {
            1 => add(v, &mut pc, p0, p1),
            2 => mul(v, &mut pc, p0, p1),
            3 => store(v, INPUT, &mut pc, p0),
            4 => output.push_str(&show(v, &mut pc, p0)),
            99 => break,
            _ => panic!(),
        }
    }
    output
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

fn add(v: &mut Vec<i32>, pc: &mut usize, p0: ParameterMode, p1: ParameterMode) {
    let ia0 = v[*pc + 1];
    let ia1 = v[*pc + 2];
    let oa = v[*pc + 3];
    let a0 = match p0 {
        ParameterMode::Position => v[ia0 as usize],
        ParameterMode::Immediate => ia0,
    };
    let a1 = match p1 {
        ParameterMode::Position => v[ia1 as usize],
        ParameterMode::Immediate => ia1,
    };
    v[oa as usize] = a0 + a1;
    *pc += 4;
}

fn mul(v: &mut Vec<i32>, pc: &mut usize, p0: ParameterMode, p1: ParameterMode) {
    let ia0 = v[*pc + 1];
    let ia1 = v[*pc + 2];
    let oa = v[*pc + 3];
    let a0 = match p0 {
        ParameterMode::Position => v[ia0 as usize],
        ParameterMode::Immediate => ia0,
    };
    let a1 = match p1 {
        ParameterMode::Position => v[ia1 as usize],
        ParameterMode::Immediate => ia1,
    };
    v[oa as usize] = a0 * a1;
    *pc += 4;
}

fn store(v: &mut Vec<i32>, input: i32, pc: &mut usize, p0: ParameterMode) {
    let is0 = v[*pc + 1];
    let _s0 = match p0 {
        ParameterMode::Position => v[is0 as usize],
        ParameterMode::Immediate => is0,
    };
    v[is0 as usize] = input;
    *pc += 2;
}

fn show(v: &mut Vec<i32>, pc: &mut usize, p0: ParameterMode) -> String {
    let is0 = v[*pc + 1];
    let s0 = match p0 {
        ParameterMode::Position => v[is0 as usize],
        ParameterMode::Immediate => is0,
    };
    *pc += 2;
    s0.to_string()
}

fn operations_5_to_8(
    v: &mut Vec<i32>,
    pc: &mut usize,
    p0: ParameterMode,
    p1: ParameterMode,
    o: Operation,
) {
    let ip0 = v[*pc + 1];
    let ip1 = v[*pc + 2];
    let oa = v[*pc + 3];
    let o0 = match p0 {
        ParameterMode::Position => v[ip0 as usize],
        ParameterMode::Immediate => ip0,
    };
    let o1 = match p1 {
        ParameterMode::Position => v[ip1 as usize],
        ParameterMode::Immediate => ip1,
    };
    if o == Operation::JumpIfTrue {
        if o0 != 0 {
            *pc = o1 as usize;
        } else {
            *pc += 3;
        }
    } else if o == Operation::JumpIfFalse {
        if o0 == 0 {
            *pc = o1 as usize;
        } else {
            *pc += 3;
        }
    } else if o == Operation::LessThan {
        v[oa as usize] = if o0 < o1 { 1 } else { 0 };
        *pc += 4;
    } else if o == Operation::Equals {
        v[oa as usize] = if o0 == o1 { 1 } else { 0 };
        *pc += 4;
    }
}

#[aoc(day05, part2, original)]
pub fn original_5b(input: &str) -> String {
    let mut v: Vec<i32> = input
        .split(',')
        .map(|o| o.parse::<i32>().unwrap())
        .collect();
    compute_5b(&mut v)
}

#[derive(PartialEq, PartialOrd)]
enum Operation {
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
}

fn compute_5b(v: &mut Vec<i32>) -> String {
    let mut pc: usize = 0;
    const INPUT: i32 = 5;
    let mut output: String = "".to_string();
    loop {
        let s = format!("{:0>5}", v[pc].to_string());
        let mut c = s.chars();
        let _p2 = c.next();
        let p1 = match c.next().unwrap() {
            '0' => ParameterMode::Position,
            '1' => ParameterMode::Immediate,
            _ => unreachable!(),
        };
        let p0 = match c.next().unwrap() {
            '0' => ParameterMode::Position,
            '1' => ParameterMode::Immediate,
            _ => unreachable!(),
        };
        let operation = c.take(2).collect::<String>().parse::<i32>().unwrap();
        match operation {
            1 => add(v, &mut pc, p0, p1),
            2 => mul(v, &mut pc, p0, p1),
            3 => store(v, INPUT, &mut pc, p0),
            4 => output.push_str(&show(v, &mut pc, p0)),
            5 => operations_5_to_8(v, &mut pc, p0, p1, Operation::JumpIfTrue),
            6 => operations_5_to_8(v, &mut pc, p0, p1, Operation::JumpIfFalse),
            7 => operations_5_to_8(v, &mut pc, p0, p1, Operation::LessThan),
            8 => operations_5_to_8(v, &mut pc, p0, p1, Operation::Equals),
            99 => break,
            _ => panic!(),
        }
    }
    output
}

#[aoc(day05, part1, icc)]
pub fn icc_5a(input: &str) -> String {
    let v: Vec<i32> = input
        .split(',')
        .map(|o| o.parse::<i32>().unwrap())
        .collect();
    let mut icc = IntCodeComputer {
        program: v,
        pc: 0,
        input0: 1,
        input1: 0,
        output: 0,
        input0_read: false,
        terminated: false,
    };
    icc.execute().to_string()
}

#[aoc(day05, part2, icc)]
pub fn icc_5b(input: &str) -> String {
    let v: Vec<i32> = input
        .split(',')
        .map(|o| o.parse::<i32>().unwrap())
        .collect();
    let mut icc = IntCodeComputer {
        program: v,
        pc: 0,
        input0: 5,
        input1: 0,
        output: 0,
        input0_read: false,
        terminated: false,
    };
    icc.execute().to_string()
}

#[cfg(test)]
mod tests {
    use day05::icc_5a;
    use day05::icc_5b;
    use day05::original_5a;
    use day05::original_5b;
    use std::fs;
    const ANSWER_5A: &str = "16489636";
    const ANSWER_5B: &str = "9386583";

    #[test]
    fn original() {
        assert!(
            original_5a(&fs::read_to_string("input/2019/day5.txt").unwrap().trim())
                .ends_with(ANSWER_5A)
        );
        assert!(
            original_5b(&fs::read_to_string("input/2019/day5.txt").unwrap().trim())
                .ends_with(ANSWER_5B)
        );
    }

    #[test]
    fn icc() {
        assert!(
            icc_5a(&fs::read_to_string("input/2019/day5.txt").unwrap().trim()).ends_with(ANSWER_5A)
        );
        assert!(
            icc_5b(&fs::read_to_string("input/2019/day5.txt").unwrap().trim()).ends_with(ANSWER_5B)
        );
    }
}
