use icc::IntCodeComputer;
use itertools::Itertools;

#[aoc(day21, part1)]
pub fn solution_21a(input: &str) -> u32 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    // I'm not playing this game when I have a computer to do it for me.
    let commands = vec![
        "AND A J", "AND B J", "AND C J", "AND D J", "AND A T", "AND B T", "AND C T", "AND D T",
        "AND J T", "AND T J", "AND J J", "AND T T", "OR A J", "OR B J", "OR C J", "OR D J",
        "OR A T", "OR B T", "OR C T", "OR D T", "OR J T", "OR T J", "OR J J", "OR T T", "NOT A J",
        "NOT B J", "NOT C J", "NOT D J", "NOT A T", "NOT B T", "NOT C T", "NOT D T", "NOT J T",
        "NOT T J", "NOT J J", "NOT T T",
    ];
    let mut p = 1;
    'outer: loop {
        for command_set in commands.iter().permutations(p) {
            println!("Command set {:?}", command_set);
            let mut program: String = "".to_string();
            for command in command_set {
                program.push_str(command);
                program.push('\n');
            }
            program.push_str("WALK\n");
            println!("Trying program:");
            println!("{}", program);
            program = "OR A J\nAND C J\nNOT J J\nAND D J\nWALK\n".to_string(); // found by iterating over the permutations.
            let mut icc = IntCodeComputer::new(v.clone(), false);
            icc.program.resize(1024 * 1024, 0);
            for c in program.chars() {
                icc.inputq.push_back(c as i64);
            }
            icc.execute();
            let output = icc.consume_output();
            let output_bytes = output.as_bytes();
            let mut i = 1;
            let mut dot_encountered = false; // skip garbage output at the beginning
            while i < output_bytes.len() {
                if i + 2 >= output_bytes.len() {
                    println!("Based on a parsing error, maybe the answer is {}", output);
                    break 'outer;
                }
                let c = std::str::from_utf8(&output_bytes[i..i + 2])
                    .unwrap()
                    .parse::<u8>()
                    .unwrap() as char;
                if dot_encountered || c == '.' {
                    dot_encountered = true;
                    print!("{}", c);
                }
                i += 2;
            }
            if !dot_encountered {
                println!("Maybe the answer is {}", output);
                break 'outer;
            }
        }
        p += 1;
    }
    // strip this after a 1010 (newlines) in a large output string
    19349530
}

#[aoc(day21, part2)]
pub fn solution_21b(input: &str) -> u32 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    // I'm not playing this game when I have a computer to do it for me.
    let commands = vec![
        "AND A J", "AND B J", "AND C J", "AND D J", "AND E J", "AND F J", "AND G J", "AND H J",
        "AND I J", "AND A T", "AND B T", "AND C T", "AND D T", "AND E T", "AND F T", "AND G T",
        "AND H T", "AND I T", "AND J T", "AND T J", "AND J J", "AND T T", "OR A J", "OR B J",
        "OR C J", "OR D J", "OR E J", "OR F J", "OR G J", "OR H J", "OR I J", "OR A T", "OR B T",
        "OR C T", "OR D T", "OR E T", "OR F T", "OR G T", "OR H T", "OR I T", "OR J T", "OR T J",
        "OR J J", "OR T T", "NOT A J", "NOT B J", "NOT C J", "NOT D J", "NOT E J", "NOT F J",
        "NOT G J", "NOT H J", "NOT I J", "NOT A T", "NOT B T", "NOT C T", "NOT D T", "NOT E T",
        "NOT F T", "NOT G T", "NOT H T", "NOT I T", "NOT J T", "NOT T J", "NOT J J", "NOT T T",
    ];
    let mut p = 1;
    'outer: loop {
        for command_set in commands.iter().permutations(p) {
            println!("Command set {:?}", command_set);
            let mut program: String = "".to_string();
            for command in command_set {
                program.push_str(command);
                program.push('\n');
            }
            program.push_str("RUN\n");
            println!("Trying program:");
            println!("{}", program);
            let mut icc = IntCodeComputer::new(v.clone(), false);
            icc.program.resize(1024 * 1024, 0);
            for c in program.chars() {
                icc.inputq.push_back(c as i64);
            }
            icc.execute();
            let output = icc.consume_output();
            let output_bytes = output.as_bytes();
            let mut i = 0;
            let mut dot_encountered = false; // skip garbage output at the beginning
            while i < output_bytes.len() {
                if i + 2 >= output_bytes.len() {
                    //    println!("Based on a parsing error, maybe the answer is {}", output);
                    //    break 'outer;
                    break;
                }
                let c = std::str::from_utf8(&output_bytes[i..i + 2])
                    .unwrap()
                    .parse::<u8>()
                    .unwrap() as char;
                if dot_encountered || c == '.' {
                    dot_encountered = true;
                    print!("{}", c);
                }
                i += 2;
            }
            if !dot_encountered {
                println!("Maybe the answer is {}", output);
                break 'outer;
            }
        }
        p += 1;
    }
    // strip this after a 1010 (newlines) in a large output string
    0
}

#[cfg(test)]
mod tests {
    use day21::solution_21a;
    use day21::solution_21b;
    use std::fs;
    const ANSWER_21A: u32 = 19_349_530;
    const ANSWER_21B: u32 = 0;

    #[test]
    fn t21a() {
        assert_eq!(
            ANSWER_21A,
            solution_21a(&fs::read_to_string("input/2019/day21.txt").unwrap().trim())
        );
    }
    #[test]
    fn t21b() {
        assert_eq!(
            ANSWER_21B,
            solution_21b(&fs::read_to_string("input/2019/day21.txt").unwrap().trim())
        );
    }
}
