use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Life {
    Empty,
    Bug,
}

const GRID_X: usize = 5;
const GRID_Y: usize = 5;
type GridLevel = [[Life; GRID_X]; GRID_Y];

#[aoc_generator(day24)]
pub fn generator(input: &str) -> [[Life; GRID_X]; GRID_Y] {
    let v = input.lines().map(|l| l.trim()).collect::<Vec<&str>>();
    let mut eris = [[Life::Empty; GRID_X]; GRID_Y];
    for (i, l) in v.iter().enumerate() {
        for (j, x) in l.chars().enumerate() {
            eris[i][j] = match x {
                '.' => Life::Empty,
                '#' => Life::Bug,
                _ => unreachable!(),
            }
        }
    }
    eris
}

fn biodiversity(grid: GridLevel) -> usize {
    let mut sum: usize = 0;
    for i in 0..GRID_X {
        for j in 0..GRID_Y {
            if grid[i][j] == Life::Bug {
                sum += 2usize.pow((i as u32) * (GRID_X as u32) + j as u32);
            }
        }
    }
    sum
}

fn total_bugs(grid: GridLevel) -> usize {
    let mut sum: usize = 0;
    for i in 0..GRID_X {
        for j in 0..GRID_Y {
            if grid[i][j] == Life::Bug {
                sum += 1;
            }
        }
    }
    sum
}

fn print_generation(grid: [[Life; GRID_X]; GRID_Y]) {
    for i in 0..GRID_X {
        for j in 0..GRID_Y {
            match grid[i][j] {
                Life::Empty => print!("."),
                Life::Bug => print!("#"),
            }
        }
        println!();
    }
    println!();
}

fn new_generation(grid: &mut [[Life; GRID_X]; GRID_Y]) {
    let mut grid_expanded = [[Life::Empty; GRID_X + 2]; GRID_Y + 2];
    for i in 0..GRID_X {
        for j in 0..GRID_Y {
            grid_expanded[i + 1][j + 1] = grid[i][j];
        }
    }
    for i in 1..=GRID_X {
        for j in 1..=GRID_Y {
            let mut adj = 0;
            if grid_expanded[i - 1][j] == Life::Bug {
                adj += 1;
            }
            if grid_expanded[i + 1][j] == Life::Bug {
                adj += 1;
            }
            if grid_expanded[i][j - 1] == Life::Bug {
                adj += 1;
            }
            if grid_expanded[i][j + 1] == Life::Bug {
                adj += 1;
            }
            grid[i - 1][j - 1] = match grid_expanded[i][j] {
                Life::Bug => {
                    if adj == 1 {
                        Life::Bug
                    } else {
                        Life::Empty
                    }
                }
                Life::Empty => {
                    if adj == 1 || adj == 2 {
                        Life::Bug
                    } else {
                        Life::Empty
                    }
                }
            }
        }
    }
}

#[aoc(day24, part1)]
pub fn biodiversity_of_first_repeated_state(grid: &[[Life; GRID_X]; GRID_Y]) -> usize {
    let mut grid_mutable = *grid;
    let mut bio: HashSet<usize> = HashSet::new();
    while bio.insert(biodiversity(grid_mutable)) {
        print_generation(grid_mutable);
        new_generation(&mut grid_mutable);
    }
    biodiversity(grid_mutable)
}

#[aoc(day24, part2)]
pub fn total_bugs_200_iterations(grid: &GridLevel) -> usize {
    total_bugs_n_iterations(grid, 200)
}

fn total_bugs_n_iterations(grid: &GridLevel, n: usize) -> usize {
    let mut grids: VecDeque<GridLevel> = VecDeque::new();
    grids.push_front(grid.clone());
    for _iterations in 0..n {
        let outer_grid = [[Life::Empty; GRID_X]; GRID_Y];
        let inner_grid = [[Life::Empty; GRID_X]; GRID_Y];
        grids.push_back(outer_grid.clone());
        grids.push_front(inner_grid.clone());
        for g in grids.iter() {
            if total_bugs(*g) == 0 {
                continue;
            }
        }
    }
    grids.iter().fold(0, |acc, g| acc + total_bugs(*g))
}

#[cfg(test)]
mod tests {
    use day24::biodiversity_of_first_repeated_state;
    use day24::generator;
    use day24::solution_24b;
    use std::fs;
    const ANSWER_24A: usize = 1_151_290;
    const ANSWER_24B: usize = 0;
    const UNIT_ANSWER_24A: usize = 2_129_920;
    const UNIT_INPUT_24A: &str = r"....#
#..#.
#..##
..#..
#....";

    #[test]
    fn t24a() {
        assert_eq!(
            ANSWER_24A,
            biodiversity_of_first_repeated_state(&generator(
                &fs::read_to_string("input/2019/day24.txt").unwrap().trim()
            ))
        );
    }
    #[test]
    fn t24b() {
        assert_eq!(
            ANSWER_24B,
            solution_24b(&generator(
                &fs::read_to_string("input/2019/day24.txt").unwrap().trim()
            ))
        );
    }
    #[test]
    fn t24a_supplied_inputs() {
        assert_eq!(
            UNIT_ANSWER_24A,
            biodiversity_of_first_repeated_state(&generator(UNIT_INPUT_24A))
        );
    }
}
