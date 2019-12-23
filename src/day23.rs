use icc::IntCodeComputer;
use std::collections::VecDeque;

#[aoc(day23, part1)]
pub fn solution_23a(input: &str) -> i64 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut comps: Vec<IntCodeComputer> = vec![
        IntCodeComputer {
            program: v.clone(),
            pc: 0,
            input: 0,
            amp_input: 0,
            use_amp_input: false,
            input_read: false,
            break_on_input: false,
            break_on_output: true,
            terminated: false,
            relative_base: 0,
            output: "".to_string(),
            previous_operation: 0,
            inputq: VecDeque::new(),
        };
        50
    ];
    let mut inputq: Vec<VecDeque<i64>> = vec![VecDeque::new(); 50];
    let mut outputq: Vec<VecDeque<i64>> = vec![VecDeque::new(); 50];
    for (i, c) in comps.iter_mut().enumerate() {
        c.program.resize(1024 * 1024, 0);
        c.input = i as i64;
        c.execute_one();
        c.input = -1;
        assert_eq!(c.previous_operation, 3);
    }

    let mut y_value_to_addr_255 = None;
    'outer: loop {
        for (i, c) in comps.iter_mut().enumerate() {
            c.execute_one();
            if c.previous_operation == 4 {
                outputq[i].push_back(c.consume_output().parse::<i64>().unwrap());

                if outputq[i].len() >= 3 {
                    // produce to input queues from this NIC
                    let addr = outputq[i].pop_front().unwrap();
                    let x = outputq[i].pop_front().unwrap();
                    let y = outputq[i].pop_front().unwrap();
                    println!("Packet ({},{}) to {} from {}", x, y, addr, i);
                    if addr == 255 {
                        println!("Breaking");
                        y_value_to_addr_255 = Some(y);
                        break 'outer;
                    }
                    inputq[addr as usize].push_back(x);
                    inputq[addr as usize].push_back(y);
                }
            }
            // consume this NIC's input queue
            if inputq[i].is_empty() {
                c.input = -1;
            } else {
                c.input = inputq[i].pop_front().unwrap() as i64;
                c.execute_one();
                while c.previous_operation != 3 {
                    c.execute_one();
                }
                c.input = inputq[i].pop_front().unwrap() as i64;
                c.execute_one();
                while c.previous_operation != 3 {
                    c.execute_one();
                }
            }
        }
    }
    y_value_to_addr_255.unwrap()
}

pub struct NAT {
    pub x: Option<i64>,
    pub y: Option<i64>,
}
#[aoc(day23, part2)]
pub fn solution_23b(input: &str) -> i64 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut nat = NAT { x: None, y: None };
    let mut last_nat_to_0_y: Option<i64> = None;
    let mut comps: Vec<IntCodeComputer> = vec![
        IntCodeComputer {
            program: v.clone(),
            pc: 0,
            input: 0,
            amp_input: 0,
            use_amp_input: false,
            input_read: false,
            break_on_output: true,
            break_on_input: false,
            terminated: false,
            relative_base: 0,
            output: "".to_string(),
            previous_operation: 0,
            inputq: VecDeque::new(),
        };
        50
    ];

    let mut idle: Vec<bool> = vec![false; 50];
    for (i, c) in comps.iter_mut().enumerate() {
        c.program.resize(1024 * 1024, 0);
        c.inputq.push_back(i as i64);
        c.execute_one();
        assert_eq!(c.previous_operation, 3);
    }

    'outer: loop {
        for i in 0..comps.len() {
            loop {
                let mut input_or_output = false;
                comps[i].execute_n_breaking(1000);
                if !comps[i].output.is_empty() {
                    idle[i] = false;
                    assert_eq!(4, comps[i].previous_operation);
                    input_or_output = true;
                    let addr = comps[i].consume_output().parse::<i64>().unwrap();
                    comps[i].execute();
                    assert_eq!(4, comps[i].previous_operation);
                    let x = comps[i].consume_output().parse::<i64>().unwrap();
                    comps[i].execute();
                    assert_eq!(4, comps[i].previous_operation);
                    let y = comps[i].consume_output().parse::<i64>().unwrap();

                    //println!("Packet ({},{}) to {} from {}", x, y, addr, i);
                    if addr == 255 {
                        nat.x = Some(x);
                        nat.y = Some(y);
                    //println!("NAT ({},{})", x, y);
                    } else {
                        comps[addr as usize].inputq.push_back(x);
                        comps[addr as usize].inputq.push_back(y);
                    }
                }
                // consume this NIC's input queue
                if !comps[i].inputq.is_empty() {
                    input_or_output = true;
                }
                if !input_or_output {
                    idle[i] = true;
                    break;
                }
            }
        }
        // Are all input and output queues empty?
        if comps.iter().all(|c| c.inputq.is_empty()) && idle.iter().all(|&i| i) {
            assert!(nat.x.is_some());
            assert!(nat.y.is_some());
            comps[0].inputq.push_back(nat.x.unwrap());
            comps[0].inputq.push_back(nat.y.unwrap());
            idle.iter_mut().for_each(|i| *i = false);
            if last_nat_to_0_y.is_some() && last_nat_to_0_y == nat.y {
                break 'outer;
            }
            last_nat_to_0_y = nat.y;
        }
    }

    nat.y.unwrap()
}

#[cfg(test)]
mod tests {
    use day23::solution_23a;
    use day23::solution_23b;
    use std::fs;
    const ANSWER_23A: i64 = 22659;
    const ANSWER_23B: i64 = 17429;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_23A,
            solution_23a(&fs::read_to_string("input/2019/day23.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_23B,
            solution_23b(&fs::read_to_string("input/2019/day23.txt").unwrap().trim())
        );
    }
}
