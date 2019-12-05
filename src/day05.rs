#[aoc(day05, part1, original)]
pub fn original_5a(input: &str) -> i32 {
    let mut v: Vec<&str> = input
        .split(',')
        //.map(|o| o.parse::<i32>().unwrap())
        .collect();
    //v[1] = 12;
    //v[2] = 2;
    compute(&mut v)
}

fn compute(mut v: &mut Vec<&str>) -> i32 {
    let mut pc: usize = 0;
    const INPUT: i32 = 1;
    loop {
        // Seriously, wtf is this? Is there an easier way to get the last 2 chars of a string in Rust?
        let opcode = (*v[pc])
            .chars()
            .rev()
            .take(2)
            .collect::<Vec<char>>()
            .iter()
            .rev()
            .take(2)
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        match opcode {
            1 => add(v, &mut pc),
            2 => mul(v, &mut pc),
            3 => println!("3"),
            4 => println!("4"),
            //3 => store(&mut v, input, &pc);
            //4 => show(&mut v, &pc);
            99 => break,
            _ => panic!(),
        }
    }
    0 //v[0]
}

fn add(v: &mut Vec<&str>, pc: &mut usize) {
    println!("add");
    /*
    let ia1: usize = v[*pc + 1] as usize;
    let ia2: usize = v[*pc + 2] as usize;
    let oa: usize = v[*pc + 3] as usize;
    v[oa] = v[ia1] + v[ia2];
    */
    *pc += 4;
}

fn mul(v: &mut Vec<&str>, pc: &mut usize) {
    println!("mul");
    /*
    let ia1: usize = v[*pc + 1] as usize;
    let ia2: usize = v[*pc + 2] as usize;
    let oa: usize = v[*pc + 3] as usize;
    v[oa] = v[ia1] * v[ia2];
    */
    *pc += 4;
}

/*
fn store(v: &mut Vec<u32>, input: i32, &pc: usize) {
    let ia1: usize = v[i + 1] as usize;
    let ia2: usize = v[i + 2] as usize;
    let oa: usize = v[i + 3] as usize;
    v[oa] = v[ia1] * v[ia2];
    pc += 2;
}
fn show(v: &mut Vec<u32>, input: i32, &pc: usize) {
    let ia1: usize = v[i + 1] as usize;
    let ia2: usize = v[i + 2] as usize;
    let oa: usize = v[i + 3] as usize;
    v[oa] = v[ia1] * v[ia2];
    pc += 2;
}
*/

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
