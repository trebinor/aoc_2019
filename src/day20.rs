use petgraph::graph::{DefaultIx, NodeIndex};
use petgraph::Graph;
use std::collections::HashMap;

type Maze = Vec<Vec<char>>;

#[allow(dead_code)]
fn display(maze: Maze) {
    for row in maze {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

#[derive(Debug, Eq, Hash, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[aoc_generator(day20)]
pub fn generator(input: &str) -> Maze {
    let v = input.lines().map(|l| l).collect::<Vec<&str>>();
    let mut maze: Maze = vec![vec![' '; v[0].len()]; v.len()];

    for (i, l) in v.iter().enumerate() {
        for (j, x) in l.chars().enumerate() {
            maze[i][j] = x;
        }
    }
    maze
}

#[aoc(day20, part1)]
pub fn shortest_path_with_portal(input: &Maze) -> usize {
    //display(input.to_vec());
    let mut portals: HashMap<String, Vec<Point>> = HashMap::new();
    let mut spaces: HashMap<Point, NodeIndex<DefaultIx>> = HashMap::new();
    let mut graph = Graph::<Point, usize>::new();
    for (i, r) in input.iter().enumerate() {
        for (j, _c) in r.iter().enumerate() {
            if input[i][j] == '.' {
                let mut name: String = "".to_string();
                if input[i - 1][j].is_ascii_uppercase() {
                    name.push(input[i - 2][j]);
                    name.push(input[i - 1][j]);
                } else if input[i + 1][j].is_ascii_uppercase() {
                    name.push(input[i + 1][j]);
                    name.push(input[i + 2][j]);
                } else if input[i][j - 1].is_ascii_uppercase() {
                    name.push(input[i][j - 2]);
                    name.push(input[i][j - 1]);
                } else if input[i][j + 1].is_ascii_uppercase() {
                    name.push(input[i][j + 1]);
                    name.push(input[i][j + 2]);
                }
                let p = Point { x: i, y: j };
                let node_idx = graph.add_node(p);
                spaces.insert(p, node_idx);
                if input[i - 1][i] == '.' {
                    //graph.update_edge(
                }
                //if grid[i][j]
                if !name.is_empty() {
                    portals.entry(name).and_modify(|e| e.push(p)).or_insert({
                        let mut v = Vec::new();
                        v.push(p);
                        v
                    });
                }
            }
        }
    }

    0
}

#[aoc(day20, part2)]
pub fn solution_20b(_input: &Maze) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use day20::shortest_path_with_portal;
    use day20::solution_20b;
    use std::fs;
    const ANSWER_20A: usize = 0;
    const ANSWER_20B: usize = 0;
    const UNIT_ANSWER_20A_1: usize = 23;
    const UNIT_ANSWER_20A_2: usize = 58;
    const UNIT_INPUT_20A_1: &str = r"         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z       ";
    const UNIT_INPUT_20A_2: &str = r"                   A
                   A
  #################.#############
  #.#...#...................#.#.#
  #.#.#.###.###.###.#########.#.#
  #.#.#.......#...#.....#.#.#...#
  #.#########.###.#####.#.#.###.#
  #.............#.#.....#.......#
  ###.###########.###.#####.#.#.#
  #.....#        A   C    #.#.#.#
  #######        S   P    #####.#
  #.#...#                 #......VT
  #.#.#.#                 #.#####
  #...#.#               YN....#.#
  #.###.#                 #####.#
DI....#.#                 #.....#
  #####.#                 #.###.#
ZZ......#               QG....#..AS
  ###.###                 #######
JO..#.#.#                 #.....#
  #.#.#.#                 ###.#.#
  #...#..DI             BU....#..LF
  #####.#                 #.#####
YN......#               VT..#....QG
  #.###.#                 #.###.#
  #.#...#                 #.....#
  ###.###    J L     J    #.#.###
  #.....#    O F     P    #.#...#
  #.###.#####.#.#####.#####.###.#
  #...#.#.#...#.....#.....#.#...#
  #.#####.###.###.#.#.#########.#
  #...#.#.....#...#.#.#.#.....#.#
  #.###.#####.###.###.#.#.#######
  #.#.........#...#.............#
  #########.###.###.#############
           B   J   C
           U   P   P               ";

    #[test]
    fn t20a_supplied_inputs_1() {
        assert_eq!(
            UNIT_ANSWER_20A_1,
            shortest_path_with_portal(UNIT_INPUT_20A_1)
        );
    }
    #[test]
    fn t20a_supplied_inputs_2() {
        assert_eq!(
            UNIT_ANSWER_20A_2,
            shortest_path_with_portal(UNIT_INPUT_20A_2)
        );
    }

    #[test]
    fn t20a() {
        assert_eq!(
            ANSWER_20A,
            shortest_path_with_portal(&fs::read_to_string("input/2019/day20.txt").unwrap().trim())
        );
    }
    #[test]
    fn t20b() {
        assert_eq!(
            ANSWER_20B,
            solution_20b(&fs::read_to_string("input/2019/day20.txt").unwrap().trim())
        );
    }
}
