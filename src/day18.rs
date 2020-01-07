//use std::collections::VecDeque;

pub struct Maze {
    grid: Vec<Vec<char>>,
    doors: Vec<char>,
    origin: (usize, usize),
}

pub struct PathHead {
    pub x: usize,
    pub y: usize,
    pub keys: Vec<char>,
}

impl Maze {
    pub fn new(w: usize, h: usize) -> Maze {
        Maze {
            grid: vec![vec![' '; w]; h],
            doors: Vec::new(),
            origin: (0, 0),
        }
    }

    #[allow(dead_code)]
    fn display(&self, _maze: Maze) {
        for r in 0..self.grid.len() {
            for c in 0..self.grid[r].len() {
                print!("{}", self.grid[r][c]);
            }
            println!();
        }
    }

    /*
    fn doors_locked(&self, paths: Vec<PathHead>) -> bool {
        !paths.any(|p| p.keys().len() >= self.doors.len())
    }
    */
}

// TODO: File issue with gobanos/aoc-runner. Why can't I get Vec<Vec<char>> to work with the generator?
#[allow(dead_code)]
#[aoc_generator(day18)]
pub fn generator(input: &str) -> Maze {
    let v = input.lines().map(|l| l.trim()).collect::<Vec<&str>>();
    let mut maze: Maze = Maze::new(v[0].len(), v.len());
    for (i, l) in v.iter().enumerate() {
        for (j, x) in l.chars().enumerate() {
            maze.grid[i][j] = x;
            match x {
                '@' => maze.origin = (i, j),
                'A'..='Z' => maze.doors.push(x),
                _ => {}
            }
        }
    }
    maze
}

#[allow(dead_code)]
#[aoc(day18, part1)]
pub fn shortest_path_to_all_keys(_input: &Maze) -> usize {
    //let maze = generator(input);
    /*
    let mut paths: VecDeque<PathHead> = VecDeque::new();
    paths.push_back(origin);
    while maze.doors_locked(paths) {
        let heads = paths.len();
        for p in paths {}
    }
    */
    0
}

#[aoc(day18, part2)]
pub fn solution_18b(_input: &Maze) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use day18::shortest_path_to_all_keys;
    use day18::solution_18b;
    use std::fs;
    const ANSWER_18A: usize = 0;
    const ANSWER_18B: usize = 0;
    const UNIT_ANSWER_18A_1: usize = 8;
    const UNIT_ANSWER_18A_2: usize = 86;
    const UNIT_ANSWER_18A_3: usize = 132;
    const UNIT_ANSWER_18A_4: usize = 136;
    const UNIT_ANSWER_18A_5: usize = 81;
    const UNIT_INPUT_18A_1: &str = r"#########
#b.A.@.a#
#########";
    const UNIT_INPUT_18A_2: &str = r"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
    const UNIT_INPUT_18A_3: &str = r"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
    const UNIT_INPUT_18A_4: &str = r"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
    const UNIT_INPUT_18A_5: &str = r"########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";

    #[test]
    fn t18a() {
        assert_eq!(
            ANSWER_18A,
            shortest_path_to_all_keys(&fs::read_to_string("input/2019/day18.txt").unwrap().trim())
        );
    }
    #[test]
    fn t18b() {
        assert_eq!(
            ANSWER_18B,
            solution_18b(&fs::read_to_string("input/2019/day18.txt").unwrap().trim())
        );
    }
    #[test]
    fn t18a_supplied_inputs_1() {
        assert_eq!(
            UNIT_ANSWER_18A_1,
            shortest_path_to_all_keys(UNIT_INPUT_18A_1)
        );
    }
    #[test]
    fn t18a_supplied_inputs_2() {
        assert_eq!(
            UNIT_ANSWER_18A_2,
            shortest_path_to_all_keys(UNIT_INPUT_18A_2)
        );
    }
    #[test]
    fn t18a_supplied_inputs_3() {
        assert_eq!(
            UNIT_ANSWER_18A_3,
            shortest_path_to_all_keys(UNIT_INPUT_18A_3)
        );
    }
    #[test]
    fn t18a_supplied_inputs_4() {
        assert_eq!(
            UNIT_ANSWER_18A_4,
            shortest_path_to_all_keys(UNIT_INPUT_18A_4)
        );
    }
    #[test]
    fn t18a_supplied_inputs_5() {
        assert_eq!(
            UNIT_ANSWER_18A_5,
            shortest_path_to_all_keys(UNIT_INPUT_18A_5)
        );
    }
}
