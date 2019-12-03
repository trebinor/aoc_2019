const GRID_X: usize = 30000;
const GRID_Y: usize = 30000;

struct Wire {
    x: usize,
    y: usize,
}

#[aoc(day03, part1, original)]
pub fn original_3a(input: &str) -> u32 {
    let v: Vec<&str> = input.split('\n').collect();
    let v1 = v[0].trim().split(',').collect();
    let v2 = v[1].trim().split(',').collect();
    let mut grid = vec![vec![0; GRID_X]; GRID_Y];
    mut_grid(&mut grid, v1);
    mut_grid(&mut grid, v2);
}

fn mut_grid(grid: &mut Vec<Vec<u32>>, wires: Vec<&str>) {
    let mut wire: Wire = Wire {
        x: GRID_X / 2,
        y: GRID_Y / 2,
    };
    for w in wires {
        let dir = w.chars().next().unwrap();
        let dist = w.get(1..).unwrap().parse::<usize>().unwrap();
        if dir == 'R' {
            for d in 0..dist {
                grid[wire.x + d][wire.y] += 1;
            }
            wire.x += dist;
        } else if dir == 'U' {
            for d in 0..dist {
                grid[wire.x][wire.y + d] += 1;
            }
            wire.y += dist;
        } else if dir == 'L' {
            for d in 0..dist {
                grid[wire.x - d][wire.y] += 1;
            }
            wire.x -= dist;
        } else if dir == 'D' {
            for d in 0..dist {
                grid[wire.x][wire.y - d] += 1;
            }
            wire.y -= dist;
        }
    }
}

/*
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
*/
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
