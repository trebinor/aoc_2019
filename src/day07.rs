use icc::IntCodeComputer;
use regex::Regex;
//use itertools::Itertools;
//use std::collections::HashMap;

#[aoc(day07, part1, original)]
pub fn original_7a(input: &str) -> i32 {
    let v: Vec<i32> = input
        .split(',')
        .map(|o| o.parse::<i32>().unwrap())
        .collect();
    let phases = (1234..=43210)
        .filter(|p| phase_sequence_allowed(*p))
        .map(|p| format!("{:0>5}", p.to_string()))
        .collect::<Vec<String>>();
    let mut max_output: i32 = std::i32::MIN;
    for p in phases {
        let mut amps: Vec<IntCodeComputer> = vec![
            IntCodeComputer {
                program: v.clone(),
                pc: 0,
                input0: 0,
                input1: 0,
                output: 0,
                input0_read: false,
                terminated: false
            };
            5
        ];
        let d = p.chars().flat_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
        for i in 0..=4 {
            amps[i].input0 = d[i] as i32;
            let output = amps[i].execute();
            if i < 4 {
                amps[i + 1].input1 = output;
            } else {
                max_output = std::cmp::max(output, max_output);
            }
        }
    }
    max_output
}

fn phase_sequence_allowed(phases: i32) -> bool {
    let v = format!("{:0>5}", phases.to_string());
    let re = Regex::new(r"[5-9]+").unwrap();
    let mut allowed = true;
    'outer: for x in 0..=4 {
        for y in 0..=4 {
            if x != y && (re.is_match(&v) || (v.chars().nth(x) == v.chars().nth(y))) {
                allowed = false;
                break 'outer;
            }
        }
    }
    allowed
}
fn phase_sequence_allowed_high(phases: i32) -> bool {
    let v = format!("{:0>5}", phases.to_string());
    let re = Regex::new(r"[0-4]+").unwrap();
    let mut allowed = true;
    'outer: for x in 0..=4 {
        for y in 0..=4 {
            if x != y && (re.is_match(&v) || (v.chars().nth(x) == v.chars().nth(y))) {
                allowed = false;
                break 'outer;
            }
        }
    }
    allowed
}

#[aoc(day07, part2, original)]
pub fn original_7b(input: &str) -> i32 {
    let v: Vec<i32> = input
        .split(',')
        .map(|o| o.parse::<i32>().unwrap())
        .collect();
    let phases = (56789..=98765)
        .filter(|p| phase_sequence_allowed_high(*p))
        .map(|p| format!("{:0>5}", p.to_string()))
        .collect::<Vec<String>>();
    let mut max_output: i32 = std::i32::MIN;
    for p in phases {
        println!("Trying phase {}", p);
        let mut amps: Vec<IntCodeComputer> = vec![
            IntCodeComputer {
                program: v.clone(),
                pc: 0,
                input0: 0,
                input1: 0,
                output: 0,
                input0_read: false,
                terminated: false
            };
            5
        ];
        'this_phase: loop {
            let d = p.chars().flat_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
            for i in 0..=4 {
                amps[i].input0 = d[i] as i32;
                let output = amps[i].execute();
                println!("output[{}]={}", i, output);
                if i < 4 {
                    amps[i + 1].input1 = output;
                } else {
                    max_output = std::cmp::max(output, max_output);
                    if amps[4].terminated {
                        break 'this_phase;
                    } else {
                        amps[0].input1 = output;
                        //amps[0].input0_read = false;
                    }
                }
            }
        }
    }
    max_output
}

#[cfg(test)]
mod tests {
    use day07::original_7a;
    use day07::original_7b;
    use std::fs;
    const ANSWER_7A: i32 = 437860;
    const ANSWER_7B: i32 = 49810599;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_7A,
            original_7a(&fs::read_to_string("input/2019/day7.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_7B,
            original_7b(&fs::read_to_string("input/2019/day7.txt").unwrap().trim())
        );
    }
}
