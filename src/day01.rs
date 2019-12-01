use std::cmp;

#[aoc(day01, part1)]
pub fn solve_1a(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.trim().parse::<f32>().unwrap())
        .map(|f| ((f / 3.0) as i32) - 2)
        .sum()
}

#[aoc(day01, part2)]
pub fn solve_1b(input: &str) -> i32 {
    let modules = input.lines().map(|l| l.trim().parse::<i32>().unwrap());
    let mut fuel: i32 = 0;
    for fuels in modules {
        let mut module_fuel = fuel_mass(fuels);
        while module_fuel != 0 {
            fuel += module_fuel;
            module_fuel = fuel_mass(module_fuel);
        }
    }
    fuel
}

fn fuel_mass(module: i32) -> i32 {
    cmp::max(0, ((module / 3.0 as i32) - 2))
}
