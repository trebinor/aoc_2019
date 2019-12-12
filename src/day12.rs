use regex::Regex;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone)]
pub struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    pub fn new() -> Position {
        Position { x: 0, y: 0, z: 0 }
    }
}

impl Velocity {
    pub fn new() -> Velocity {
        Velocity { x: 0, y: 0, z: 0 }
    }
}

#[derive(Clone)]
pub struct Planet {
    old_position: Position,
    position: Position,
    old_velocity: Velocity,
    velocity: Velocity,
}

impl Planet {
    pub fn new() -> Planet {
        Planet {
            old_position: Position::new(),
            position: Position::new(),
            old_velocity: Velocity::new(),
            velocity: Velocity::new(),
        }
    }
    pub fn potential_energy(&self) -> i32 {
        self.position.x + self.position.y + self.position.z
    }

    pub fn kinetic_energy(&self) -> i32 {
        self.velocity.x + self.velocity.y + self.velocity.z
    }
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
pub fn total_energy_all_planets(v: &[Position]) -> i32 {
    let mut planets: Vec<Planet> = Vec::new();
    for position in v {
        let mut planet = Planet::new();
        planet.position = *position;
        planets.push(planet);
    }
    //TODO: apply gravity and velocity in loop
    planets
        .iter()
        .map(|p| p.potential_energy() * p.kinetic_energy())
        .sum()
}

#[aoc(day12, part2)]
pub fn part_2(v: &[Position]) -> u32 {
    0
}
