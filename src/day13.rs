use icc::IntCodeComputer;

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

fn get_tile(tile_num: i64) -> Tile {
    match tile_num {
        0 => Tile::Empty,
        1 => Tile::Wall,
        2 => Tile::Block,
        3 => Tile::HorizontalPaddle,
        4 => Tile::Ball,
        _ => unreachable!(),
    }
}

#[aoc(day13, part1)]
pub fn original_13a(input: &str) -> i32 {
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
        break_on_output: true,
        terminated: false,
        relative_base: 0,
        output: "".to_string(),
        previous_operation: 0,
    };
    icc.program.resize(1024 * 1024, 0);
    let mut block_tiles: i32 = 0;
    loop {
        icc.execute();
        if icc.terminated {
            break;
        }
        let x: i32 = icc.consume_output().parse().unwrap();
        icc.execute();
        let y: i32 = icc.consume_output().parse().unwrap();
        icc.execute();
        let t: Tile = get_tile(icc.consume_output().parse().unwrap());
        if t == Tile::Block {
            block_tiles += 1;
        }
    }
    block_tiles
}

#[aoc(day13, part2)]
pub fn original_13b(input: &str) -> i64 {
    let mut v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut icc = IntCodeComputer {
        program: v.clone(),
        pc: 0,
        input: 0, // neutral joystick by default
        amp_input: 0,
        use_amp_input: false,
        input_read: false,
        break_on_output: true,
        terminated: false,
        relative_base: 0,
        output: "".to_string(),
        previous_operation: 0,
    };
    icc.program.resize(1024 * 1024, 0);
    v[0] = 2; // 2 quarters
    let mut score: i64 = 0;
    let mut block_tiles: i32 = 0;
    let mut paddle_x: i32 = 0;
    let mut ball_x: i32 = 0;
    loop {
        icc.execute();
        if icc.terminated {
            break;
        } else if icc.previous_operation == 4 {
            // TODO: outputs are not necessarily sequential. Track the state.
            let x: i32 = icc.consume_output().parse().unwrap();
            icc.execute();
            let y: i32 = icc.consume_output().parse().unwrap();
            icc.execute();
            let output: i64 = icc.consume_output().parse().unwrap();
            if x == -1 && y == 0 {
                score = output;
            } else {
                let t: Tile = get_tile(output);
                if t == Tile::Block {
                    block_tiles += 1;
                } else if t == Tile::HorizontalPaddle {
                    paddle_x = x;
                } else if t == Tile::Ball {
                    ball_x = x;
                }
            }
        }
        icc.input = if ball_x < paddle_x {
            -1
        } else if ball_x > paddle_x {
            1
        } else {
            0
        };
    }
    score
}

/*
#[cfg(test)]
mod tests {
    use day09::original_9a;
    use day09::original_9b;
    use std::fs;
    const ANSWER_9A: i64 = 2932210790;
    const ANSWER_9B: i64 = 73144;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_9A,
            original_9a(&fs::read_to_string("input/2019/day9.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_9B,
            original_9b(&fs::read_to_string("input/2019/day9.txt").unwrap().trim())
        );
    }
}
*/
