use regex::Regex;

#[derive(Debug)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Position> {
    // This is the most annoying data format yet, but I have a feeling I'll need this generator again.
    let re = Regex::new(r"<x=([-0-9]+), y=([-0-9]+), z=([-0-9]+)>").unwrap();
    let mut positions: Vec<Position> = Vec::new();
    for l in input.lines() {
        let caps = re.captures(l.trim()).unwrap();
        let p = Position {
            x: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            y: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            z: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        };
        positions.push(p);
    }
    positions
}

#[aoc(day12, part1)]
pub fn total_energy(v: &[Position]) -> u32 {
    0
}

#[aoc(day12, part2)]
pub fn part_2(v: &[Position]) -> u32 {
    0
}
