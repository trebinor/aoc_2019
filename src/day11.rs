//use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use icc::IntCodeComputer;
const GRID_X: usize = 30000;
const GRID_Y: usize = 30000;

#[derive(Clone, Copy, Debug, std::cmp::PartialEq)]
enum PanelColor {
    Black,
    White,
}

#[derive(Clone, Copy, Debug, std::cmp::PartialEq)]
enum TurnDirection {
    Left,
    Right
}

#[derive(Clone, Copy, Debug, std::cmp::PartialEq)]
enum Orienation {
    Up,
    Down,
    Left,
    Right 
}

#[derive(Debug, Clone)]
struct PanelPoint {
    painted: bool,
    color: PanelColor,
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day11, part1)]
pub fn panels_painted_at_least_once(v: &[i64]) -> u32 {
    let mut panels_painted = 0;
    let mut robot_orientation = Orienation::Up;
    let mut panel = vec![vec![PanelPoint {painted: false, color: PanelColor::Black}; GRID_X]; GRID_Y];
    //let (mut x, mut y) = (GRID_X / 2, GRID_Y / 2);
    let mut x = GRID_X / 2;
    let mut y = GRID_Y / 2;
    let mut icc = IntCodeComputer{
        program: v.to_vec().clone(),
        pc: 0,
        input: 0,
        amp_input: 0,
        use_amp_input: false,
        input_read: false,
        break_on_output: true,
        terminated: false,
        relative_base: 0,
        output: "".to_string(),
    };
    icc.program.resize(1024 * 1024, 0);
    panel[x][y].color = PanelColor::White;
    loop {
        //println!("Current panel {:?}", panel[x][y].color);
        icc.input = if panel[x][y].color == PanelColor::Black { 0 } else { 1 };
        icc.execute();
                if icc.terminated {
            break;
        }
        let paint_this_panel_color = if icc.consume_output().parse::<i64>().unwrap() == 0 { PanelColor::Black } else { PanelColor::White };
        if !panel[x][y].painted {
            panels_painted += 1;
        }
        panel[x][y].painted = true;
        panel[x][y].color = paint_this_panel_color;
        icc.execute();
                if icc.terminated {
            break;
        }
        let turn_direction = if icc.consume_output().parse::<i64>().unwrap() == 0 { TurnDirection::Left } else { TurnDirection::Right };
        match robot_orientation {
            Orienation::Up => if turn_direction == TurnDirection::Left { robot_orientation = Orienation::Left; x -= 1; } else { robot_orientation = Orienation::Right; x += 1;},
            Orienation::Down => if turn_direction == TurnDirection::Left { robot_orientation = Orienation::Right; x += 1; } else { robot_orientation = Orienation::Left; x -= 1;},
            Orienation::Left => if turn_direction == TurnDirection::Left { robot_orientation = Orienation::Down; y -= 1; } else { robot_orientation = Orienation::Up; y += 1;},
            Orienation::Right => if turn_direction == TurnDirection::Left { robot_orientation = Orienation::Up; y += 1; } else { robot_orientation = Orienation::Down; y -= 1;},
            _ => unreachable!(),
        }

        //println!("New coordinates {},{}", x, y);
    }
    //panels_painted
    for c in (GRID_X / 2) - 50..(GRID_X / 2) + 50 {
        for r in (GRID_Y / 2) - 50..(GRID_Y / 2 + 50) {
            match panel[r][c].color {
                PanelColor::Black => print!(" "),
                PanelColor::White => print!("*"),
                _ => unreachable!(),
            }
        }
        println!();
    }
    0
}

#[cfg(test)]
mod tests {
    use day11::panels_painted_at_least_once;
    use day11::generator;
    const ANSWER_11A: u32 = 0;
    const ANSWER_11B: u32 = 0;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_11A,
            panels_painted_at_least_once(&generator(&fs::read_to_string("input/2019/day11.txt").unwrap().trim())));
        assert_eq!(
            ANSWER_11B,
            panels_painted_at_least_once(&generator(&fs::read_to_string("input/2019/day11.txt").unwrap().trim())));
    }

    #[test]
    fn supplied_inputs() {
        assert_eq!(UNIT_ANSWER_10A_1, generator(UNIT_INPUT_10A_1).find_best_point());
        assert_eq!(UNIT_ANSWER_10A_2, generator(UNIT_INPUT_10A_2).find_best_point());
        assert_eq!(UNIT_ANSWER_10A_3, generator(UNIT_INPUT_10A_3).find_best_point());
        assert_eq!(UNIT_ANSWER_10A_4, generator(UNIT_INPUT_10A_4).find_best_point());
        assert_eq!(33, visible_asteroids(&generator(UNIT_INPUT_10A_1)));
        assert_eq!(35, visible_asteroids(&generator(UNIT_INPUT_10A_2)));
        assert_eq!(41, visible_asteroids(&generator(UNIT_INPUT_10A_3)));
        assert_eq!(210, visible_asteroids(&generator(UNIT_INPUT_10A_4)));
    }
}
