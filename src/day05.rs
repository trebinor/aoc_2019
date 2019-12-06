#[aoc(day05, part1, original)]
pub fn original_5a(input: &str) -> String {
    let mut v: Vec<i32> = input
        .split(',')
        .map(|o| o.parse::<i32>().unwrap())
        .collect();
    //v[1] = 12;
    //v[2] = 2;
    //v.resize(32 * 1024 * 1024, 0);
    compute(&mut v)
}

fn compute(mut v: &mut Vec<i32>) -> String {
    let mut pc: usize = 0;
    const INPUT: i32 = 1;
    let mut output: String = "".to_string();
    loop {
        let s = format!("{:0>5}", v[pc].to_string());
        println!("{}", s);
        let mut c = s.chars();
        let p2 = c.next();
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
        println!("p1={:?} p0={:?}", p1, p0);
        println!("pc = {}", pc);
        match operation {
            1 => add(v, &mut pc, p0, p1),
            2 => mul(v, &mut pc, p0, p1),
            3 => store(v, INPUT, &mut pc, p0),
            4 => output.push_str(&show(v, &mut pc, p0)),
            99 => break,
            _ => panic!(),
        }
        println!("output is {}", output);
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
    println!("p1={:?} p0={:?}", p1, p0);
    println!("ia0 = {}, ia1 = {}, oa = {}", ia0, ia1, oa);
    let a0 = match p0 {
        ParameterMode::Position => v[ia0 as usize],
        ParameterMode::Immediate => ia0,
    };
    println!("ia0 = {}, ia1 = {}, oa = {}", ia0, ia1, oa);
    let a1 = match p1 {
        ParameterMode::Position => v[ia1 as usize],
        ParameterMode::Immediate => ia1,
    };
    println!("Wrote {} + {} = {} to {} from add", a0, a1, a0 + a1, oa);
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
    println!("Wrote {} to {} from mul", a0 * a1, oa);
    *pc += 4;
}

fn store(v: &mut Vec<i32>, input: i32, pc: &mut usize, p0: ParameterMode) {
    let is0 = v[*pc + 1];
    let s0 = match p0 {
        ParameterMode::Position => v[is0 as usize],
        ParameterMode::Immediate => is0,
    };
    println!("Wrote {} to {} from store", input, is0);
    v[is0 as usize] = input;
    /*
    let ia1: usize = v[i + 1] as usize;
    let ia2: usize = v[i + 2] as usize;
    let oa: usize = v[i + 3] as usize;
    v[oa] = v[ia1] * v[ia2];
    */
    *pc += 2;
}
fn show(v: &mut Vec<i32>, pc: &mut usize, p0: ParameterMode) -> String {
    let is0 = v[*pc + 1];
    let s0 = match p0 {
        ParameterMode::Position => v[is0 as usize],
        ParameterMode::Immediate => is0,
    };
    *pc += 2;
    println!("show {}", s0);
    s0.to_string()
    /*
    let ia1: usize = v[i + 1] as usize;
    let ia2: usize = v[i + 2] as usize;
    let oa: usize = v[i + 3] as usize;
    v[oa] = v[ia1] * v[ia2];
    */
}
//fn immediate(v: &mut Vec<&str>

#[aoc(day05, part2, original)]
pub fn original_5b(_input: &str) -> u32 {
    /*
    let min = 278384;
    let max = 824795;
    let mut count = 0;
    for i in min..=max {
        if is_a_double_digit_strict(i) && is_monotonically_increasing(i) {
            count += 1;
        }
    }
    count
    */
    0
}

#[cfg(test)]
mod tests {
    use day04::original_5a;
    use day04::original_5b;
    use std::fs;
    const ANSWER_5A: u32 = 921;
    const ANSWER_5B: u32 = 603;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_5A,
            original_5a(&fs::read_to_string("input/2019/day5.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_5B,
            original_5b(&fs::read_to_string("input/2019/day5.txt").unwrap().trim())
        );
    }
}
