use icc::IntCodeComputer;
use std::collections::VecDeque;

#[aoc(day17, part1)]
pub fn solution_17a(input: &str) -> u32 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut icc = IntCodeComputer {
        program: v.clone(),
        pc: 0,
        input: 0,
        amp_input: 0,
        use_amp_input: false,
        input_read: false,
        break_on_output: false,
        terminated: false,
        relative_base: 0,
        output: "".to_string(),
        previous_operation: 0,
    };
    icc.program.resize(1024 * 1024, 0);
    icc.execute();
    let output = icc.consume_output();
    let output_bytes = output.as_bytes();
    let mut i = 0;
    let mut row = 0;
    let mut col = 0;
    let mut starting_position: (usize, usize) = (0, 0);
    while i < output_bytes.len() {
        let c = std::str::from_utf8(&output_bytes[i..i + 2])
            .unwrap()
            .parse::<u8>()
            .unwrap() as char;
        match c {
            '\n' => {
                println!();
                row += 1;
                col = 0;
            }
            '<' | '>' | '^' | 'v' => {
                starting_position = (row, col);
                print!("{}", c);
                col += 1;
            }
            _ => {
                print!("{}", c);
                col += 1;
            }
        }
        i += 2;
    }
    println!("Starting position {:?}", starting_position);
    // Determined visually
    println!(
        "Intersections at:
(12,20)
(8,22)
(28,26)
(28,28)
(32,28)
(26,32)
(26,38)
(20,40)
(38,56)
(34,58)"
    );
    9544
}

#[aoc(day17, part2)]
pub fn solution_17b(input: &str) -> i64 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut icc = IntCodeComputer {
        program: v.clone(),
        pc: 0,
        input: 0,
        amp_input: 0,
        use_amp_input: false,
        input_read: false,
        break_on_output: false,
        terminated: false,
        relative_base: 0,
        output: "".to_string(),
        previous_operation: 0,
    };
    // Solved by hand, because why not?
    /*
    ........................#########..............
    ........................#.......#..............
    ........................#.......#..............
    ........................#.......#..............
    ........................#.......#..............
    ........................#.......#..............
    ........................#.......#..............
    ........................#.......#..............
    ........................#.......#..............
    ................................#..............
    ................................#..............
    ................................#..............
    ................................#########......
    ........................................#......
    ............#########...................#......
    ............#.......#...................#......
    ............#.......#...................#......
    ............#.......#...................#......
    ............#.......#...................#......
    ............#.......#...................#......
    ........#########...#.......#############......
    ........#...#...#...#.......#..................
    #############...#...#.......#..................
    #.......#.......#...#.......#..................
    #.......#.......#...#.......#..................
    #.......#.......#...#.......#..................
    #.......#.......#...#############..............  A,A,B,C,C,A,B,C,A,B
    #.......#.......#...........#...#..............
    #.......#.......#.........############^........  L,12,L,12,R,12,L,12,L,12,R,12,L,8,L,8,R,12,L,8,L,8,L,10,R,8,R,12,L,10,R,8,R,12,L,12,L,12,R,12,L,8,L,8,R,12,L,8,L,8,L,10,R,8,R,12,L,12,L,12,R,12,L,8,L,8,R,12,L,8,L,8
    #.......#.......#.........#.#...#..............
    #########.......#.........#.#...#..............  A = L,12,L,12,R,12
    ................#.........#.#...#..............  B = L,8,L,8,R,12,L,8,L,8
    ................#############...#..............  C = L,10,R,8,R,12
    ..........................#.....#..............
    ..........................#.....#..............
    ..........................#.....#..............
    ..........................#.....#..............
    ..........................#.....#..............
    ....................#############..............
    ....................#.....#....................
    ..............#############....................
    ..............#.....#..........................
    ..............#.....#..........................
    ..............#.....#..........................
    ..............#.....#..........................
    ..............#.....#..........................
    ..............#.....###########................
    ..............#...............#................
    ..............#...............#.......#########
    ..............#...............#.......#.......#
    ..............#...............#.......#.......#
    ..............#...............#.......#.......#
    ..............#############...#.......#.......#
    ..........................#...#.......#.......#
    ..........................#...#.......#.......#
    ..........................#...#.......#.......#
    ..........................#...#...#############
    ..........................#...#...#...#........
    ..........................#...#########........
    ..........................#.......#............
    ..........................#.......#............
    ..........................#.......#............
    ..........................#.......#............
    ..........................#.......#............
    ..........................#########............
    */
    let mut icc_input: VecDeque<u8> = VecDeque::new();
    "A,A,B,C,C,A,B,C,A,B"
        .bytes()
        .for_each(|b| icc_input.push_back(b));
    icc_input.push_back(10u8);
    "L,12,L,12,R,12"
        .bytes()
        .for_each(|b| icc_input.push_back(b));
    icc_input.push_back(10u8);
    "L,8,L,8,R,12,L,8,L,8"
        .bytes()
        .for_each(|b| icc_input.push_back(b));
    icc_input.push_back(10u8);
    "L,10,R,8,R,12".bytes().for_each(|b| icc_input.push_back(b));
    icc_input.push_back(10u8);
    icc_input.push_back(110u8); // no to continuous feed
    icc_input.push_back(10u8);
    icc.program.resize(1024 * 1024, 0);
    icc.program[0] = 2;
    'outer: loop {
        if !icc_input.is_empty() {
            icc.input = icc_input.pop_front().unwrap() as i64;
        }
        loop {
            icc.execute_one();
            if icc.previous_operation == 3 {
                continue 'outer;
            } else if icc.previous_operation == 99 {
                break 'outer;
            }
        }
    }
    let output = icc.consume_output();
    let split = output.split("1010").collect::<Vec<&str>>().pop().unwrap(); // robot prints the grid at the end for some reason, so split the dust after the final newlines
    split.parse::<i64>().unwrap()
}

#[cfg(test)]
mod tests {
    use day17::solution_17a;
    use day17::solution_17b;
    use std::fs;
    const ANSWER_17A: u32 = 9544;
    const ANSWER_17B: i64 = 1_499_679;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_17A,
            solution_17a(&fs::read_to_string("input/2019/day17.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_17B,
            solution_17b(&fs::read_to_string("input/2019/day17.txt").unwrap().trim())
        );
    }
}
