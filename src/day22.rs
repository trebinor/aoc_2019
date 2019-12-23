fn deal_with_increment(cards: &mut [usize], increment: usize) {
    println!("deal with increment {}", increment);
    let mut new_cards = [0; 10007];
    let mut i = 0;
    let mut j = 0;
    while i < cards.len() {
        assert_eq!(new_cards[j], 0);
        new_cards[j] = cards[i];
        i += 1;
        j = (j + increment) % cards.len()
    }
    cards.copy_from_slice(&new_cards);
    for i in 0..cards.len() {
        assert_eq!(new_cards[i], cards[i]);
    }
}

fn deal_into_new_stack(cards: &mut [usize]) {
    println!("deal into new stack");
    cards.reverse();
}

fn cut(cards: &mut [usize], offset: i32) {
    println!("cut {}", offset);
    if offset > 0 {
        cards.rotate_left(offset as usize);
    } else if offset < 0 {
        cards.rotate_right(offset.abs() as usize);
    }
}

#[aoc(day22, part1)]
pub fn solution_22a(input: &str) -> usize {
    let s: Vec<&str> = input.lines().map(|l| l.trim()).collect();
    let mut cards = [0; 10007];
    for i in 0..cards.len() {
        cards[i] = i;
        println!("Card[{}]", cards[i]);
    }
    for l in s {
        if l.starts_with("deal with increment") {
            deal_with_increment(&mut cards, l.split_at(20).1.parse::<usize>().unwrap());
        // index of increment in command
        } else if l.starts_with("cut") {
            cut(&mut cards, l.split_at(4).1.parse::<i32>().unwrap()); // index of cut number in command
        } else if l.starts_with("deal into new stack") {
            deal_into_new_stack(&mut cards);
        } else {
            panic!("Unhandled card command! {}", l);
        }
    }
    cards.to_vec().iter().position(|e| *e == 2019).unwrap()
}

#[aoc(day22, part2)]
pub fn solution_22b(input: &str) -> u64 {
    let s: Vec<&str> = input.lines().map(|l| l.trim()).collect();
    let mut cards = [0; 119_315_717_514_047usize];
    for i in 0..cards.len() {
        cards[i] = i;
    }
    for i in 0..101_741_582_076_661usize {
        for l in &s {
            if l.starts_with("deal with increment") {
                deal_with_increment(&mut cards, l.split_at(20).1.parse::<usize>().unwrap());
            // index of increment in command
            } else if l.starts_with("cut") {
                cut(&mut cards, l.split_at(4).1.parse::<i32>().unwrap()); // index of cut number in command
            } else if l.starts_with("deal into new stack") {
                deal_into_new_stack(&mut cards);
            } else {
                panic!("Unhandled card command! {}", l);
            }
        }
    }
    cards[2020] as u64
}

#[cfg(test)]
mod tests {
    use day22::solution_22a;
    use day22::solution_22b;
    use std::fs;
    const ANSWER_22A: u32 = 0;
    const ANSWER_22B: u32 = 0;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_22A,
            solution_22a(&fs::read_to_string("input/2019/day22.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_22B,
            solution_22b(&fs::read_to_string("input/2019/day22.txt").unwrap().trim())
        );
    }
}
