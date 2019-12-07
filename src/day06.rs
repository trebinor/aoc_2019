use itertools::Itertools;
use std::collections::HashMap;

#[aoc(day06, part1, original)]
pub fn original_6a(input: &str) -> u32 {
    let objects: HashMap<_, _> = input
        .lines()
        .flat_map(|l| l.trim().rsplitn(2, ")"))
        .tuples()
        .collect();
    let mut orbit_count = 0;
    for mut parent in objects.keys() {
        while let Some(node) = objects.get(parent) {
            orbit_count += 1;
            parent = node;
        }
    }
    orbit_count
}

#[aoc(day06, part2, original)]
pub fn original_6b(input: &str) -> u32 {
    let objects: HashMap<_, _> = input
        .lines()
        .flat_map(|l| l.trim().rsplitn(2, ")"))
        .tuples()
        .collect();
    let mut orbital_transfers = 0;
    let mut you_parents: Vec<&str> = Vec::new();
    let mut san_parents: Vec<&str> = Vec::new();
    for mut parent in objects.keys() {
        while let Some(mut node) = objects.get(parent) {
            match Some(parent) {
                Some(&"YOU") => {
                    while let Some(you_parent) = objects.get(*node) {
                        you_parents.push(you_parent);
                        node = you_parent;
                    }
                }
                Some(&"SAN") => {
                    while let Some(san_parent) = objects.get(*node) {
                        san_parents.push(san_parent);
                        node = san_parent;
                    }
                }
                Some(&&_) => (),
                None => (),
            }
            parent = node;
        }
    }
    'outer: for y in you_parents {
        if let Some(pos) = san_parents.iter().position(|n| *n == y) {
            println!("pos{} y{}", pos, y);
            orbital_transfers += pos as u32;
            break 'outer;
        }
        orbital_transfers += 1;
    }
    orbital_transfers
}

#[cfg(test)]
mod tests {
    use day06::original_6a;
    use day06::original_6b;
    use std::fs;
    const ANSWER_6A: i32 = 3369286;
    const ANSWER_6B: i32 = 5051054;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_6A,
            original_6a(&fs::read_to_string("input/2019/day6.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_6B,
            original_6b(&fs::read_to_string("input/2019/day6.txt").unwrap().trim())
        );
    }
}
