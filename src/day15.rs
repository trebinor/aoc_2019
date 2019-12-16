use icc::IntCodeComputer;
use rand::prelude::*;

const GRID_X: usize = 100;
const GRID_Y: usize = 100;

#[derive(PartialEq, Debug)]
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

#[aoc(day15, part1)]
pub fn original_15a(input: &str) -> u32 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();

    let mut rng = rand::thread_rng();
    let mut min_steps: u32 = std::u32::MAX;
    let mut grid = [[' '; GRID_X]; GRID_Y];
    let mut x;
    let mut y;
    for i in 1..=10000 {
        x = GRID_X / 2;
        y = GRID_Y / 2;
        grid[x][y] = 'X';
        println!("Sim {}", i);
        let mut steps: u32 = 0;
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
        loop {
            icc.input = rng.gen_range(1, 5); //NSWE
            icc.execute();
            match icc.consume_output().parse().unwrap() {
                0 => {
                    match icc.input {
                        1 => grid[x][y + 1] = '#',
                        2 => grid[x][y - 1] = '#',
                        3 => grid[x - 1][y] = '#',
                        4 => grid[x + 1][y] = '#',
                        _ => unreachable!(),
                    };
                    //println!("Robot found wall. Still at ({},{})", x, y);
                }
                1 => {
                    grid[x][y] = if grid[x][y] == 'X' { 'X' } else { '.' };
                    match icc.input {
                        1 => y += 1,
                        2 => y -= 1,
                        3 => x -= 1,
                        4 => x += 1,
                        _ => unreachable!(),
                    }
                    //println!("Robot moved to ({},{})", x, y);
                    grid[x][y] = if grid[x][y] == 'X' { 'X' } else { '.' };
                    steps += 1;
                }
                2 => {
                    grid[x][y] = if grid[x][y] == 'X' { 'X' } else { '.' };
                    match icc.input {
                        1 => y += 1,
                        2 => y -= 1,
                        3 => x -= 1,
                        4 => x += 1,
                        _ => unreachable!(),
                    }
                    //println!("Robot moved to ({},{}) and found oxygen system there", x, y);
                    grid[x][y] = 'S';
                    steps += 1;
                    break;
                }
                _ => unreachable!(),
            }
        }
        min_steps = std::cmp::min(steps, min_steps);
    }
    print_grid(&grid);
    min_steps
}

/* Grid after 10000 random walks */

fn print_grid(grid: &[[char; 100]; 100]) {
    for a in 0..GRID_X {
        for b in 0..GRID_Y {
            print!("{}", grid[b][a]);
        }
        println!();
    }
}

#[aoc(day15, part2)]
pub fn original_15b(input: &str) -> i64 {
    /*
    let mut v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    v[0] = 2; // 2 quarters
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
    let mut score: i64 = 0;
    let mut block_tiles: i32 = 0;
    let mut paddle_x: i32 = 0;
    let mut ball_x: i32 = 0;
    loop {
        icc.execute();
        if icc.terminated {
            break;
        } else if icc.previous_operation == 4 {
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
    */
    0
}

#[cfg(test)]
mod tests {
    use day15::original_15a;
    use day15::original_15b;
    use std::fs;
    const ANSWER_15A: i64 = 204;
    const ANSWER_15B: i64 = 21426;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_15A,
            original_15a(&fs::read_to_string("input/2019/day15.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_15B,
            original_15b(&fs::read_to_string("input/2019/day15.txt").unwrap().trim())
        );
    }
}
