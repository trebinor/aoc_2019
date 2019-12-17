use std::collections::VecDeque;

#[aoc(day16, part1)]
pub fn solution_16a(input: &str) -> u32 {
    let s: Vec<i32> = input
        .trim()
        .chars()
        .map(|n| n.to_string().parse::<i32>().unwrap())
        .collect();
    let base_repeat: Vec<i32> = vec![0, 1, 0, -1];
    let mut previous_signal: Vec<i32> = s.clone();
    let mut signal: Vec<i32> = s.clone();
    for phases in 1..=100 {
        for i in 0..s.len() {
            let mut vdq: VecDeque<i32> = VecDeque::new();
            let mut j = 0;
            let mut first_value_skipped: bool = false;
            'repeat: loop {
                for k in 0..(i + 1) {
                    if !first_value_skipped {
                        first_value_skipped = true;
                        continue;
                    }
                    vdq.push_back(base_repeat[j % 4]);
                    if vdq.len() == s.len() {
                        break 'repeat;
                    }
                }
                j += 1;
            }
            let mut sum: i32 = 0;
            for x in 0..signal.len() {
                sum += previous_signal[x] * vdq[x]
            }
            signal[i] = sum.abs() % 10;
            //println!("{:?}", signal);
        }
        previous_signal = signal.clone();
    }
    println!("{:?}", signal);
    0
}

#[aoc(day16, part2)]
pub fn solution_16b(input: &str) -> u32 {
    let s: Vec<i32> = input
        .trim()
        .chars()
        .map(|n| n.to_string().parse::<i32>().unwrap())
        .collect();
    println!("{:?}", s);
    0
}

#[cfg(test)]
mod tests {
    use day16::solution_16a;
    use day16::solution_16b;
    use std::fs;
    const ANSWER_16A: u32 = 0;
    const ANSWER_16B: u32 = 0;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_16A,
            solution_16a(&fs::read_to_string("input/2019/day16.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_16B,
            solution_16b(&fs::read_to_string("input/2019/day16.txt").unwrap().trim())
        );
    }
}
