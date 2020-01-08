use petgraph::algo::dijkstra;
use petgraph::graph::{DefaultIx, NodeIndex};
use petgraph::Graph;
use petgraph::Undirected;
use std::collections::HashMap;

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
    println!("Input is {}", input);
    let v = input.lines().map(|l| l).collect::<Vec<&str>>();
    let mut maze: Maze = vec![vec![' '; v[0].len()]; v.len()];

    for (y, l) in v.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            maze[y][x] = c;
        }
    }
    maze
}

#[aoc(day20, part1)]
pub fn shortest_portal_path(input: &Maze) -> usize {
    display(input.to_vec());
    let mut portals: HashMap<String, Vec<Point>> = HashMap::new();
    let mut spaces: HashMap<Point, NodeIndex<DefaultIx>> = HashMap::new();
    let mut graph = Graph::<Point, usize, Undirected>::new_undirected();
    for (j, y) in input.iter().enumerate() {
        for (i, _x) in y.iter().enumerate() {
            if input[j][i] == '.' {
                let mut name: String = "".to_string();
                if input[j - 1][i].is_ascii_uppercase() {
                    name.push(input[j - 2][i]);
                    name.push(input[j - 1][i]);
                } else if input[j + 1][i].is_ascii_uppercase() {
                    name.push(input[j + 1][i]);
                    name.push(input[j + 2][i]);
                } else if input[j][i - 1].is_ascii_uppercase() {
                    name.push(input[j][i - 2]);
                    name.push(input[j][i - 1]);
                } else if input[j][i + 1].is_ascii_uppercase() {
                    name.push(input[j][i + 1]);
                    name.push(input[j][i + 2]);
                }
                let p = Point { x: i, y: j };
                let node_idx = graph.add_node(p);
                spaces.insert(p, node_idx);
                if input[j - 1][i] == '.' {
                    //println!("Adding edge from {:?} to ({},{})", p, j - 1, i);
                    let other_node = spaces.get(&Point { x: i, y: j - 1 });
                    graph.update_edge(node_idx, *other_node.unwrap(), 1);
                }
                if input[j][i - 1] == '.' {
                    //println!("Adding edge from {:?} to ({},{})", p, i - 1, j);
                    let other_node = spaces.get(&Point { x: i - 1, y: j });
                    graph.update_edge(node_idx, *other_node.unwrap(), 1);
                }

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

    for (k, v) in portals.iter() {
        if k != "AA" && k != "ZZ" {
            //println!("Adding edge from portal space {:?} to {:?}", v[0], v[1]);
            graph.update_edge(*spaces.get(&v[0]).unwrap(), *spaces.get(&v[1]).unwrap(), 1);
        }
    }

    let start = *spaces.get(&portals.get("AA").unwrap()[0]).unwrap();
    let goal = *spaces.get(&portals.get("ZZ").unwrap()[0]).unwrap();
    //println!("Start is {:?}", start);
    //println!("Goal is {:?}", goal);
    //println!("Portals are {:?}", portals);
    //println!("Spaces are {:?}", spaces);
    //println!("Graph is {:?}", graph);
    let res = dijkstra(&graph, start, Some(goal), |_| 1);
    //println!("{:?}", res);

    *res.get(&goal).unwrap()
}

#[aoc(day20, part2)]
pub fn shortest_recursive_portal_path(_input: &Maze) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use day20::generator;
    use day20::shortest_portal_path;
    use day20::shortest_recursive_portal_path;
    use std::fs;
    const ANSWER_20A: usize = 464;
    const ANSWER_20B: usize = 0;
    const UNIT_ANSWER_20A_1: usize = 23;
    const UNIT_ANSWER_20A_2: usize = 58;
    const UNIT_ANSWER_20B_1: usize = 396;
    const UNIT_INPUT_20A_1: &str = concat!(
        "         A           \n",
        "         A           \n",
        "  #######.#########  \n",
        "  #######.........#  \n",
        "  #######.#######.#  \n",
        "  #######.#######.#  \n",
        "  #######.#######.#  \n",
        "  #####  B    ###.#  \n",
        "BC...##  C    ###.#  \n",
        "  ##.##       ###.#  \n",
        "  ##...DE  F  ###.#  \n",
        "  #####    G  ###.#  \n",
        "  #########.#####.#  \n",
        "DE..#######...###.#  \n",
        "  #.#########.###.#  \n",
        "FG..#########.....#  \n",
        "  ###########.#####  \n",
        "             Z       \n",
        "             Z       \n",
    );
    const UNIT_INPUT_20A_2: &str = concat!(
        "                   A               \n",
        "                   A               \n",
        "  #################.#############  \n",
        "  #.#...#...................#.#.#  \n",
        "  #.#.#.###.###.###.#########.#.#  \n",
        "  #.#.#.......#...#.....#.#.#...#  \n",
        "  #.#########.###.#####.#.#.###.#  \n",
        "  #.............#.#.....#.......#  \n",
        "  ###.###########.###.#####.#.#.#  \n",
        "  #.....#        A   C    #.#.#.#  \n",
        "  #######        S   P    #####.#  \n",
        "  #.#...#                 #......VT\n",
        "  #.#.#.#                 #.#####  \n",
        "  #...#.#               YN....#.#  \n",
        "  #.###.#                 #####.#  \n",
        "DI....#.#                 #.....#  \n",
        "  #####.#                 #.###.#  \n",
        "ZZ......#               QG....#..AS\n",
        "  ###.###                 #######  \n",
        "JO..#.#.#                 #.....#  \n",
        "  #.#.#.#                 ###.#.#  \n",
        "  #...#..DI             BU....#..LF\n",
        "  #####.#                 #.#####  \n",
        "YN......#               VT..#....QG\n",
        "  #.###.#                 #.###.#  \n",
        "  #.#...#                 #.....#  \n",
        "  ###.###    J L     J    #.#.###  \n",
        "  #.....#    O F     P    #.#...#  \n",
        "  #.###.#####.#.#####.#####.###.#  \n",
        "  #...#.#.#...#.....#.....#.#...#  \n",
        "  #.#####.###.###.#.#.#########.#  \n",
        "  #...#.#.....#...#.#.#.#.....#.#  \n",
        "  #.###.#####.###.###.#.#.#######  \n",
        "  #.#.........#...#.............#  \n",
        "  #########.###.###.#############  \n",
        "           B   J   C               \n",
        "           U   P   P               \n",
    );

    const UNIT_INPUT_20B_1: &str = concat!(
        "             Z L X W       C                 \n",
        "             Z P Q B       K                 \n",
        "  ###########.#.#.#.#######.###############  \n",
        "  #...#.......#.#.......#.#.......#.#.#...#  \n",
        "  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  \n",
        "  #.#...#.#.#...#.#.#...#...#...#.#.......#  \n",
        "  #.###.#######.###.###.#.###.###.#.#######  \n",
        "  #...#.......#.#...#...#.............#...#  \n",
        "  #.#########.#######.#.#######.#######.###  \n",
        "  #...#.#    F       R I       Z    #.#.#.#  \n",
        "  #.###.#    D       E C       H    #.#.#.#  \n",
        "  #.#...#                           #...#.#  \n",
        "  #.###.#                           #.###.#  \n",
        "  #.#....OA                       WB..#.#..ZH\n",
        "  #.###.#                           #.#.#.#  \n",
        "CJ......#                           #.....#  \n",
        "  #######                           #######  \n",
        "  #.#....CK                         #......IC\n",
        "  #.###.#                           #.###.#  \n",
        "  #.....#                           #...#.#  \n",
        "  ###.###                           #.#.#.#  \n",
        "XF....#.#                         RF..#.#.#  \n",
        "  #####.#                           #######  \n",
        "  #......CJ                       NM..#...#  \n",
        "  ###.#.#                           #.###.#  \n",
        "RE....#.#                           #......RF\n",
        "  ###.###        X   X       L      #.#.#.#  \n",
        "  #.....#        F   Q       P      #.#.#.#  \n",
        "  ###.###########.###.#######.#########.###  \n",
        "  #.....#...#.....#.......#...#.....#.#...#  \n",
        "  #####.#.###.#######.#######.###.###.#.#.#  \n",
        "  #.......#.......#.#.#.#.#...#...#...#.#.#  \n",
        "  #####.###.#####.#.#.#.#.###.###.#.###.###  \n",
        "  #.......#.....#.#...#...............#...#  \n",
        "  #############.#.#.###.###################  \n",
        "               A O F   N                     \n",
        "               A A D   M                     \n",
    );

    #[test]
    fn t20a_supplied_inputs_1() {
        assert_eq!(
            UNIT_ANSWER_20A_1,
            shortest_portal_path(&generator(UNIT_INPUT_20A_1))
        );
    }
    #[test]
    fn t20a_supplied_inputs_2() {
        assert_eq!(
            UNIT_ANSWER_20A_2,
            shortest_portal_path(&generator(UNIT_INPUT_20A_2))
        );
    }

    #[test]
    fn t20b_supplied_inputs_1() {
        assert_eq!(
            UNIT_ANSWER_20B_1,
            shortest_recursive_portal_path(&generator(UNIT_INPUT_20B_1))
        );
    }

    #[test]
    fn t20a() {
        assert_eq!(
            ANSWER_20A,
            shortest_portal_path(&generator(
                &fs::read_to_string("input/2019/day20.txt").unwrap()
            ))
        );
    }
    #[test]
    fn t20b() {
        assert_eq!(
            ANSWER_20B,
            shortest_recursive_portal_path(&generator(
                &fs::read_to_string("input/2019/day20.txt").unwrap()
            ))
        );
    }
}
