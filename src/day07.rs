use icc::IntCodeComputer;
use regex::Regex;

#[aoc(day07, part1, original)]
pub fn original_7a(input: &str) -> i64 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let phases = (1234..=43210)
        .filter(|p| phase_sequence_allowed(*p))
        .map(|p| format!("{:0>5}", p.to_string()))
        .collect::<Vec<String>>();
    let mut max_output: i64 = std::i64::MIN;
    for p in phases {
        let mut amps: Vec<IntCodeComputer> = vec![
            IntCodeComputer {
                program: v.clone(),
                pc: 0,
                input: 0,
                amp_input: 0,
                use_amp_input: true,
                input_read: false,
                break_on_output: false,
                terminated: false,
                relative_base: 0,
                output: "".to_string(),
                previous_operation: 0,
            };
            5
        ];
        let d = p.chars().flat_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
        for i in 0..=4 {
            amps[i].input = d[i] as i64;
            amps[i].execute();
            let output = amps[i].consume_output().parse::<i64>().unwrap();
            if i < 4 {
                amps[i + 1].amp_input = output;
            } else {
                max_output = std::cmp::max(output, max_output);
            }
        }
    }
    max_output
}

fn phase_sequence_allowed(phases: i64) -> bool {
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
fn phase_sequence_allowed_high(phases: i64) -> bool {
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
pub fn original_7b(input: &str) -> i64 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let phases = (56789..=98765)
        .filter(|p| phase_sequence_allowed_high(*p))
        .map(|p| format!("{:0>5}", p.to_string()))
        .collect::<Vec<String>>();
    let mut max_output: i64 = std::i64::MIN;
    for p in phases {
        let mut amps: Vec<IntCodeComputer> = vec![
            IntCodeComputer {
                program: v.clone(),
                pc: 0,
                input: 0,
                amp_input: 0,
                use_amp_input: true,
                input_read: false,
                break_on_output: true,
                terminated: false,
                relative_base: 0,
                output: "".to_string(),
                previous_operation: 0,
            };
            5
        ];
        'this_phase: loop {
            let d = p.chars().flat_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
            for i in 0..=4 {
                amps[i].input = d[i] as i64;
                amps[i].execute();
                if i < 4 {
                    if amps[i].terminated {
                        continue;
                    }
                } else if amps[4].terminated {
                    break 'this_phase;
                }
                let output = amps[i].consume_output().parse::<i64>().unwrap();
                if i < 4 {
                    amps[i + 1].amp_input = output;
                } else {
                    max_output = std::cmp::max(output, max_output);
                    if amps[4].terminated {
                        break 'this_phase;
                    } else {
                        amps[0].amp_input = output;
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
    const ANSWER_7A: i64 = 437860;
    const ANSWER_7B: i64 = 49810599;

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
