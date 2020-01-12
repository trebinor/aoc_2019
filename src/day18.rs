type Maze = Vec<Vec<char>>;

#[allow(dead_code)]
fn display(maze: Maze) {
    for (j, y) in maze.iter().enumerate() {
        for (i, _x) in y.iter().enumerate() {
            print!("{}", maze[j][i]);
        }
        println!();
    }
}

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Maze {
    let v = input.lines().map(|l| l).collect::<Vec<&str>>();
    let mut maze: Maze = vec![vec![' '; v[0].len()]; v.len()];

    for (y, l) in v.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            maze[y][x] = c;
        }
    }
    maze
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point(i8, i8, u32);
impl Point {
    fn key_present(&self, key: char) -> bool {
        self.2 & (1 << (key as u8 - 97u8)) != 0
    }

    fn add_key(&mut self, key: char) {
        match key {
            'a'..='z' => self.2 |= 1 << (key as u8 - 97u8),
            _ => unreachable!(),
        }
    }
    fn successors(&self, input: &Maze) -> Vec<Point> {
        let &Point(x, y, k) = self;
        let mut path = Vec::new();
        for m in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let i: i8 = x + m.0;
            let j: i8 = y + m.1;
            let mut keys = k;
            let mut key_point = Point(0, 0, k);
            match input[j as usize][i as usize] {
                '#' => continue,
                door @ 'A'..='Z' => {
                    if !self.key_present(door.to_ascii_lowercase()) {
                        continue;
                    }
                }
                key @ 'a'..='z' => {
                    keys = {
                        key_point.add_key(key);
                        key_point.2
                    }
                }
                _ => {}
            }
            path.push(Point(i, j, keys));
        }
        path
    }
}

#[aoc(day18, part1)]
pub fn shortest_path_to_all_keys(input: &Maze) -> usize {
    let mut goal = Point(0, 0, 0);
    let mut origin_x: i8 = 0;
    let mut origin_y: i8 = 0;
    for (j, y) in input.iter().enumerate() {
        for (i, _x) in y.iter().enumerate() {
            match input[j][i] {
                '@' => {
                    origin_x = i as i8;
                    origin_y = j as i8;
                }
                key @ 'a'..='z' => goal.add_key(key),
                _ => {}
            }
        }
    }
    let origin = Point(origin_x, origin_y, 0);
    let shortest_path =
        pathfinding::directed::bfs::bfs(&origin, |p| p.successors(input), |p| p.2 == goal.2);
    shortest_path.unwrap().len() - 1 // off by 1 for some reason
}

#[aoc(day18, part2)]
pub fn solution_18b(_input: &Maze) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use day18::generator;
    use day18::shortest_path_to_all_keys;
    use day18::solution_18b;
    use std::fs;
    const ANSWER_18A: usize = 4118;
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
            shortest_path_to_all_keys(&generator(
                &fs::read_to_string("input/2019/day18.txt").unwrap().trim()
            ))
        );
    }
    #[test]
    fn t18b() {
        assert_eq!(
            ANSWER_18B,
            solution_18b(&generator(
                &fs::read_to_string("input/2019/day18.txt").unwrap().trim()
            ))
        );
    }
    #[test]
    fn t18a_supplied_inputs_1() {
        assert_eq!(
            UNIT_ANSWER_18A_1,
            shortest_path_to_all_keys(&generator(UNIT_INPUT_18A_1))
        );
    }
    #[test]
    fn t18a_supplied_inputs_2() {
        assert_eq!(
            UNIT_ANSWER_18A_2,
            shortest_path_to_all_keys(&generator(UNIT_INPUT_18A_2))
        );
    }
    #[test]
    fn t18a_supplied_inputs_3() {
        assert_eq!(
            UNIT_ANSWER_18A_3,
            shortest_path_to_all_keys(&generator(UNIT_INPUT_18A_3))
        );
    }
    #[test]
    fn t18a_supplied_inputs_4() {
        assert_eq!(
            UNIT_ANSWER_18A_4,
            shortest_path_to_all_keys(&generator(UNIT_INPUT_18A_4))
        );
    }
    #[test]
    fn t18a_supplied_inputs_5() {
        assert_eq!(
            UNIT_ANSWER_18A_5,
            shortest_path_to_all_keys(&generator(UNIT_INPUT_18A_5))
        );
    }
}
