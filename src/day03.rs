#[aoc(day03, part1, original)]
pub fn original_3a(input: &str) -> u32 {
    let mut v: Vec<u32> = input
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    v[1] = 12;
    v[2] = 2;
    compute(&mut v)
}

fn compute(mut v: &mut Vec<u32>) -> u32 {
    let mut i: usize = 0;
    loop {
        match v[i] {
            1 => do_opcode_1(&mut v, i),
            2 => do_opcode_2(&mut v, i),
            99 => break,
            _ => panic!(),
        }
        i += 4;
    }
    v[0]
}

fn do_opcode_1(v: &mut Vec<u32>, i: usize) {
    let ia1: usize = v[i + 1] as usize;
    let ia2: usize = v[i + 2] as usize;
    let oa: usize = v[i + 3] as usize;
    v[oa] = v[ia1] + v[ia2];
}
fn do_opcode_2(v: &mut Vec<u32>, i: usize) {
    let ia1: usize = v[i + 1] as usize;
    let ia2: usize = v[i + 2] as usize;
    let oa: usize = v[i + 3] as usize;
    v[oa] = v[ia1] * v[ia2];
}

const PART2_EXPECTED_OUTPUT: u32 = 19690720;
#[aoc(day03, part2, original)]
pub fn original_3b(input: &str) -> u32 {
    let v: Vec<u32> = input
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    for noun in 0..99 {
        for verb in 0..99 {
            let mut v_cloned = v.clone();
            v_cloned[1] = noun;
            v_cloned[2] = verb;
            if compute(&mut v_cloned) == PART2_EXPECTED_OUTPUT {
                return 100 * noun + verb;
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use day03::original_3a;
    use day03::original_3b;
    use std::fs;
    const ANSWER_3A: u32 = 5866714;
    const ANSWER_3B: u32 = 5208;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_3A,
            original_3a(&fs::read_to_string("input/2019/day3.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_3B,
            original_3b(&fs::read_to_string("input/2019/day3.txt").unwrap().trim())
        );
    }
}
