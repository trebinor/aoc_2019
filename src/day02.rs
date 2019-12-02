//use std::cmp;

#[aoc(day02, part1, original)]
pub fn original_2a(input: &str) -> u32 {
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
    let oi1: usize = v[i + 1] as usize;
    let oi2: usize = v[i + 2] as usize;
    let loc: usize = v[i + 3] as usize;
    v[loc] = v[oi1] + v[oi2];
}
fn do_opcode_2(v: &mut Vec<u32>, i: usize) {
    let oi1: usize = v[i + 1] as usize;
    let oi2: usize = v[i + 2] as usize;
    let loc: usize = v[i + 3] as usize;
    v[loc] = v[oi1] * v[oi2];
}

const PART2_EXPECTED_OUTPUT: u32 = 19690720;
#[aoc(day02, part2, original)]
pub fn original_1b(input: &str) -> u32 {
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
/*
#[aoc(day01, part2, original)]
pub fn original_1b(input: &str) -> i32 {
    let modules = input.lines().map(|l| l.trim().parse::<i32>().unwrap());
    let mut fuel: i32 = 0;
    for fuels in modules {
        let mut module_fuel = original_fuel_mass_compound(fuels);
        while module_fuel != 0 {
            fuel += module_fuel;
            module_fuel = original_fuel_mass_compound(module_fuel);
        }
    }
    fuel
}

fn revised_fuel_mass_simple(module: u32) -> u32 {
    (module / 3.0 as u32) - 2
}

fn original_fuel_mass_compound(module: i32) -> i32 {
    cmp::max(0, module / 3.0 as i32 - 2)
}

fn revised_fuel_mass_compound(mass: u32) -> u32 {
    match mass {
        0 => 0,
        _ => mass + revised_fuel_mass_compound(cmp::max(0, (mass as i32 / 3.0 as i32) - 2) as u32),
    }
}

#[aoc(day01, part1, revised)]
pub fn revised_1a(input: &str) -> u32 {
    input
        .lines()
        .map(|l| revised_fuel_mass_simple(l.trim().parse::<u32>().unwrap()))
        .sum()
}

#[aoc(day01, part2, revised)]
pub fn revised_1b(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mass = l.trim().parse::<u32>().unwrap();
            revised_fuel_mass_compound(mass) - mass
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use day01::original_1a;
    use day01::original_1b;
    use std::fs;
    const ANSWER_1A: i32 = 3369286;
    const ANSWER_1B: i32 = 5051054;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_1A,
            original_1a(&fs::read_to_string("input/2019/day1.txt").unwrap())
        );
        assert_eq!(
            ANSWER_1B,
            original_1b(&fs::read_to_string("input/2019/day1.txt").unwrap())
        );
    }
}
*/
