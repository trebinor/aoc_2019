use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

/*
impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
*/

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}

/*
impl PartialEq for Velocity {
    fn eq(&self, other: &Velocity) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
*/

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

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct PlanetaryBody {
    position: Position,
    velocity: Velocity,
}

/*
impl PartialEq for PlanetaryBody {
    fn eq(&self, other: &PlanetaryBody) -> bool {
        self.position == other.position && self.velocity == other.velocity
    }
}
*/

impl PlanetaryBody {
    pub fn new() -> PlanetaryBody {
        PlanetaryBody {
            position: Position::new(),
            velocity: Velocity::new(),
        }
    }
    pub fn potential_energy(&self) -> i32 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    pub fn kinetic_energy(&self) -> i32 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
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

const ITERATIONS: u32 = 1000;

#[aoc(day12, part1)]
pub fn total_energy_all_planets(v: &[Position]) -> u64 {
    let mut bodies: Vec<PlanetaryBody> = Vec::new();
    for position in v {
        let mut body = PlanetaryBody::new();
        body.position = *position;
        bodies.push(body);
    }
    for i in 1..=ITERATIONS {
        //Is there a more rustic way to double-iterate over a vec?
        for a in 0..bodies.len() {
            for b in 0..bodies.len() {
                if a == b {
                    continue;
                }
                bodies[a].velocity.x += if bodies[b].position.x > bodies[a].position.x {
                    1
                } else if bodies[b].position.x < bodies[a].position.x {
                    -1
                } else {
                    0
                };
                bodies[a].velocity.y += if bodies[b].position.y > bodies[a].position.y {
                    1
                } else if bodies[b].position.y < bodies[a].position.y {
                    -1
                } else {
                    0
                };
                bodies[a].velocity.z += if bodies[b].position.z > bodies[a].position.z {
                    1
                } else if bodies[b].position.z < bodies[a].position.z {
                    -1
                } else {
                    0
                };
            }
        }
        for a in 0..bodies.len() {
            bodies[a].position.x += bodies[a].velocity.x;
            bodies[a].position.y += bodies[a].velocity.y;
            bodies[a].position.z += bodies[a].velocity.z;
        }
    }

    bodies
        .iter()
        .map(|p| p.potential_energy() as u64 * p.kinetic_energy() as u64)
        .sum()
}

#[aoc(day12, part2)]
pub fn part_2(v: &[Position]) -> u64 {
    let mut bodies: Vec<PlanetaryBody> = Vec::new();
    for position in v {
        let mut body = PlanetaryBody::new();
        body.position = *position;
        bodies.push(body);
    }
    let mut n_body_simulation: HashSet<Vec<PlanetaryBody>> = HashSet::new();
    let mut i: u64 = 0;
    loop {
        //why do these not work here, but they do work in part 1
        for a in 0..bodies.len() {
            for b in 0..bodies.len() {
                if a == b {
                    continue;
                }
                bodies[a].velocity.x += if bodies[b].position.x > bodies[a].position.x {
                    1
                } else if bodies[b].position.x < bodies[a].position.x {
                    -1
                } else {
                    0
                };
                bodies[a].velocity.y += if bodies[b].position.y > bodies[a].position.y {
                    1
                } else if bodies[b].position.y < bodies[a].position.y {
                    -1
                } else {
                    0
                };
                bodies[a].velocity.z += if bodies[b].position.z > bodies[a].position.z {
                    1
                } else if bodies[b].position.z < bodies[a].position.z {
                    -1
                } else {
                    0
                };
            }
        }

        for a in 0..bodies.len() {
            bodies[a].position.x += bodies[a].velocity.x;
            bodies[a].position.y += bodies[a].velocity.y;
            bodies[a].position.z += bodies[a].velocity.z;
        }
        i += 1;
        if !n_body_simulation.insert(bodies.clone()) {
            break;
        }
    }
    i
}
