#[aoc(day20, part1)]
pub fn shortest_path_with_portal(input: &str) -> usize {
    let _v = input.lines().map(|l| l.trim()).collect::<Vec<&str>>();
    0
}

#[aoc(day20, part2)]
pub fn solution_20b(input: &str) -> usize {
    let _v = input.lines().map(|l| l.trim()).collect::<Vec<&str>>();
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
