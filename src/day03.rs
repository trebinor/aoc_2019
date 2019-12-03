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
    let mut grid1 = vec![vec![false; GRID_X]; GRID_Y];
    let mut grid2 = vec![vec![false; GRID_X]; GRID_Y];
    mut_grid(&mut grid1, v1);
    mut_grid(&mut grid2, v2);

    let mut manhattans = Vec::new();
    for x in 0..GRID_X {
        for y in 0..GRID_Y {
            if grid1[x][y] && grid2[x][y] {
                let manhattan = ((x as i32) - (GRID_X as i32) / 2).abs()
                    + ((y as i32) - (GRID_Y as i32) / 2).abs();
                println!("({},{}) {}", x, y, manhattan);
                manhattans.push(manhattan);
            }
        }
    }
    *manhattans.iter().min().unwrap() as u32
}

fn mut_grid(grid: &mut Vec<Vec<bool>>, wires: Vec<&str>) {
    let mut wire: Wire = Wire {
        x: GRID_X / 2,
        y: GRID_Y / 2,
    };
    for w in wires {
        let dir = w.chars().next().unwrap();
        let dist = w.get(1..).unwrap().parse::<usize>().unwrap();
        if dir == 'R' {
            for d in 0..dist {
                grid[wire.x + d][wire.y] = true;
            }
            wire.x += dist;
        } else if dir == 'U' {
            for d in 0..dist {
                grid[wire.x][wire.y + d] = true;
            }
            wire.y += dist;
        } else if dir == 'L' {
            for d in 0..dist {
                grid[wire.x - d][wire.y] = true;
            }
            wire.x -= dist;
        } else if dir == 'D' {
            for d in 0..dist {
                grid[wire.x][wire.y - d] = true;
            }
            wire.y -= dist;
        }
    }
}

/*
//const PART2_EXPECTED_OUTPUT: u32 = 19690720;
#[aoc(day03, part2, original)]
pub fn original_3b(input: &str) -> u32 {
    let v: Vec<&str> = input.split('\n').collect();
    let v1 = v[0].trim().split(',').collect();
    let v2 = v[1].trim().split(',').collect();
    let mut grid1 = vec![vec![false; GRID_X]; GRID_Y];
    let mut grid2 = vec![vec![false; GRID_X]; GRID_Y];
    mut_grid(&mut grid1, v1);
    mut_grid(&mut grid2, v2);

    let mut manhattans = Vec::new();
    for x in 0..GRID_X {
        for y in 0..GRID_Y {
            if grid1[x][y] && grid2[x][y] {
                let manhattan = ((x as i32) - (GRID_X as i32) / 2).abs()
                    + ((y as i32) - (GRID_Y as i32) / 2).abs();
                println!("({},{}) {}", x, y, manhattan);
                manhattans.push(manhattan);
            }
        }
    }

    // find shortest paths to each intersection and return minimum sum

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
    const ANSWER_3A: u32 = 557;
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
