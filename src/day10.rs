//use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

trait PointAnalysis {
    fn find_best_point(&self) -> Point;
}

impl PointAnalysis for AsteroidField {
    fn find_best_point(&self) -> Point {
        Point{x: 0, y: 0}
    }
}

//https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)
type AsteroidField = HashMap<Point, HashSet<Point>>;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> AsteroidField {
    let v: Vec<&str> =
    input
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<&str>>();
        let mut asteroid_field: AsteroidField = HashMap::new();
        asteroid_field
}

#[aoc(day10, part1)]
pub fn visible_asteroids(asteroid_field: &AsteroidField) -> u32 {
        if let Some(point) = asteroid_field.get(&asteroid_field.find_best_point()) {
            // how many asteroids for this point?
            0
        } else {
            // None, so 0 asteroids
            0
        }
}

#[cfg(test)]
mod tests {
    use day10::visible_asteroids;
    use day10::generator;
    use day10::Point;
    use day10::PointAnalysis;
    use std::fs;
    const UNIT_INPUT_10A_1: &str =
r"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
const UNIT_INPUT_10A_2: &str =
r"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
const UNIT_INPUT_10A_3: &str =
r".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
const UNIT_INPUT_10A_4: &str =
r".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
    const ANSWER_10A: u32 = 0;
    const ANSWER_10B: u32 = 0;
    const UNIT_ANSWER_10A_1: Point = Point { x: 5, y: 8};
    const UNIT_ANSWER_10A_2: Point = Point { x: 1, y: 2};
    const UNIT_ANSWER_10A_3: Point = Point { x: 6, y: 3};
    const UNIT_ANSWER_10A_4: Point = Point { x: 11, y: 13};

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_10A,
            visible_asteroids(&generator(&fs::read_to_string("input/2019/day10.txt").unwrap().trim())));
        assert_eq!(
            ANSWER_10B,
            visible_asteroids(&generator(&fs::read_to_string("input/2019/day10.txt").unwrap().trim())));
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
