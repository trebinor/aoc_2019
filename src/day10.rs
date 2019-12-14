//use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

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

trait PointAnalysis {
    fn find_best_point(&mut self, &[Point]) -> Point;
}

impl PointAnalysis for AsteroidSightMap {
    fn find_best_point(&mut self, asteroid_field: &[Point]) -> Point {
        // Create keys in the sight map for all asteroids in the field
        for a in asteroid_field {
            self.entry(*a).or_default();
        }

        // Apply algorithm to each asteroid and update sight map
        for source in asteroid_field {
            'target: for target in asteroid_field {
                println!("{:?} to {:?}", source, target);
                // draw a ray
                let mut dx = target.x as f32 - source.x as f32;
                let mut dy = target.y as f32 - source.y as f32;
                let step = if dx.abs() >= dy.abs() {
                    dx.abs()
                } else {
                    dy.abs()
                };
                dx /= step;
                dy /= step;
                let mut x = source.x as f32;
                let mut y = source.y as f32;
                let mut i = 1;
                while i as f32 <= step {
                    //println!("{} < {}", i as f32, step);
                    // put pixel at x,y.  if it intersects an asteroid other than target, continue
                    if asteroid_field
                        .iter()
                        .find(|&&a| {
                            a == Point {
                                x: x as usize,
                                y: y as usize,
                            }
                        })
                        .is_some()
                    {
                        if x as usize == target.x && y as usize == target.y {
                            self.entry(*source).and_modify(|e| {
                                e.insert(*target);
                            });
                        }
                        continue 'target;
                    } else {
                        x += dx;
                        y += dy;
                        i += 1;
                    }
                }
            }
        }

        // return point with largest count in hashset
        *self
            .keys()
            .max_by(|x, y| self.get(x).unwrap().len().cmp(&self.get(y).unwrap().len()))
            .unwrap()
    }
}

//https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)
type AsteroidSightMap = HashMap<Point, HashSet<Point>>;
type AsteroidField = Vec<Point>;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> AsteroidField {
    let mut v = input.lines().map(|l| l.trim()).collect::<Vec<&str>>();
    let mut asteroid_field: AsteroidField = AsteroidField::new();
    for (i, l) in v.iter().enumerate() {
        for (j, x) in l.chars().enumerate() {
            match x {
                '.' => (),
                '#' => asteroid_field.push(Point { x: j, y: i }),
                _ => unreachable!(),
            }
        }
    }
    asteroid_field
}

#[aoc(day10, part1)]
pub fn visible_asteroids(asteroid_field: &[Point]) -> u32 {
    let mut asteroid_sight_map = AsteroidSightMap::new();
    let mut best_point = &asteroid_sight_map.find_best_point(asteroid_field);
    if let Some(asteroids_visible_by_best_point) = &asteroid_sight_map.get(&best_point) {
        // how many asteroids for this point?
        asteroids_visible_by_best_point.len() as u32
    } else {
        // None, so 0 asteroids
        0
    }
}

#[cfg(test)]
mod tests {
    use day10::generator;
    use day10::visible_asteroids;
    use day10::AsteroidSightMap;
    use day10::Point;
    use day10::PointAnalysis;
    use std::fs;
    const UNIT_INPUT_10A_1: &str = r"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
    const UNIT_INPUT_10A_2: &str = r"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
    const UNIT_INPUT_10A_3: &str = r".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
    const UNIT_INPUT_10A_4: &str = r".#..##.###...#######
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
    const UNIT_ANSWER_10A_1: Point = Point { x: 5, y: 8 };
    const UNIT_ANSWER_10A_2: Point = Point { x: 1, y: 2 };
    const UNIT_ANSWER_10A_3: Point = Point { x: 6, y: 3 };
    const UNIT_ANSWER_10A_4: Point = Point { x: 11, y: 13 };

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_10A,
            visible_asteroids(&generator(
                &fs::read_to_string("input/2019/day10.txt").unwrap().trim()
            ))
        );
        assert_eq!(
            ANSWER_10B,
            visible_asteroids(&generator(
                &fs::read_to_string("input/2019/day10.txt").unwrap().trim()
            ))
        );
    }

    #[test]
    fn supplied_inputs() {
        assert_eq!(
            UNIT_ANSWER_10A_1,
            AsteroidSightMap::new().find_best_point(&generator(UNIT_INPUT_10A_1))
        );
        assert_eq!(
            UNIT_ANSWER_10A_2,
            AsteroidSightMap::new().find_best_point(&generator(UNIT_INPUT_10A_2))
        );
        assert_eq!(
            UNIT_ANSWER_10A_3,
            AsteroidSightMap::new().find_best_point(&generator(UNIT_INPUT_10A_3))
        );
        assert_eq!(
            UNIT_ANSWER_10A_4,
            AsteroidSightMap::new().find_best_point(&generator(UNIT_INPUT_10A_4))
        );
        assert_eq!(33, visible_asteroids(&generator(UNIT_INPUT_10A_1)));
        assert_eq!(35, visible_asteroids(&generator(UNIT_INPUT_10A_2)));
        assert_eq!(41, visible_asteroids(&generator(UNIT_INPUT_10A_3)));
        assert_eq!(210, visible_asteroids(&generator(UNIT_INPUT_10A_4)));
    }
}
